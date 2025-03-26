module Utils exposing (..)

import Html exposing (div, h2, hr, input, p, table, tbody, td, text, th, thead, tr)
import Html.Attributes exposing (class, id)
import Http
import Json.Encode exposing (Value, bool, object)
import Types exposing (Deployment, DeploymentError, Model)


modelToJson : Model -> Value
modelToJson model =
    object
        [ ( "reload_seconds", Json.Encode.string (Debug.toString model.reload_seconds) )
        , ( "auto_refresh", bool model.auto_refresh )
        , ( "fetching", bool model.fetching )
        ]


getHeading : Bool -> String
getHeading fetching =
    if fetching then
        "ecscope ..."

    else
        "ecscope"


getRowClass : Deployment -> String
getRowClass deployment =
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


getColorForString : String -> List String -> String
getColorForString input colors =
    let
        hash =
            String.foldl (\char acc -> acc + Char.toCode char) 0 input

        index =
            Basics.remainderBy (List.length colors) hash
    in
    case List.drop index colorPool |> List.head of
        Just color ->
            color

        Nothing ->
            "#000000"


getServiceNameTableData : Deployment -> Html.Html msg
getServiceNameTableData deployment =
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


renderDeploymentRow : Deployment -> Html.Html msg
renderDeploymentRow deployment =
    let
        rowClass =
            getRowClass deployment
    in
    tr [ class rowClass ]
        [ getServiceNameTableData deployment
        , td [] [ text deployment.keys ]
        , td [] [ text deployment.status ]
        , td [] [ text (String.fromInt deployment.running_count) ]
        , td [] [ text (String.fromInt deployment.desired_count) ]
        , td [] [ text (String.fromInt deployment.pending_count) ]
        , td [] [ text (String.fromInt deployment.failed_count) ]
        ]


getDeploymentsTable : List Deployment -> Html.Html msg
getDeploymentsTable deployments =
    table [ class "table-auto w-full px-4 py-2", id "deployments-table" ]
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
            (List.map renderDeploymentRow deployments)
        ]


getErrorServiceNameTableData : DeploymentError -> Html.Html msg
getErrorServiceNameTableData error =
    td [ class ("font-semibold text-[" ++ getColorForString error.service_name colorPool ++ "]") ] [ text error.service_name ]


renderErrorWithNewlines : String -> List (Html.Html msg)
renderErrorWithNewlines errorText =
    String.split "\n" errorText
        |> List.map (\line -> div [] [ text line ])


renderErrorRow : DeploymentError -> Html.Html msg
renderErrorRow error =
    tr []
        [ getErrorServiceNameTableData error
        , td [] [ text error.keys ]
        , td [] (renderErrorWithNewlines error.error)
        ]


getErrorsTable : List DeploymentError -> Html.Html msg
getErrorsTable errors =
    table [ class "table-auto w-full px-4 py-2", id "errors-table" ]
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


getResultsDiv : List Deployment -> List DeploymentError -> Html.Html msg
getResultsDiv deployments errors =
    div []
        (List.concat
            [ if not (List.isEmpty deployments) then
                [ div []
                    [ h2 [ class "text-xl font-bold mb-6 text-[#83a598]" ] [ text "Deployments" ]
                    , getDeploymentsTable deployments
                    ]
                , if not (List.isEmpty errors) then
                    hr [ class "h-px my-10 bg-[#928374] border-0 dark:bg-[#928374]" ] []

                  else
                    Html.text ""
                ]

              else
                []
            , if not (List.isEmpty errors) then
                [ div []
                    [ h2 [ class "text-xl font-bold mb-6 text-[#fb4934]" ] [ text "Errors" ]
                    , getErrorsTable errors
                    ]
                ]

              else
                []
            ]
        )


httpErrorToString : Http.Error -> String
httpErrorToString error =
    case error of
        Http.BadUrl url ->
            "Bad URL: " ++ url

        Http.Timeout ->
            "Request timed out"

        Http.NetworkError ->
            "Network error occurred"

        Http.BadStatus statusCode ->
            "Bad status code: " ++ String.fromInt statusCode

        Http.BadBody body ->
            "Bad body: " ++ body


getHttpErrorDiv : Http.Error -> Html.Html msg
getHttpErrorDiv error =
    div [ class "error-message" ]
        [ p [] [ text ("Error: " ++ httpErrorToString error) ]
        ]


