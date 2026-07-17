## Plain-R tests for the wickra-timemachine R binding (no testthat dependency).
## Mirrors the Rust/Python/Node/Go/C#/Java tests and doubles as the completeness
## guard: it exercises the full public surface (version + new + command).

library(wickratimemachine)

## A deterministic two-record feed on SYM: a rising then falling trade.
feed <- paste(
  paste0(
    '{"ts":10,"symbol":"SYM","feed":{"kind":"market","type":"trade",',
    '"symbol":{"base":"AAA","quote":"USDT"},"price":"100","quantity":"1",',
    '"aggressor":"Buy","timestamp":10}}'
  ),
  paste0(
    '{"ts":20,"symbol":"SYM","feed":{"kind":"market","type":"trade",',
    '"symbol":{"base":"AAA","quote":"USDT"},"price":"105","quantity":"1",',
    '"aggressor":"Sell","timestamp":20}}'
  ),
  sep = "\n"
)

load_cmd <- function() {
  paste0('{"cmd":"load","data":', toString_json(feed), "}")
}

## Encode a string as a JSON string literal (escape backslash, quote, newline).
toString_json <- function(s) {
  s <- gsub("\\\\", "\\\\\\\\", s)
  s <- gsub('"', '\\\\"', s)
  s <- gsub("\n", "\\\\n", s)
  paste0('"', s, '"')
}

## version
stopifnot(nzchar(wktimemachine_version()))

## seek reconstructs the snapshot at ts = 20 (last trade at 105)
tm <- wktimemachine_new("{}")
invisible(wktimemachine_command(tm, load_cmd()))
snap <- wktimemachine_command(tm, '{"cmd":"seek","ts":20}')
stopifnot(grepl('"ts":20', snap, fixed = TRUE))
stopifnot(grepl('"last":105', snap, fixed = TRUE))

## seek is byte-identical across handles (the cross-language golden core)
tm2 <- wktimemachine_new("{}")
invisible(wktimemachine_command(tm2, load_cmd()))
snap2 <- wktimemachine_command(tm2, '{"cmd":"seek","ts":20}')
stopifnot(identical(snap, snap2))

## an invalid spec is a hard error at construction
err <- tryCatch(wktimemachine_new("{ not valid json"), error = function(e) e)
stopifnot(inherits(err, "error"))

cat("wickra-timemachine R tests passed\n")
