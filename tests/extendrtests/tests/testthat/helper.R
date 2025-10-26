skip_if_on_nightly <- function() {
  result <- tryCatch(
    processx::run(
      "rustup",
      args = "default",
      error_on_status = FALSE
    ),
    error = function(e) list(status = 1)
  )
  if (result[["status"]] == 0) {
    stdout <- result[["stdout"]]
    condition <- stringi::stri_startswith_fixed(
      stringi::stri_trim_left(stdout),
      pattern = "nightly"
    )
  } else {
    condition <- TRUE # rustup failed, something is fishy
  }

  skip_if(isTRUE(condition), "`nightly` toolchain")
}
