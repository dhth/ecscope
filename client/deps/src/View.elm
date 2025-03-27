module View exposing (view)

import Html exposing (a, button, div, h1, h2, hr, input, label, p, pre, span, table, tbody, td, text, th, thead, tr)
import Html.Attributes exposing (checked, class, href, id, target)
import Html.Events
import Http
import Json.Encode
import Model exposing (Model)
import Types exposing (Deployment, DeploymentError, Msg(..), Status(..))
import Utils exposing (..)


view : Model -> Html.Html Msg
view model =
    div [ class "container" ] <|
        [ div [ class "bg-[#282828] text-[#ebdbb2]" ] <|
            [ div [] <|
                [ modelDebugDiv model
                , heading model.fetching
                , fetchControlsDiv model
                , mainDiv model
                ]
            ]
        ]


modelDebugDiv : Model -> Html.Html msg
modelDebugDiv model =
    if model.debug then
        div [ class "debug bg-gray-800 text-white p-4 overflow-auto mb-5" ]
            [ pre [ class "whitespace-pre-wrap" ]
                [ text (Json.Encode.encode 4 (modelToJson model)) ]
            ]

    else
        Html.text ""


heading : Bool -> Html.Html msg
heading fetching =
    let
        headingText =
            if fetching then
                "ecscope ..."

            else
                "ecscope"
    in
    h1 [ class "text-3xl font-bold mb-6" ]
        [ a [ href "https://github.com/dhth/ecscope", target "_blank" ] [ text headingText ]
        ]


fetchControlsDiv : Model -> Html.Html Msg
fetchControlsDiv model =
    div [ class "mb-4 flex items-center space-x-4" ]
        [ div [ class "flex items-center space-x-2 py-2 rounded bg-[#282828]" ]
            [ label [ class "flex items-center space-x-2", Html.Attributes.for "auto-refresh-toggle" ]
                [ input
                    [ id "auto-refresh-toggle"
                    , Html.Attributes.type_ "checkbox"
                    , class "w-4 h-4 text-[#fabd2f] bg-[#282828] rounded focus:ring-[#fabd2f]"
                    , Html.Events.onCheck AutoRefreshToggled
                    , checked
                        (model.auto_refresh == True)
                    ]
                    []
                , span [] [ text "Auto refresh every" ]
                ]
            , input
                [ id "auto-refresh-interval"
                , Html.Attributes.type_ "number"
                , class "w-12 h-8 text-center text-[#ebdbb2] bg-[#3c3836] rounded focus:ring-[#fabd2f]"
                , Html.Attributes.min "5"
                , Html.Attributes.max "300"
                , Html.Attributes.value (model.reload_seconds |> String.fromInt)
                , Html.Attributes.disabled model.auto_refresh
                , Html.Events.onInput (\value -> AutoRefreshScheduleChanged (String.toInt value))
                ]
                []
            , span [] [ text "seconds" ]
            ]
        , div []
            [ button
                [ id "manual-refresh"
                , class "font-semibold px-4 py-1 bg-[#d3869b] text-[#282828] disabled:bg-[#928374]"
                , Html.Attributes.disabled (model.auto_refresh || model.fetching)
                , Html.Events.onClick FetchResults
                ]
                [ text "Refresh" ]
            ]
        ]


mainDiv : Model -> Html.Html msg
mainDiv model =
    case model.status of
        Loaded results ->
            resultsDiv results.deployments results.errors

        Errored error ->
            httpErrorDiv error

        Loading ->
            loadingMessage


httpErrorDiv : Http.Error -> Html.Html msg
httpErrorDiv error =
    div [ class "error-message" ]
        [ p [] [ text ("Error: " ++ httpErrorToString error) ]
        ]


loadingMessage : Html.Html msg
loadingMessage =
    div [] <|
        [ h2 [ class "text-xl font-bold mb-6 text-[#fabd2f]", id "loading-message" ] [ text "Loading..." ]
        ]


resultsDiv : List Deployment -> List DeploymentError -> Html.Html msg
resultsDiv deployments errors =
    let
        depsPresent =
            not (List.isEmpty deployments)

        errorsPresent =
            not (List.isEmpty errors)
    in
    if not depsPresent && not errorsPresent then
        noDeploymentResultsDiv

    else
        div [ id "deployment-results" ]
            (List.concat
                [ if depsPresent then
                    [ deploymentDetailsDiv deployments
                    ]

                  else
                    []
                , if depsPresent && errorsPresent then
                    [ deploymentResultsDivider ]

                  else
                    []
                , if errorsPresent then
                    [ deploymentErrorsDiv errors
                    ]

                  else
                    []
                ]
            )