getLoadingMessage : Html.Html msg
getLoadingMessage =
    div [ id "deployments" ] <|
        [ p [] [ text "loading..." ]
        ]


colorPool : List String
colorPool =
    [ "#fe77a4"
    , "#d3869a"
    , "#ff4c8b"
    , "#ffb0c2"
    , "#df748b"
    , "#ff6682"
    , "#f19597"
    , "#d89e9d"
    , "#fc5260"
    , "#e96462"
    , "#ffb5a2"
    , "#febcac"
    , "#f0947b"
    , "#ff6334"
    , "#af9084"
    , "#ff5405"
    , "#e98658"
    , "#be876e"
    , "#ff803b"
    , "#fd780b"
    , "#ff9743"
    , "#e2ac85"
    , "#d67717"
    , "#d4925c"
    , "#ffb472"
    , "#fe9103"
    , "#de9644"
    , "#dc8b00"
    , "#ffb13c"
    , "#c9b094"
    , "#faca7d"
    , "#c7921f"
    , "#c6a267"
    , "#d3cdc5"
    , "#fabd2f"
    , "#dcad50"
    , "#daa402"
    , "#ffc20c"
    , "#fbcf56"
    , "#b29807"
    , "#e7c727"
    , "#c7b648"
    , "#9c9360"
    , "#cec48b"
    , "#bbb206"
    , "#ddd601"
    , "#d1cc74"
    , "#b8bb26"
    , "#acaa5e"
    , "#b4c800"
    , "#a6b92b"
    , "#a8b64c"
    , "#aab08a"
    , "#849843"
    , "#a8d906"
    , "#a8a9a3"
    , "#88b500"
    , "#add562"
    , "#a0d845"
    , "#8de107"
    , "#829b60"
    , "#7db839"
    , "#94bc63"
    , "#71c200"
    , "#b5d092"
    , "#6e9f3a"
    , "#51a100"
    , "#b5e48c"
    , "#8ce852"
    , "#59d412"
    , "#89d967"
    , "#59c435"
    , "#4ba539"
    , "#00b700"
    , "#00db04"
    , "#9ae089"
    , "#6fbd63"
    , "#83b87a"
    , "#5ddb63"
    , "#04eb4d"
    , "#7a9879"
    , "#00ce48"
    , "#05b64c"
    , "#9cdea5"
    , "#64d97f"
    , "#8fbc96"
    , "#4daa67"
    , "#00d977"
    , "#12b667"
    , "#6ed999"
    , "#63bd8f"
    , "#00d990"
    , "#a7e0c2"
    , "#0abe88"
    , "#90b4a6"
    , "#83a598"
    , "#5cab95"
    , "#b9d9cf"
    , "#03d7b3"
    , "#00b499"
    , "#6fd0bd"
    , "#1edacd"
    , "#19b7b2"
    , "#89cbce"
    , "#4dcfdb"
    , "#62a6ae"
    , "#90e1ef"
    , "#01aac0"
    , "#48cae4"
    , "#00ddff"
    , "#6ac1db"
    , "#00c3f9"
    , "#99bbcd"
    , "#149ccd"
    , "#6da2c6"
    , "#7bcaff"
    , "#07b1fa"
    , "#b4d4fb"
    , "#629fdb"
    , "#5aaaff"
    , "#0798ff"
    , "#4896ef"
    , "#bbd1ff"
    , "#9fb9f0"
    , "#949aab"
    , "#7b8ad5"
    , "#8498fb"
    , "#aaaffc"
    , "#8187dc"
    , "#ada7ff"
    , "#aba3ca"
    , "#d2c8f2"
    , "#a681fb"
    , "#b798f0"
    , "#c3a4e1"
    , "#ce8cf7"
    , "#c97df9"
    , "#e6a5f4"
    , "#e47cfb"
    , "#ffc6ff"
    , "#f344ff"
    , "#a882a7"
    , "#c57fbf"
    , "#ff4ded"
    , "#f081de"
    , "#fc69e6"
    , "#dfa5ca"
    , "#f646c1"
    , "#ceb4c3"
    , "#f27abe"
    , "#ae8c99"
    , "#ee91b6"
    ]
