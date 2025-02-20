use extendr_api::prelude::*;

// Class for testing
#[derive(Default, Debug, Clone, Copy)]
#[extendr]
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

#[extendr]
fn externalptr_use_ref_manually() -> ExternalPtr<i32> {
    let extptr = ExternalPtr::new(1);
    let robj: Robj = extptr.into();
    let extptr2: &ExternalPtr<i32> = robj.try_into().unwrap();
    extptr2.clone()
}

#[extendr]
fn create_numeric_externalptr(x: Doubles) -> ExternalPtr<Doubles> {
    ExternalPtr::new(x)
}

#[extendr]
fn sum_integer_externalptr(x: ExternalPtr<Integers>) -> Rint {
    x.into_iter().sum()
}

mod submod {
    use super::*;

    #[extendr]
    impl Wrapper {
        pub fn a_10(&self) -> i32 {
            self.a + 10
        }
    }
    extendr_module! {
      mod submod;
      impl Wrapper;
    }
}

// Macro to generate exports
extendr_module! {
    mod externalptr;
    impl Wrapper;
    use submod;
    fn create_numeric_externalptr;
    fn sum_integer_externalptr;
}
