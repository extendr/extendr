initSidebarItems({"attr":[["extendr",""]],"constant":[["FALSE","FALSE value eg. `r!(FALSE)`"],["NA_INTEGER","NA value for integers eg. `r!(NA_INTEGER)`"],["NA_LOGICAL","NA value for logical. `r!(NA_LOGICAL)`"],["NA_REAL","NA value for real values eg. `r!(NA_REAL)`"],["NA_STRING","NA value for strings. `r!(NA_STRING)`"],["NULL","NULL value eg. `r!(NULL)`"],["TRUE","TRUE value eg. `r!(TRUE)`"]],"derive":[["IntoDataFrameRow","Enable the construction of dataframes from arrays of structures."],["IntoRobj","Derives an implementation of `From<Struct> for Robj` and `From<&Struct> for Robj` on this struct."],["TryFromRobj","Derives an implementation of `TryFrom<Robj> for Struct` and `TryFrom<&Robj> for Struct` on this struct."]],"enum":[["Rany","Enum use to unpack R objects into their specialist wrappers."],["Rtype","Type of R objects used by [Robj::rtype]."]],"fn":[["rtype_to_sxp","Convert extendr’s Rtype to R’s SEXPTYPE. Panics if the type is Unknown."],["sxp_to_rtype","Convert R’s SEXPTYPE to extendr’s Rtype."]],"macro":[["R","Execute R code by parsing and evaluating tokens."],["Rraw","Execute R code by parsing and evaluating tokens but without expanding parameters."],["call","Call a function or primitive defined by a text expression with arbitrary parameters. This currently works by parsing and evaluating the string in R, but will probably acquire some shortcuts for simple expessions, for example by caching symbols and constant values."],["data_frame","Create a dataframe."],["extendr_module","Define a module and export symbols to R Example:"],["factor","Create a factor."],["global","Get a global variable."],["lang","A macro for constructing R langage objects."],["list","Create a List R object from a list of name-value pairs."],["pairlist","Create a Pairlist R object from a list of name-value pairs."],["r","Convert a rust expression to an R object."],["reprint","Print via the R error stream."],["reprintln","Print with a newline via the R output stream."],["rprint","Print via the R output stream."],["rprintln","Print with a newline via the R output stream."],["sym","The sym! macro install symbols. You should cache your symbols in variables as generating them is costly."],["test","Macro for running tests."],["var","Get a local variable from the calling function or a global variable if no such variable exists."]],"mod":[["deserializer","Convert R objects to a wide variety of types."],["error","Error handling in Rust called from R."],["functions",""],["graphics","Graphic Device Operations"],["io",""],["iter",""],["lang_macros","Argument parsing and checking."],["metadata","Module metadata"],["na",""],["ownership","Maintain ownership of R objects."],["prelude","Common exports for extendr-api."],["rmacros","rmacros - a set of macros to call actual R functions in a rusty way."],["robj","R object handling."],["robj_ndarray",""],["scalar",""],["serializer","See https://serde.rs/impl-serializer.html"],["thread_safety","Provide limited protection for multithreaded access to the R API."],["wrapper","Wrappers are lightweight proxies for references to R datatypes. They do not contain an Robj (see array.rs for an example of this)."]],"trait":[["Deref","Used for immutable dereferencing operations, like `*v`."],["DerefMut","Used for mutable dereferencing operations, like in `*v = 1;`."],["TryFrom","Simple and safe type conversions that may fail in a controlled way under some circumstances. It is the reciprocal of [`TryInto`]."],["TryInto","An attempted conversion that consumes `self`, which may or may not be expensive."]]});