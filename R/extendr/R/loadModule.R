
# loadModule is called in a package's R code to expose Rust functions and impls
# in the package namespace.
#
# see https://github.com/RcppCore/Rcpp/blob/master/R/loadModule.R
# for the Rcpp version.
#
# example: Loads all code in the current package
# 
#    loadModule("myrustmod", TRUE)

loadModule <- function(
    module,
    what = character(),
    loadNow,
    env = topenv(parent.frame())
) {
    module.to.load = NULL
    if (is(module, "character")) {
        # eg. loadModule("fred") where "fred" is an Extendr module.

        # eg. convert "fred" to ".__Mod__fred"
        module.meta.name = methods::methodsPackageMetaName("Mod", name)

        # eg. does env contain ".__Mod__fred"?
        if (exists(module.meta.name, envir = env, inherits = FALSE)) {
            # get existing module
            module.to.load = get(module.meta.name, envir = env)
        }
    } else {
        stop("Argument \"module\" should be the name of a module.")
    }
}
