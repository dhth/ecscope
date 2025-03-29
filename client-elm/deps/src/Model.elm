module Model exposing (Model, initialModel)

import Types exposing (Status(..))


type alias Model =
    { status : Status
    , reload_seconds : Int
    , auto_refresh : Bool
    , fetching : Bool
    , debug : Bool
    }


initialModel : Model
initialModel =
    { status = Loading
    , reload_seconds = 5
    , auto_refresh = False
    , fetching = True
    , debug = False
    }
