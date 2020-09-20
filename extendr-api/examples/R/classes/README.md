# Class example

Build in this directory with:

    R CMD INSTALL .

Then in R:

    > library(classes)
    >
    > # construct a new Person
    > jim <- Person$new()
    >
    > # set the name
    > jim$set_name("Jim")
    >
    > # get the name
    > print(jim$name())
