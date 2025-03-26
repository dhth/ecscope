module Deps exposing (main)

import Browser
import Cmds exposing (fetchDeps)
import Subscriptions exposing (subs)
import Types exposing (Model, Msg, Status(..))
import Update exposing (update)
import View exposing (view)


initialModel : Model
initialModel =
    { status = Loading
    , reload_seconds = 5
    , auto_refresh = False
    , fetching = True
    , debug = False
    }


main : Program () Model Msg
main =
    Browser.element
        { init = \_ -> ( initialModel, fetchDeps )
        , view = view
        , update = update
        , subscriptions = subs
        }
