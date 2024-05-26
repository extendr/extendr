use extendr_api::prelude::*;

// Class for testing
#[derive(Default, Debug, Clone, Copy)]
struct Wrapper {
    a: i32,
}

/// Class for testing (exported)
/// @examples
/// x <- Wrapper$new()
/// x$a()
/// x$set_a(10)
/// x$a()
/// @export
#[extendr]
impl Wrapper {
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
    fn me_explicit_ref(&self) -> &Wrapper {
        self
    }

    /// Method for getting one's ref mut (explicit) self.
    fn me_explicit_mut(&mut self) -> &mut Wrapper {
        self
    }

    fn max_ref(&self, other: &'static Wrapper) -> &Self {
        if self.a > other.a {
            self
        } else {
            other
        }
    }

    /// `offset` does nothing.
    fn max_ref_offset(&self, other: &'static Wrapper, _offset: i32) -> &Self {
        if self.a > other.a {
            self
        } else {
            other
        }
    }

    fn max_ref2(&self, other: &'static Self) -> &Self {
        if self.a > other.a {
            self
        } else {
            other
        }
    }
}

// Macro to generate exports
extendr_module! {
    mod externalptr;
    impl Wrapper;
}
