# A runnable R example: load a small recorded feed and reconstruct the market
# snapshot at a past timestamp.
#
#   R CMD INSTALL bindings/r
#   Rscript examples/r/seek.R
#
# Every language example loads the same feed and prints the same summary.
library(wickratimemachine)

feed <- paste(
  paste0(
    '{"ts":10,"symbol":"SYM","feed":{"kind":"market","type":"trade",',
    '"symbol":{"base":"AAA","quote":"USDT"},"price":"100","quantity":"1",',
    '"aggressor":"Buy","timestamp":10}}'
  ),
  paste0(
    '{"ts":20,"symbol":"SYM","feed":{"kind":"market","type":"trade",',
    '"symbol":{"base":"AAA","quote":"USDT"},"price":"110","quantity":"2",',
    '"aggressor":"Sell","timestamp":20}}'
  ),
  sep = "\n"
)

# Encode the feed as a JSON string literal (escape backslash, quote, newline).
json_string <- function(s) {
  s <- gsub("\\\\", "\\\\\\\\", s)
  s <- gsub('"', '\\\\"', s)
  s <- gsub("\n", "\\\\n", s)
  paste0('"', s, '"')
}

tm <- wktimemachine_new("{}")
invisible(wktimemachine_command(tm, paste0('{"cmd":"load","data":', json_string(feed), "}")))
snapshot <- wktimemachine_command(tm, '{"cmd":"seek","ts":20}')

cat(sprintf("wickra-timemachine %s\n", wktimemachine_version()))
cat(sprintf("SYM last present: %s\n", grepl('"last":110', snapshot, fixed = TRUE)))
