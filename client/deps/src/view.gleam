import gleam/int
import gleam/json
import lustre/attribute
import lustre/element
import lustre/element/html
import lustre/event
import model.{type Model}
import types.{type Msg}

pub fn view(model: Model) -> element.Element(Msg) {
  html.div([attribute.class("container")], [
    html.div([attribute.class("bg-[#282828] text-[#ebdbb2]")], [
      html.div([], [model_debug_div(model), heading(model.fetching)]),
    ]),
  ])
}

fn model_debug_div(model: Model) -> element.Element(Msg) {
  case model.debug {
    True ->
      html.div(
        [attribute.class("debug bg-gray-800 text-white p-4 overflow-auto mb-5")],
        [
          html.pre([attribute.class("whitespace-pre-wrap")], [
            element.text(json.to_string(model.encode_model(model))),
          ]),
        ],
      )
    False -> element.none()
  }
}

fn heading(fetching: Bool) -> element.Element(Msg) {
  let heading_text = case fetching {
    True -> "ecscope ..."
    False -> "ecscope"
  }

  html.h1([attribute.class("text-3xl font-bold mb-6")], [
    html.a(
      [
        attribute.href("https://github.com/dhth/ecscope"),
        attribute.target("_blank"),
      ],
      [element.text(heading_text)],
    ),
  ])
}

fn fetch_controls_div(model: Model) -> element.Element(Msg) {
  html.div([attribute.class("mb-4 flex items-center space-x-4")], [
    html.div(
      [attribute.class("flex items-center space-x-2 py-2 rounded bg-[#282828]")],
      [
        html.label(
          [
            attribute.class("flex items-center space-x-2"),
            attribute.for("auto-refresh-toggle"),
          ],
          [element.text("auto refresh every")],
        ),
        html.input([
          attribute.class(
            "w-4 h-4 text-[#fabd2f] bg-[#282828] rounded focus:ring-[#fabd2f]",
          ),
          attribute.id("auto-refresh-toggle"),
          attribute.type_("checkbox"),
          event.on_check(types.AutoRefreshToggled),
          attribute.checked(model.auto_refresh),
        ]),
        html.input([attribute.id("auto-refresh-interval"),
        attribute.type_("number"), attribute.min("5"), attribute.max("300"),
        attribute.value(int.to_string(model.reload_seconds)])
      ],
    ),
  ])
}
