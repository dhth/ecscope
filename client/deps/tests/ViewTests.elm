module ViewTests exposing (..)

import Expect
import Html.Attributes
import Model exposing (initialModel)
import Test exposing (..)
import Test.Html.Query as Query
import Test.Html.Selector exposing (attribute, id, tag, text)
import Types exposing (DeploymentResults, Status(..))
import View exposing (view)


initialViewTest : Test
initialViewTest =
    Test.describe "Initial view"
        [ Test.test "shows loading indicator" <|
            \_ ->
                initialModel
                    |> view
                    |> Query.fromHtml
                    |> Query.find [ id "loading-message" ]
                    |> Query.has [ text "Loading..." ]
        ]


fetchControlsTest : Test
fetchControlsTest =
    Test.describe "Fetch controls"
        [ Test.test "don't allow manual refresh when auto refresh is ON" <|
            \_ ->
                { initialModel | auto_refresh = True }
                    |> view
                    |> Query.fromHtml
                    |> Query.findAll [ id "manual-refresh", attribute (Html.Attributes.disabled True) ]
                    |> Query.count (Expect.equal 1)
        , Test.test "disable manual refresh when already fetch in progress" <|
            \_ ->
                { initialModel | auto_refresh = False, fetching = True }
                    |> view
                    |> Query.fromHtml
                    |> Query.findAll [ id "manual-refresh", attribute (Html.Attributes.disabled True) ]
                    |> Query.count (Expect.equal 1)
        , Test.test "allow manual refresh when auto refresh is OFF and no fetch in progress" <|
            \_ ->
                { initialModel | auto_refresh = False, fetching = False }
                    |> view
                    |> Query.fromHtml
                    |> Query.findAll [ id "manual-refresh", attribute (Html.Attributes.disabled False) ]
                    |> Query.count (Expect.equal 1)
        , Test.test "disable auto refresh interval input when auto refresh is OFF" <|
            \_ ->
                initialModel
                    |> view
                    |> Query.fromHtml
                    |> Query.findAll [ id "auto-refresh-interval", attribute (Html.Attributes.disabled False) ]
                    |> Query.count (Expect.equal 1)
        , Test.test "allow auto refresh interval input when auto refresh is ON" <|
            \_ ->
                { initialModel | auto_refresh = True }
                    |> view
                    |> Query.fromHtml
                    |> Query.findAll [ id "auto-refresh-interval", attribute (Html.Attributes.disabled True) ]
                    |> Query.count (Expect.equal 1)
        ]


sampleDeploymentResultsWithErrors : DeploymentResults
sampleDeploymentResultsWithErrors =
    { deployments =
        [ { service_name = "service-a"
          , keys = "key-a"
          , status = "PRIMARY"
          , running_count = 2
          , desired_count = 2
          , pending_count = 0
          , failed_count = 0
          }
        ]
    , errors =
        [ { service_name = "service-b"
          , keys = "key-b"
          , error = "couldn't find credentials"
          }
        ]
    }


sampleDeploymentResultsWithNoErrors : DeploymentResults
sampleDeploymentResultsWithNoErrors =
    { deployments =
        [ { service_name = "service-a"
          , keys = "key-a"
          , status = "PRIMARY"
          , running_count = 2
          , desired_count = 2
          , pending_count = 0
          , failed_count = 0
          }
        ]
    , errors = []
    }


sampleDeploymentResultsWithErrorOnly : DeploymentResults
sampleDeploymentResultsWithErrorOnly =
    { deployments = []
    , errors =
        [ { service_name = "service-a"
          , keys = "key-a"
          , error = "couldn't find credentials"
          }
        , { service_name = "service-b"
          , keys = "key-b"
          , error = "couldn't find credentials"
          }
        ]
    }


sampleDeploymentResultsWithNoData : DeploymentResults
sampleDeploymentResultsWithNoData =
    { deployments = []
    , errors = []
    }


resultsTest : Test
resultsTest =
    Test.describe "Results view"
        [ Test.test "doesn't show any table on initial load" <|
            \_ ->
                initialModel
                    |> view
                    |> Query.fromHtml
                    |> Query.findAll [ tag "table" ]
                    |> Query.count (Expect.equal 0)
        , Test.test "shows both deployment details and errors if present" <|
            \_ ->
                { initialModel | status = Loaded sampleDeploymentResultsWithErrors }
                    |> view
                    |> Query.fromHtml
                    |> Query.findAll [ tag "table" ]
                    |> Query.count (Expect.equal 2)
        , Test.test "only shows deployment details if no errors present" <|
            \_ ->
                { initialModel | status = Loaded sampleDeploymentResultsWithNoErrors }
                    |> view
                    |> Query.fromHtml
                    |> Query.find [ tag "table" ]
                    |> Query.has [ id "deployment-details-table" ]
        , Test.test "only shows deployment errors if no details present" <|
            \_ ->
                { initialModel | status = Loaded sampleDeploymentResultsWithErrorOnly }
                    |> view
                    |> Query.fromHtml
                    |> Query.find [ tag "table" ]
                    |> Query.has [ id "deployment-errors-table" ]
        , Test.test "doesn't show deployment results if no data present" <|
            \_ ->
                { initialModel | status = Loaded sampleDeploymentResultsWithNoData }
                    |> view
                    |> Query.fromHtml
                    |> Query.findAll [ id "deployment-results" ]
                    |> Query.count (Expect.equal 0)
        , Test.test "shows no deployment found if no data present" <|
            \_ ->
                { initialModel | status = Loaded sampleDeploymentResultsWithNoData }
                    |> view
                    |> Query.fromHtml
                    |> Query.findAll [ id "no-deployment-results" ]
                    |> Query.count (Expect.equal 1)
        ]
