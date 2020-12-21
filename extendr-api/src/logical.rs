
use super::integer_is_na;
/// Bool is a wrapper for i32 in the context of an R's tristate boolean.
#[derive(Debug)]
pub struct Bool(pub i32);

impl Bool {
    // Convert this Bool to a bool. Note NA will be true.
    pub fn to_bool(&self) -> bool {
        self.0 != 0
    }

    // Convert this construct a Bool from a rust boolean.
    pub fn from_bool(val: bool) -> Self {
        Bool(val as i32)
    }

    // Return true if this Bool is NA.
    pub fn is_na(&self) -> bool {
        integer_is_na(self.0)
    }

    // Construct a NA Bool.
    pub fn na() -> Bool {
        Bool(std::i32::MIN)
    }
}

impl Clone for Bool {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Copy for Bool {}

impl From<i32> for Bool {
    fn from(v: i32) -> Self {
        Self(v)
    }
}

impl From<bool> for Bool {
    fn from(v: bool) -> Self {
        Self(v as i32)
    }
}

impl From<Bool> for bool {
    fn from(v: Bool) -> Self {
        v.0 != 0
    }
}

impl From<&Bool> for bool {
    fn from(v: &Bool) -> Self {
        v.0 != 0
    }
}

impl PartialEq<Bool> for Bool {
    fn eq(&self, rhs: &Bool) -> bool {
        self.0 == rhs.0
    }
}
