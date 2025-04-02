import gleam/int
import gleam/json
import gleam/list
import gleam/string
import lustre/attribute
import lustre/element
import lustre/element/html
import lustre/event
import lustre_http
import model.{type Model}
import types.{type Msg}
import utils

pub fn view(model: Model) -> element.Element(Msg) {
  html.div([attribute.class("w-2/3 mx-auto bg-[#282828] text-[#ebdbb2] mt-8")], [
    html.div([], [
      html.div([], [
        model_debug_div(model),
        heading(model.fetching),
        fetch_controls_div(model),
        main_div(model),
      ]),
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

  html.h1([attribute.class("text-3xl font-bold mb-4")], [
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
      [attribute.class("flex items-center space-x-2 py-2 bg-[#282828]")],
      [
        html.input([
          attribute.class(
            "w-4 h-4 text-[#fabd2f] bg-[#282828] focus:ring-[#fabd2f]",
          ),
          attribute.id("auto-refresh-toggle"),
          attribute.type_("checkbox"),
          event.on_check(types.AutoRefreshToggled),
          attribute.checked(model.auto_refresh),
        ]),
        html.label(
          [
            attribute.class("items-center space-x-2"),
            attribute.for("auto-refresh-toggle"),
          ],
          [element.text("auto refresh every")],
        ),
        html.input([
          attribute.class(
            "h-8 text-center text-[#ebdbb2] bg-[#3c3836] focus:ring-[#fabd2f] pl-5 pr-2",
          ),
          attribute.id("auto-refresh-interval"),
          attribute.type_("number"),
          attribute.min("5"),
          attribute.max("60"),
          attribute.value(int.to_string(model.reload_seconds)),
          attribute.disabled(model.auto_refresh),
          event.on_input(types.AutoRefreshScheduleChanged),
        ]),
        html.p([attribute.class("items-center space-x-2")], [
          element.text("seconds"),
        ]),
        html.div([], [
          html.button(
            [
              attribute.id("manual-refresh"),
              attribute.class(
                "font-semibold px-4 py-1 ml-4 bg-[#d3869b] text-[#282828] disabled:bg-[#928374]",
              ),
              attribute.disabled(model.auto_refresh || model.fetching),
              event.on_click(types.FetchResults),
            ],
            [element.text("Refresh")],
          ),
        ]),
      ],
    ),
  ])
}

fn main_div(model: Model) -> element.Element(Msg) {
  html.div([attribute.class("mb-8")], [
    case model.status {
      types.Errored(error) -> http_error_div(error)
      types.Loaded(results) -> results_div(results)
      types.Loading -> loading_div()
    },
  ])
}

fn http_error_div(error: lustre_http.HttpError) -> element.Element(Msg) {
  html.div([attribute.class("error-message")], [
    html.p([], [element.text("Error: " <> utils.http_error_to_string(error))]),
  ])
}

fn loading_div() -> element.Element(Msg) {
  html.div([], [
    html.h2([attribute.class("text-xl font-bold mb-6 text-[#fabd2f]")], [
      element.text("Loading..."),
    ]),
  ])
}

fn results_div(results: types.DeploymentResults) -> element.Element(Msg) {
  let deps_present = !list.is_empty(results.deployments)
  let errors_present = !list.is_empty(results.errors)

  let results = case deps_present, errors_present {
    False, False -> [no_deployment_results_div()]
    True, False -> [deployment_details_div(results.deployments)]
    False, True -> [deployment_errors_div(results.errors)]
    True, True -> [
      deployment_details_div(results.deployments),
      deployment_results_divider(),
      deployment_errors_div(results.errors),
    ]
  }

  html.div([attribute.id("deployment-results")], results)
}

fn no_deployment_results_div() -> element.Element(Msg) {
  html.div([attribute.id("no-deployment-results")], [
    html.h2([attribute.class("text-xl font-bold mb-6 text-[#83a598]")], [
      element.text("No Deployments found"),
    ]),
  ])
}

fn deployment_details_div(
  deployments: List(types.Deployment),
) -> element.Element(Msg) {
  html.div([attribute.id("deployment-details")], [
    html.h2([attribute.class("text-xl font-bold mb-6 text-[#83a598]")], [
      element.text("Deployments"),
    ]),
    html.div([attribute.class("legend mb-4 p-4")], [
      html.p([attribute.class("p-1 m-1 font-semibold")], [
        element.text("Legend: "),
      ]),
      html.p([attribute.class("legend-upcoming py-1 px-2 m-1 font-semibold")], [
        element.text("upcoming"),
      ]),
      html.p([attribute.class("legend-active py-1 px-2 m-1 font-semibold")], [
        element.text("being replaced"),
      ]),
      html.p([attribute.class("legend-draining py-1 px-2 m-1 font-semibold")], [
        element.text("draining"),
      ]),
      html.p([attribute.class("legend-failing py-1 px-2 m-1 font-semibold")], [
        element.text("failing"),
      ]),
    ]),
    deployments_table(deployments),
  ])
}

fn deployments_table(
  deployments: List(types.Deployment),
) -> element.Element(Msg) {
  html.table(
    [
      attribute.class("table-auto w-full px-4 py-2"),
      attribute.id("deployment-details-table"),
    ],
    [
      html.thead([], [
        html.tr([], [
          html.th([], [element.text("Service")]),
          html.th([], [element.text("Keys")]),
          html.th([], [element.text("Status")]),
          html.th([], [element.text("Running")]),
          html.th([], [element.text("Desired")]),
          html.th([], [element.text("Pending")]),
          html.th([], [element.text("Failed")]),
        ]),
      ]),
      html.tbody([], list.map(deployments, deployment_table_row)),
    ],
  )
}

fn deployment_table_row(deployment: types.Deployment) -> element.Element(Msg) {
  html.tr([attribute.class(table_row_class(deployment))], [
    service_name_table_data(deployment),
    html.td([], [element.text(deployment.keys)]),
    html.td([], [element.text(deployment.status)]),
    html.td([], [element.text(int.to_string(deployment.running_count))]),
    html.td([], [element.text(int.to_string(deployment.desired_count))]),
    html.td([], [element.text(int.to_string(deployment.pending_count))]),
    html.td([], [element.text(int.to_string(deployment.failed_count))]),
  ])
}

fn table_row_class(deployment: types.Deployment) -> String {
  case deployment {
    _ if deployment.failed_count > 0 -> "row-failing"
    _ if deployment.status == "ACTIVE" -> "row-being-replaced"
    _ if deployment.running_count != deployment.desired_count -> "row-upcoming"
    _ if deployment.status == "DRAINING" -> "row-draining"
    _ -> ""
  }
}

fn service_name_table_data(deployment: types.Deployment) -> element.Element(Msg) {
  let service_color = utils.color_for_string(deployment.service_name)
  case deployment {
    _
      if deployment.failed_count > 0
      || deployment.status == "ACTIVE"
      || deployment.running_count != deployment.desired_count
      || deployment.status == "DRAINING"
    ->
      html.td([attribute.class("font-semibold")], [
        element.text(deployment.service_name),
      ])

    _ ->
      html.td([attribute.class("font-semibold " <> service_color)], [
        element.text(deployment.service_name),
      ])
  }
}

fn deployment_results_divider() -> element.Element(Msg) {
  html.hr([
    attribute.class("border-y-2 my-10 border-[#504945]"),
    attribute.id("results-divider"),
  ])
}

fn deployment_errors_div(
  errors: List(types.DeploymentError),
) -> element.Element(Msg) {
  html.div([attribute.id("deployment-errors")], [
    html.h2([attribute.class("text-xl font-bold mb-6 text-[#fb4934]")], [
      element.text("Errors"),
    ]),
    deployment_errors_table(errors),
  ])
}

fn deployment_errors_table(
  errors: List(types.DeploymentError),
) -> element.Element(Msg) {
  html.table(
    [
      attribute.class("table-auto w-full px-4 py-2"),
      attribute.id("deployment-errors-table"),
    ],
    [
      html.thead([], [
        html.tr([], [
          html.th([], [element.text("Service")]),
          html.th([], [element.text("Keys")]),
          html.th([], [element.text("Error")]),
        ]),
      ]),
      html.tbody([], list.map(errors, deployment_error_row)),
    ],
  )
}

fn deployment_error_row(error: types.DeploymentError) -> element.Element(Msg) {
  html.tr([attribute.class("border-y-2 border-[#504945]")], [
    error_table_service_name_td(error),
    html.td([], [element.text(error.keys)]),
    html.td([], error_with_new_lines(error.error)),
  ])
}

fn error_table_service_name_td(
  error: types.DeploymentError,
) -> element.Element(Msg) {
  let service_color = utils.color_for_string(error.service_name)
  html.td([attribute.class("font-semibold text-[" <> service_color <> "]")], [
    element.text(error.service_name),
  ])
}

fn error_with_new_lines(text: String) -> List(element.Element(Msg)) {
  text
  |> string.split("\n")
  |> list.map(fn(s) { html.div([], [element.text(s)]) })
}
