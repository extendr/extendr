use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_submodule() -> &'static str {
    "Hello World!"
}

// Class for testing
#[derive(Default, Debug, Clone, Copy)]
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
    fn set_a(&mut self, x: i32) {
        self.a = x;
    }

    /// Method for getting stuff.
    fn a(&self) -> i32 {
        self.a
    }

    // NOTE: Cannot move ownership, as that concept is incompatible with bridging
    // data from R to Rust
    // fn myself(self) -> Self {
    //     self
    // }

    /// Method for getting one's (by way of a copy) self.
    fn me_owned(&self) -> Self {
        // only possible due to `derive(Clone, Copy)`
        *self
    }

    /// Method for getting one's (ref) self.
    fn me_ref(&self) -> &Self {
        self
    }

    /// Method for getting one's (ref mut) self.
    fn me_mut(&mut self) -> &mut Self {
        self
    }

    /// Method for getting one's ref (explicit) self.
    fn me_explicit_ref(&self) -> &MySubmoduleClass {
        self
    }

    /// Method for getting one's ref mut (explicit) self.
    fn me_explicit_mut(&mut self) -> &mut MySubmoduleClass {
        self
    }
}

// Macro to generate exports
extendr_module! {
    mod submodule;
    fn hello_submodule;
    impl MySubmoduleClass;
}
