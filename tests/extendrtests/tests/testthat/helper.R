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
  testthat::skip_if(condition, "cargo expand not available")
}
