import gleam/dynamic/decode
import gleam/option
import lustre_http

pub type Deployment {
  Deployment(
    service_name: String,
    keys: String,
    status: String,
    running_count: Int,
    desired_count: Int,
    pending_count: Int,
    failed_count: Int,
  )
}

fn deployment_decoder() -> decode.Decoder(Deployment) {
  use service_name <- decode.field("service_name", decode.string)
  use keys <- decode.field("keys", decode.string)
  use status <- decode.field("status", decode.string)
  use running_count <- decode.field("running_count", decode.int)
  use desired_count <- decode.field("desired_count", decode.int)
  use pending_count <- decode.field("pending_count", decode.int)
  use failed_count <- decode.field("failed_count", decode.int)
  decode.success(Deployment(
    service_name:,
    keys:,
    status:,
    running_count:,
    desired_count:,
    pending_count:,
    failed_count:,
  ))
}

pub type DeploymentError {
  DeploymentError(service_name: String, keys: String, error: String)
}

fn deployment_error_decoder() -> decode.Decoder(DeploymentError) {
  use service_name <- decode.field("service_name", decode.string)
  use keys <- decode.field("keys", decode.string)
  use error <- decode.field("error", decode.string)
  decode.success(DeploymentError(service_name:, keys:, error:))
}

pub type DeploymentResults {
  DeploymentResults(
    deployments: List(Deployment),
    errors: List(DeploymentError),
  )
}

pub fn deployment_results_decoder() -> decode.Decoder(DeploymentResults) {
  use deployments <- decode.field(
    "deployments",
    decode.list(deployment_decoder()),
  )
  use errors <- decode.field("errors", decode.list(deployment_error_decoder()))
  decode.success(DeploymentResults(deployments:, errors:))
}

pub type Status {
  Loading
  Loaded(DeploymentResults)
  Errored(lustre_http.HttpError)
}

pub type RefreshScheduleNumSeconds =
  option.Option(Int)

pub type Msg {
  FetchResults
  AutoRefreshToggled(Bool)
  AutoRefreshScheduleChanged(RefreshScheduleNumSeconds)
  ResultsFetched(Result(DeploymentResults, lustre_http.HttpError))
  Tick
}
