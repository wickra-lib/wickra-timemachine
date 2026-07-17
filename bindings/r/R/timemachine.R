#' The wickra-timemachine library version.
#' @return A version string.
#' @export
wktimemachine_version <- function() {
  .Call(C_wktimemachine_version)
}

#' Build a time-machine handle from a spec JSON.
#' @param spec_json A `TimelineSpec` JSON string (`"{}"` uses the default spec).
#' @return A `wickra_timemachine` handle (an external pointer).
#' @export
wktimemachine_new <- function(spec_json) {
  .Call(C_wktimemachine_new, spec_json)
}

#' Apply a command JSON and return the resulting response JSON.
#' @param tm A time-machine handle from [wktimemachine_new()].
#' @param cmd_json A command JSON string (`load`, `seek`, `state_at`, `play`,
#'   `version`).
#' @return The response as a JSON string.
#' @export
wktimemachine_command <- function(tm, cmd_json) {
  .Call(C_wktimemachine_command, tm, cmd_json)
}
