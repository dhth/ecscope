module Cmds exposing (fetchDeps)

import Http
import Serde exposing (deploymentResultsDecoder)
import Types exposing (Msg(..))


fetchDeps : Cmd Msg
fetchDeps =
    Http.get
        { url = "/dev/api/deps"
        , expect = Http.expectJson ResultsFetched deploymentResultsDecoder
        }
