import gleam/int
import gleam/list
import gleam/result
import gleam/string
import lustre_http

// keep this up to date with number of custom colors in tailwind.config.js
pub const num_custom_colors = 45

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
  |> list.fold(0, fn(a, b) { a * 31 + b })
}

pub fn color_for_string(input: String) -> String {
  let hash = simple_hash(input)
  let index = hash |> int.remainder(num_custom_colors) |> result.unwrap(0)
  "text-color" <> int.to_string(index)
}
