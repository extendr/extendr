use crate::scalar::Rint;
use crate::*;

/// Make any `Vec<Option<T>>` into a vector in R.
///
///
/// Can be derived via `#[derive(Rvctr)]`
/// Additionally, there is a blanket `impl` for all `Vec<Option<T>>` for ease of use.
pub trait Rvctr
where
    Self: Sized + std::fmt::Debug,
{
    /// The subclass assigned to the vector
    fn class() -> &'static str {
        "extendr_vctr"
    }

    /// Used in the `print()` method of the created vector
    /// When using `#[derive(Rvctr)]`, the debug diplay method is used.
    fn format(&self) -> Strings;
    /// Used to determine the length of the vector
    fn length(&self) -> Rint;
    /// How to subset the vector—**this is 1-based indexed**
    fn subset(&self, idx: Integers) -> Self;
    /// Used in `c(x, y)` consumes `y` and clones `self`
    fn extend(self, y: Self) -> Self;
}

impl<T: std::fmt::Debug + Clone> Rvctr for Vec<Option<T>> {
    fn class() -> &'static str {
        "extendr_vctr"
    }

    fn format(&self) -> Strings {
        self.iter()
            .map(|xi| match xi {
                Some(xi) => format!("{:?}", xi),
                None => String::from("NA"),
            })
            .collect::<Strings>()
    }

    fn length(&self) -> Rint {
        Rint::from(self.len() as i32)
    }

    fn subset(&self, idx: Integers) -> Self {
        let x_len = self.len();
        idx.iter()
            .map(|i| match i {
                _ if i.is_na() => None,
                _ if i.0 <= 0 || i.0 as usize > x_len => None,
                _ => self.get(i.0 as usize - 1).cloned().unwrap_or(None),
            })
            .collect()
    }

    fn extend(mut self, y: Self) -> Self {
        Extend::extend(&mut self, y);
        self
    }
}
