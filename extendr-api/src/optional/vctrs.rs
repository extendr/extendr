use crate::scalar::Rint;
use crate::*;

pub trait Rvctr
where
    Self: Sized + std::fmt::Debug,
{
    fn class() -> &'static str {
        "extendr_vctr"
    }
    fn format(&self) -> Strings;
    fn length(&self) -> Rint;
    fn subset(&self, idx: Integers) -> Self;
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
