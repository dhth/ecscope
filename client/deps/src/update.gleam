import effects
import gleam/int
import lustre/effect
import model.{type Model, Model}
import types.{type Msg}

pub fn update(model: Model, msg: Msg) -> #(Model, effect.Effect(Msg)) {
  case msg {
    types.AutoRefreshScheduleChanged(seconds_string) ->
      case int.parse(seconds_string) {
        Error(_) -> #(model, effect.none())
        Ok(seconds) ->
          case seconds {
            s if s >= 5 && s <= 300 -> #(
              Model(..model, reload_seconds: s),
              effect.none(),
            )
            _ -> #(Model(..model, reload_seconds: 5), effect.none())
          }
      }
    types.AutoRefreshToggled(checked) ->
      case checked {
        // TODO
        c if c && model.fetching -> #(
          Model(..model, fetching: True, auto_refresh: True),
          effect.none(),
        )
        _ -> #(Model(..model, auto_refresh: checked), effect.none())
      }
    types.FetchResults -> #(
      Model(..model, fetching: True),
      effects.fetch_deps(),
    )
    types.ResultsFetched(results) ->
      case results {
        Error(err) -> #(
          Model(..model, status: types.Errored(err), fetching: False),
          effect.none(),
        )
        Ok(results) -> #(
          Model(..model, status: types.Loaded(results), fetching: False),
          effect.none(),
        )
      }
    types.Tick -> #(model, effect.none())
    // TODO
  }
}
