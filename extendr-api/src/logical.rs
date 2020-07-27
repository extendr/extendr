


/// Bool is a wrapper for i32 in the context of an R boolean.
#[derive(Debug)]
pub struct Bool(pub i32);

impl Clone for Bool {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Copy for Bool {

}

impl From<i32> for Bool {
    fn from(v: i32) -> Self {
        Self(v)
    }
}

impl PartialEq<Bool> for Bool {
    fn eq(&self, rhs: &Bool) -> bool {
        self.0 == rhs.0
    }
}

