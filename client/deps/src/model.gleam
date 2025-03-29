import gleam/json
import types.{type Status, Loading}

pub type Model {
  Model(
    status: Status,
    reload_seconds: Int,
    auto_refresh: Bool,
    fetching: Bool,
    debug: Bool,
  )
}

pub fn encode_model(model: Model) -> json.Json {
  json.object([
    #("reload_seconds", json.int(model.reload_seconds)),
    #("auto_refresh", json.bool(model.auto_refresh)),
    #("fetching", json.bool(model.fetching)),
    #("debug", json.bool(model.debug)),
  ])
}

pub fn initial_model() -> Model {
  Model(
    status: Loading,
    reload_seconds: 5,
    auto_refresh: False,
    fetching: True,
    debug: False,
  )
}
