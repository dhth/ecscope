module View exposing (view)

import Html exposing (button, div, h1, input, label, pre, span, text)
import Html.Attributes exposing (checked, class, id)
import Html.Events
import Json.Encode
import Types exposing (Model, Msg(..), Status(..))
import Utils exposing (..)


view : Model -> Html.Html Msg
view model =
    div [ class "container" ] <|
        [ div [ class "bg-[#282828] text-[#ebdbb2]" ] <|
            [ div [] <|
                [ if model.debug then
                    div [ class "debug bg-gray-800 text-white p-4 overflow-auto mb-5" ]
                        [ pre [ class "whitespace-pre-wrap" ]
                            [ text (Json.Encode.encode 4 (modelToJson model)) ]
                        ]

                  else
                    Html.text ""
                , h1 [ class "text-3xl font-bold mb-6" ]
                    [ text (getHeading model.fetching)
                    ]
                , div [ class "mb-4 flex items-center space-x-4" ]
                    [ div [ class "flex items-center space-x-2 py-2 rounded bg-[#282828]" ]
                        [ label [ class "flex items-center space-x-2", Html.Attributes.for "refresh-toggle" ]
                            [ input
                                [ id "refresh-toggle"
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
                            [ id "refresh-interval"
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
                            , class "font-semibold px-4 py-1 bg-[#d3869b] text-[#282828] rounded disabled:bg-[#928374]"
                            , Html.Attributes.disabled (model.auto_refresh || model.fetching)
                            , Html.Events.onClick FetchResults
                            ]
                            [ text "Refresh" ]
                        ]
                    ]
                , case model.status of
                    Loaded results ->
                        getResultsDiv results.deployments results.errors

                    Errored error ->
                        getHttpErrorDiv error

                    Loading ->
                        getLoadingMessage
                ]
            ]
        ]
