skip_if_no_cargo_expand <- function() {
  result <- tryCatch(
    processx::run(
      "cargo",
      args = c("expand", "--version"),
      error_on_status = FALSE
    ),
    error = function(e) list(status = 1, stderr = "")
  )
  condition <- (result$status == 0) && (!nzchar(result$stderr))
  testthat::skip_if_not(condition, "cargo expand not available")
}

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

  testthat::skip_if(isTRUE(condition), "`nightly` toolchain")
}
