import gleam/int
import gleam/list
import gleam/result
import gleam/string
import lustre_http

const color_pool = [
  "#fd780b", "#a882a7", "#b798f0", "#59d412", "#7bcaff", "#ffb472", "#00ce48",
  "#1edacd", "#a0d845", "#a681fb", "#f081de", "#63bd8f", "#d3cdc5", "#64d97f",
  "#acaa5e", "#90e1ef", "#ddd601", "#4896ef", "#e98658", "#b5d092", "#9fb9f0",
  "#ff6682", "#149ccd", "#59c435", "#83a598", "#f27abe", "#b9d9cf", "#88b500",
  "#faca7d", "#f344ff", "#f19597", "#aab08a", "#fabd2f", "#0abe88", "#c9b094",
  "#a6b92b", "#b4c800", "#00c3f9", "#d89e9d", "#48cae4", "#00b700", "#4dcfdb",
  "#e96462", "#7b8ad5", "#00b499", "#fe9103", "#04eb4d", "#daa402", "#5cab95",
  "#ce8cf7", "#7db839", "#d3869a", "#f0947b", "#a7e0c2", "#ff4ded", "#b5e48c",
  "#ff4c8b", "#ff9743", "#ffc6ff", "#de9644", "#8fbc96", "#b29807", "#ffc20c",
  "#be876e", "#ceb4c3", "#ee91b6", "#a8b64c", "#fe77a4", "#7a9879", "#febcac",
  "#add562", "#a8d906", "#b4d4fb", "#dcad50", "#ff5405", "#e47cfb", "#f646c1",
  "#c6a267", "#ada7ff", "#d4925c", "#8187dc", "#6ac1db", "#5aaaff", "#5ddb63",
  "#94bc63", "#c7921f", "#83b87a", "#e2ac85", "#ffb0c2", "#99bbcd", "#c57fbf",
  "#fc5260", "#6fbd63", "#4daa67", "#00d977", "#0798ff", "#07b1fa", "#cec48b",
  "#c3a4e1", "#849843", "#bbd1ff", "#df748b", "#9ae089", "#a8a9a3", "#89d967",
  "#e7c727", "#8498fb", "#829b60", "#62a6ae", "#dc8b00", "#6ed999", "#e6a5f4",
  "#9cdea5", "#aba3ca", "#4ba539", "#c7b648", "#949aab", "#9c9360", "#05b64c",
  "#ffb13c", "#b8bb26", "#8de107", "#6fd0bd", "#d1cc74", "#ffb5a2", "#03d7b3",
  "#00db04", "#90b4a6", "#fbcf56", "#bbb206", "#51a100", "#ff803b", "#ff6334",
  "#af9084", "#00d990", "#aaaffc", "#ae8c99", "#89cbce", "#19b7b2", "#71c200",
  "#629fdb", "#6da2c6", "#12b667", "#fc69e6", "#01aac0", "#8ce852", "#c97df9",
  "#d2c8f2", "#d67717", "#dfa5ca", "#00ddff", "#6e9f3a",
]

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
  let index = hash |> int.remainder(list.length(color_pool)) |> result.unwrap(0)
  // TODO: what a monstrosity
  // how to have static arrays with O(1) access in gleam??
  case list.split(color_pool, index) {
    #(_, colors) ->
      case list.first(colors) {
        Error(_) -> "#ebdbb2"
        Ok(color) -> color
      }
  }
}
