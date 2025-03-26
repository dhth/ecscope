module Types exposing (..)

import Http
import Json.Decode exposing (Decoder)
import Json.Decode.Pipeline


type alias Deployment =
    { service_name : String
    , keys : String
    , status : String
    , running_count : Int
    , desired_count : Int
    , pending_count : Int
    , failed_count : Int
    }


type alias DeploymentError =
    { service_name : String
    , keys : String
    , error : String
    }


type alias DeploymentResults =
    { deployments : List Deployment
    , errors : List DeploymentError
    }


type Status
    = Loading
    | Loaded DeploymentResults
    | Errored Http.Error


type alias RefreshScheduleNumSeconds =
    Maybe Int


type alias Model =
    { status : Status
    , reload_seconds : Int
    , auto_refresh : Bool
    , fetching : Bool
    , debug : Bool
    }


type Msg
    = FetchResults
    | AutoRefreshToggled Bool
    | AutoRefreshScheduleChanged RefreshScheduleNumSeconds
    | ResultsFetched (Result Http.Error DeploymentResults)
    | Tick


deploymentDecoder : Decoder Deployment
deploymentDecoder =
    Json.Decode.succeed Deployment
        |> Json.Decode.Pipeline.required "service_name" Json.Decode.string
        |> Json.Decode.Pipeline.required "keys" Json.Decode.string
        |> Json.Decode.Pipeline.required "status" Json.Decode.string
        |> Json.Decode.Pipeline.required "running_count" Json.Decode.int
        |> Json.Decode.Pipeline.required "desired_count" Json.Decode.int
        |> Json.Decode.Pipeline.required "pending_count" Json.Decode.int
        |> Json.Decode.Pipeline.required "failed_count" Json.Decode.int


errorDecoder : Decoder DeploymentError
errorDecoder =
    Json.Decode.succeed DeploymentError
        |> Json.Decode.Pipeline.required "service_name" Json.Decode.string
        |> Json.Decode.Pipeline.required "keys" Json.Decode.string
        |> Json.Decode.Pipeline.required "error" Json.Decode.string


deploymentResultsDecoder : Decoder DeploymentResults
deploymentResultsDecoder =
    Json.Decode.map2 DeploymentResults
        (Json.Decode.field "deployments" (Json.Decode.list deploymentDecoder))
        (Json.Decode.field "errors" (Json.Decode.list errorDecoder))
