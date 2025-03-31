import gleam/dict
import gleam/dynamic
import gleam/dynamic/decode
import gleeunit/should
import types.{deployment_results_decoder}

pub fn decode_deployment_test() {
  // GIVEN
  let results =
    dynamic.from(
      dict.from_list([
        #("service_name", dynamic.from("service-a")),
        #("keys", dynamic.from("qa")),
        #(
          "cluster_arn",
          dynamic.from("arn:aws:ecs:eu-central-1:000000000000:cluster/cluster"),
        ),
        #("deployment_id", dynamic.from("ecs-svc/0000000000000000000")),
        #("status", dynamic.from("PRIMARY")),
        #("running_count", dynamic.from(2)),
        #("desired_count", dynamic.from(2)),
        #("pending_count", dynamic.from(0)),
        #("failed_count", dynamic.from(0)),
      ]),
    )
  let errors =
    dynamic.from(
      dict.from_list([
        #("service_name", dynamic.from("service-b")),
        #("keys", dynamic.from("staging")),
        #(
          "error",
          dynamic.from(
            "couldn't find profile\nmake sure profile is set up correctly",
          ),
        ),
      ]),
    )

  let data =
    dynamic.from(
      dict.from_list([#("deployments", [results]), #("errors", [errors])]),
    )
  let decoder = deployment_results_decoder()

  // WHEN
  // THEN
  data |> decode.run(decoder) |> should.be_ok
}
