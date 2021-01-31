use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_submodule() -> &'static str {
    "Hello World!"
}

// Class for testing
#[derive(Default, Debug)]
struct MySubmoduleClass {
    a: i32,
}

/// Class for testing (exported)
/// @examples
/// x <- MySubmoduleClass$new()
/// x$a()
/// x$set_a(10)
/// x$a()
/// @export
#[extendr]
impl MySubmoduleClass {
    /// Method for making a new object.
    fn new() -> Self {
        Self { a: 0 }
    }
    
    /// Method for setting stuff.
    /// @param x a number
    fn set_a(& mut self, x: i32) {
        self.a = x;
    }
    
    /// Method for getting stuff.
    fn a(&self) -> i32 {
        self.a
    }
    
    /// Method for getting one's self.
    fn me(&self) -> &Self {
        self
    }
}


// Macro to generate exports
extendr_module! {
    mod submodule;
    fn hello_submodule;
    impl MySubmoduleClass;
}
