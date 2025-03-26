module Serde exposing (..)

import Json.Decode exposing (Decoder, field, int, list, map2, string, succeed)
import Json.Decode.Pipeline exposing (required)
import Types exposing (Deployment, DeploymentError, DeploymentResults)


deploymentDecoder : Decoder Deployment
deploymentDecoder =
    succeed Deployment
        |> required "service_name" string
        |> required "keys" string
        |> required "status" string
        |> required "running_count" int
        |> required "desired_count" int
        |> required "pending_count" int
        |> required "failed_count" int


errorDecoder : Decoder DeploymentError
errorDecoder =
    succeed DeploymentError
        |> required "service_name" string
        |> required "keys" string
        |> required "error" string


deploymentResultsDecoder : Decoder DeploymentResults
deploymentResultsDecoder =
    map2 DeploymentResults
        (field "deployments" (list deploymentDecoder))
        (field "errors" (list errorDecoder))
