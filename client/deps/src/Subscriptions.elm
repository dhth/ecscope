module Subscriptions exposing (scheduleNextTick, subs)

import Process
import Task
import Time
import Types exposing (Model, Msg(..))


subs : Model -> Sub Msg
subs model =
    if model.auto_refresh then
        Time.every (toFloat model.reload_seconds * 1000) (\_ -> Tick)

    else
        Sub.none


scheduleNextTick : Model -> Cmd Msg
scheduleNextTick model =
    Process.sleep (toFloat (model.reload_seconds * 1000))
        |> Task.perform (\_ -> Tick)
