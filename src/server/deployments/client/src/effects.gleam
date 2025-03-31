import lustre/effect
import lustre_http
import plinth/browser/window
import plinth/javascript/global
import types.{type Msg, deployment_results_decoder}

pub fn fetch_deps() -> effect.Effect(Msg) {
  let expect =
    lustre_http.expect_json(deployment_results_decoder(), types.ResultsFetched)

  lustre_http.get(window.location() <> "api/deps", expect)
}

pub fn schedule_next_tick(delay_seconds: Int) -> effect.Effect(Msg) {
  effect.from(fn(dispatch) {
    global.set_timeout(delay_seconds * 1000, fn() { dispatch(types.Tick) })
    Nil
  })
}
