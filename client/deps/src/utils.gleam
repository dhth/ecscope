import gleam/int
import gleam/list
import gleam/string
import lustre_http

const color_pool = ["#fe77a4", "#d3869a", "#ff4c8b"]

pub fn http_error_to_string(error: lustre_http.HttpError) -> String {
  case error {
    lustre_http.BadUrl(url) -> "bad url: " <> url
    lustre_http.InternalServerError(error) -> "internal server error: " <> error
    lustre_http.JsonError(error) -> "json error: " <> string.inspect(error)
    lustre_http.NetworkError -> "network error"
    lustre_http.NotFound -> "not found"
    lustre_http.OtherError(code, body) ->
      "server returned " <> int.to_string(code) <> " code with body: " <> body
    lustre_http.Unauthorized -> "unauthorized"
  }
}

fn simple_hash(input: String) -> Int {
  string.to_utf_codepoints(input)
  |> list.map(string.utf_codepoint_to_int)
  |> list.fold(0, fn(a, b) { a + b })
}

pub fn color_for_string(input: String) -> String {
  let hash = simple_hash(input)
  // TODO: what a monstrosity
  // how to have static arrays with O(1) access in gleam??
  case list.split(color_pool, hash) {
    #(_, colors) ->
      case list.first(colors) {
        Error(_) -> "#000000"
        Ok(color) -> color
      }
  }
}
