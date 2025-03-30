import effects.{fetch_deps}
import lustre
import lustre/effect
import model.{type Model, Model}
import types.{type Msg, Loading}
import update
import view

pub fn main() {
  let app = lustre.application(init, update.update, view.view)
  let assert Ok(_) = lustre.start(app, "#app", Nil)
}

fn init(_) -> #(Model, effect.Effect(Msg)) {
  #(
    Model(
      status: Loading,
      reload_seconds: 5,
      auto_refresh: False,
      fetching: True,
      debug: False,
    ),
    fetch_deps(),
  )
}
