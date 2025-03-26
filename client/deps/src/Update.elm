module Update exposing (update)

import Cmds exposing (fetchDeps)
import Subscriptions exposing (scheduleNextTick)
import Types exposing (Model, Msg(..), Status(..))


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        FetchResults ->
            ( { model | fetching = True }, fetchDeps )

        AutoRefreshToggled checked ->
            if checked && model.fetching then
                ( { model | fetching = True, auto_refresh = True }, Cmd.batch [ fetchDeps, scheduleNextTick model ] )

            else
                ( { model | auto_refresh = checked }, Cmd.none )

        AutoRefreshScheduleChanged maybeSeconds ->
            case maybeSeconds of
                Just seconds ->
                    if seconds >= 5 && seconds <= 300 then
                        ( { model | reload_seconds = seconds }, Cmd.none )

                    else
                        ( { model | reload_seconds = 5 }, Cmd.none )

                Nothing ->
                    ( { model | reload_seconds = 5 }, Cmd.none )

        ResultsFetched results ->
            case results of
                Ok response ->
                    ( { model | status = Loaded response, fetching = False }, Cmd.none )

                Err err ->
                    ( { model | status = Errored err, fetching = False }, Cmd.none )

        Tick ->
            if model.auto_refresh then
                if model.fetching then
                    ( model, scheduleNextTick model )

                else
                    ( { model | fetching = True }, Cmd.batch [ fetchDeps, scheduleNextTick model ] )

            else
                ( model, Cmd.none )
