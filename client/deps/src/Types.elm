module Types exposing (..)

import Http


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


type Msg
    = FetchResults
    | AutoRefreshToggled Bool
    | AutoRefreshScheduleChanged RefreshScheduleNumSeconds
    | ResultsFetched (Result Http.Error DeploymentResults)
    | Tick
