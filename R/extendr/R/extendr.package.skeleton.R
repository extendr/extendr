

extendr.package.skeleton <-function(
    name = "hello",
    list = character(),
    environment = .Goblalnv,
    path = ".",
    force = FALSE,
    code_files = character(),
    rust_files = character(),
    example_code = TRUE,
    attributes = TRUE,
    module = TRUE,
    author = "Your name here",
    mainainer = "maintainer name here",
    email = "maintainer.name@maintainer.org",
    license = "MIT"
) {
    if (!is(name, "character") || length(name) != 1) {
        stop("Expects a single string for package.name")
    }

    if (!is(rust_files, "character")) {
        stop("Expects a character vector for rust_files")
    }

    package.skeleton(
        name = name,
        list = list,
        environment = environment,
        path = path,
        force = force,
        code_files = code_files,
        example_code = example_code,
        attributes = attributes,
        module = module,
        author = author,
        mainainer = mainainer,
        email = email,
        license = license,
    )

    package.dir <- file.path(path, name)

    # modify DESCRIPTION
    desc.path = file.path(package.dir, "DESCRIPTION")
    if (file.exists(desc.path)) {
        dcf <- read.dcf(desc.path)
        print(dcf)
        # todo: mod DESCRIPTION here
        write.dcf(dcf, file=desc.path)
    }

    # modify NAMESPACE
    ns.path = file.path(package.dir, "NAMESPACE")
    if (file.exists(ns.path)) {
        ns.lines = readLines(ns.path)

        if (!grepl("useDynLib", ns.lines)) {
            # useDynLib is missing - write one
            ns.lines = c(sprintf("useDynLib(%s)", name), ns.lines)
        }
        if (!grepl("exportPattern", ns.lines)) {
            # exportPattern is missing - write one
            ns.lines = c("exportPattern(\"^[[:alpha:]]+\")", ns.lines)
        }

        writeLines(lines, ns.path)
    }
}
