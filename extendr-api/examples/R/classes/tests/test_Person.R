
library(classes)

p <- new("Person")

stopifnot(mode(p) == "S4")

p$set_name("xyz")

stopifnot(p$name() == "xyz")

gc()
rm(p)
gc()
