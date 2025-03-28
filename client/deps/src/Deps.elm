module Deps exposing (main)

import Browser
import Cmds exposing (fetchDeps)
import Model exposing (Model, initialModel)
import Subscriptions exposing (subs)
import Types exposing (Msg, Status(..))
import Update exposing (update)
import View exposing (view)


main : Program () Model Msg
main =
    Browser.element
        { init = \_ -> ( initialModel, fetchDeps )
        , view = view
        , update = update
        , subscriptions = subs
        }
