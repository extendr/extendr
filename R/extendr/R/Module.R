
# example:
#   mod <- Module("fred")
#   mod$
Module <- function(
    module,
    PACKAGE = methods::getPackageName(where),
    where = topenv(parent.frame()),
    mustStart = FALSE
) {
    if(!is.character(module)) {
        stop("Argument \"module\" should be the name of a module.")
    }


}
