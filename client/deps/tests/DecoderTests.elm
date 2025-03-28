module DecoderTests exposing (..)

import Expect
import Json.Decode exposing (decodeString)
import Serde exposing (deploymentDecoder, deploymentResultsDecoder, deploymentErrorDecoder)
import Test exposing (..)


decodeDeploymentTest : Test
decodeDeploymentTest =
    test "decodes deployment correctly" <|
        \_ ->
            """
{
  "service_name": "some-service",
  "keys": "qa",
  "status": "PRIMARY",
  "running_count": 2,
  "desired_count": 2,
  "pending_count": 0,
  "failed_count": 0
}
            """
                |> decodeString deploymentDecoder
                |> Expect.ok


decodeDeploymentErrorTest : Test
decodeDeploymentErrorTest =
    test "decodes deployment error correctly" <|
        \_ ->
            """
{
  "service_name": "some-service",
  "keys": "qa",
  "error": "getting credentials for profile failed\\nprofile \\"blah\\" doesn't exist"
}
            """
                |> decodeString deploymentErrorDecoder
                |> Expect.ok


decodeDeploymentResultsTest : Test
decodeDeploymentResultsTest =
    test "decodes deployment results correctly" <|
        \_ ->
            """
{
  "deployments": [
    {
      "service_name": "some-service",
      "keys": "qa",
      "status": "PRIMARY",
      "running_count": 2,
      "desired_count": 2,
      "pending_count": 0,
      "failed_count": 0
    }
  ],
  "errors": [
    {
      "service_name": "some-service",
      "keys": "qa",
      "error": "getting credentials for profile failed\\nprofile \\"blah\\" doesn't exist"
    }
  ]
}
            """
                |> decodeString deploymentResultsDecoder
                |> Expect.ok
