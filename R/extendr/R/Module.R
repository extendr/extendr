
# getNativeSymbolInfo

# example:
# mod <- Module("mod_formals", getDynLib(fx))
# norm <- mod$norm
# norm()
# norm(x = 2, y = 3)

Module <- function(
    module_name,
    PACKAGE = methods::getPackageName(where),
    where = topenv(parent.frame()),
    mustStart = FALSE
) {
    
}
