import lustre/effect
import lustre_http
import types.{type Msg, deployment_results_decoder}

pub fn fetch_deps() -> effect.Effect(Msg) {
  let expect =
    lustre_http.expect_json(deployment_results_decoder(), types.ResultsFetched)

  lustre_http.get("http://127.0.0.1:4500/dev/api/deps", expect)
}