noDeploymentResultsDiv : Html.Html msg
noDeploymentResultsDiv =
    div [ id "no-deployment-results" ]
        [ h2 [ class "text-xl font-bold mb-6 text-[#83a598]" ] [ text "No Deployments found" ]
        ]


deploymentDetailsDiv : List Deployment -> Html.Html msg
deploymentDetailsDiv deployments =
    div [ id "deployment-details" ]
        [ h2 [ class "text-xl font-bold mb-6 text-[#83a598]" ] [ text "Deployments" ]
        , div [ class "legend mb-4 p-4" ]
            [ p [ class "p-1 m-1 font-semibold" ] [ text "Legend: " ]
            , p [ class "legend-pending py-1 px-2 m-1 font-semibold" ] [ text "pending" ]
            , p [ class "legend-active py-1 px-2 m-1 font-semibold" ] [ text "being replaced" ]
            , p [ class "legend-draining py-1 px-2 m-1 font-semibold" ] [ text "draining" ]
            , p [ class "legend-failing py-1 px-2 m-1 font-semibold" ] [ text "failing" ]
            ]
        , deploymentsTable deployments
        ]


deploymentResultsDivider : Html.Html msg
deploymentResultsDivider =
    hr [ class "h-px my-10 bg-[#928374] border-0 dark:bg-[#928374]", id "results-divider" ] []


deploymentErrorsDiv : List DeploymentError -> Html.Html msg
deploymentErrorsDiv errors =
    div [ id "deployment-errors" ]
        [ h2 [ class "text-xl font-bold mb-6 text-[#fb4934]" ] [ text "Errors" ]
        , errorsTable errors
        ]


deploymentsTable : List Deployment -> Html.Html msg
deploymentsTable deployments =
    table [ class "table-auto w-full px-4 py-2", id "deployment-details-table" ]
        [ thead []
            [ tr []
                [ th [] [ text "Service" ]
                , th [] [ text "Keys" ]
                , th [] [ text "Status" ]
                , th [] [ text "Running" ]
                , th [] [ text "Desired" ]
                , th [] [ text "Pending" ]
                , th [] [ text "Failed" ]
                ]
            ]
        , tbody []
            (List.map deploymentTableRow deployments)
        ]


errorsTable : List DeploymentError -> Html.Html msg
errorsTable errors =
    table [ class "table-auto w-full px-4 py-2", id "deployment-errors-table" ]
        [ thead []
            [ tr []
                [ th [] [ text "Service" ]
                , th [] [ text "Keys" ]
                , th [] [ text "Error" ]
                ]
            ]
        , tbody []
            (List.map renderErrorRow errors)
        ]


deploymentTableRow : Deployment -> Html.Html msg
deploymentTableRow deployment =
    let
        rowClass =
            tableRowClass deployment
    in
    tr [ class rowClass ]
        [ serviceNameTableData deployment
        , td [] [ text deployment.keys ]
        , td [] [ text deployment.status ]
        , td [] [ text (String.fromInt deployment.running_count) ]
        , td [] [ text (String.fromInt deployment.desired_count) ]
        , td [] [ text (String.fromInt deployment.pending_count) ]
        , td [] [ text (String.fromInt deployment.failed_count) ]
        ]


tableRowClass : Deployment -> String
tableRowClass deployment =
    if deployment.failed_count > 0 then
        "row-failing"

    else if deployment.status == "ACTIVE" then
        "row-active"

    else if not (deployment.running_count == deployment.desired_count) then
        "row-pending"

    else if deployment.status == "DRAINING" then
        "row-draining"

    else
        ""


serviceNameTableData : Deployment -> Html.Html msg
serviceNameTableData deployment =
    if
        deployment.failed_count
            > 0
            || deployment.status
            == "ACTIVE"
            || not
                (deployment.running_count
                    == deployment.desired_count
                )
            || deployment.status
            == "DRAINING"
    then
        td [ class "font-semibold" ] [ text deployment.service_name ]

    else
        td [ class ("font-semibold text-[" ++ getColorForString deployment.service_name colorPool ++ "]") ] [ text deployment.service_name ]


renderErrorRow : DeploymentError -> Html.Html msg
renderErrorRow error =
    tr []
        [ errorTableServiceNameTableData error
        , td [] [ text error.keys ]
        , td [] (errorWithNewlines error.error)
        ]


errorTableServiceNameTableData : DeploymentError -> Html.Html msg
errorTableServiceNameTableData error =
    td [ class ("font-semibold text-[" ++ getColorForString error.service_name colorPool ++ "]") ] [ text error.service_name ]


errorWithNewlines : String -> List (Html.Html msg)
errorWithNewlines errorText =
    String.split "\n" errorText
        |> List.map (\line -> div [] [ text line ])
