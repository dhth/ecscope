module Cmds exposing (fetchDeps)

import Http
import Types exposing (Msg(..), deploymentResultsDecoder)


fetchDeps : Cmd Msg
fetchDeps =
    Http.get
        { url = "/api/deps"
        , expect = Http.expectJson ResultsFetched deploymentResultsDecoder
        }
