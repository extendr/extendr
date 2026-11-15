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

/// A generic wrapper for `Vec<Option<T>>` that can be used for foreign data types
#[derive(Debug, Clone)]
pub struct Vctr<T: Rvctr> {
    pub(crate) inner: Integers,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Rvctr + 'static> TryFrom<&Robj> for Vctr<T> {
    type Error = Error;

    fn try_from(value: &Robj) -> Result<Self> {
        let inner = Integers::try_from(value)?;

        let ptr = match inner.get_attrib("extendr_ptr") {
            Some(ptr) => ptr,
            None => return Err(Error::ExpectedExternalPtr(value.clone())),
        };

        let _ = ExternalPtr::<T>::try_from(&ptr)?;

        Ok(Vctr {
            inner,
            phantom: std::marker::PhantomData,
        })
    }
}

impl<T: Rvctr + 'static> TryFrom<Robj> for Vctr<T> {
    type Error = Error;

    fn try_from(value: Robj) -> Result<Self> {
        Self::try_from(&value)
    }
}

impl<T: Rvctr + 'static> Vctr<T> {
    pub fn as_vctr(&self) -> Robj {
        let mut x = self.inner.clone();
        let t_class = T::class();
        let res = if t_class == "extendr_vctr" {
            x.set_class([t_class])
        } else {
            x.set_class([t_class, "extendr_vctr"])
        };
        res.map(|x| x.clone().into_robj())
            .unwrap_or_else(|e| panic!("failed to set class `{t_class}`: {e}"))
    }

    pub fn try_into_inner(&self) -> Result<ExternalPtr<T>> {
        let ptr_attrib = self
            .inner
            .get_attrib("extendr_ptr")
            .ok_or_else(|| Error::ExpectedExternalPtr(self.inner.clone().into()))?;

        ExternalPtr::<T>::try_from(&ptr_attrib)
    }

    pub fn format(&self) -> Result<Strings> {
        let inner = self.try_into_inner()?;
        Ok(inner.format())
    }

    fn format_robj(x: &Robj) -> Result<Strings> {
        Vctr::<T>::try_from(x)?.format()
    }
}

impl<T: Rvctr + 'static> From<T> for Vctr<T> {
    fn from(value: T) -> Self {
        let n = value.length().0;
        let ptr = ExternalPtr::new(value);
        let mut inner = Integers::from_values((1..n + 1).map(Rint::from));
        inner
            .set_attrib("extendr_ptr", ptr)
            .unwrap_or_else(|e| panic!("failed to set `extendr_ptr`: {e}"));
        let format_fn = ExternalPtr::<fn(&Robj) -> Result<Strings>>::new(Vctr::<T>::format_robj);
        inner
            .set_attrib("extendr_format", format_fn)
            .unwrap_or_else(|e| panic!("failed to set `extendr_format`: {e}"));
        Vctr {
            inner,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Rvctr + 'static> From<Vctr<T>> for Robj {
    fn from(value: Vctr<T>) -> Self {
        value.as_vctr()
    }
}

pub(crate) mod extendr_vctr {
    use crate as extendr_api;
    use crate::*;

    /// @export
    #[extendr_macros::extendr(r_name = "format.extendr_vctr")]
    fn format_extendr_vctr(x: Robj) -> Result<Strings> {
        let format_fn = x
            .get_attrib("extendr_format")
            .ok_or_else(|| Error::ExpectedExternalPtr(x.clone()))?;
        let format_fn = <&ExternalPtr<fn(&Robj) -> Result<Strings>>>::try_from(&format_fn)?;
        format_fn.try_addr()?(&x)
    }

    /// @export
    #[extendr_macros::extendr(r_name = "print.extendr_vctr", invisible)]
    fn print_extendr_vctr(x: Robj) -> Result<Robj> {
        let class = x
            .class()
            .and_then(|mut c| c.next())
            .unwrap_or("extendr_vctr");
        rprintln!("<{class}[{}]>", x.len());
        let formatted = extendr_macros::call!("format", x.clone())?;
        extendr_macros::call!("print", formatted, quote = false)?;
        Ok(x)
    }

    extendr_macros::extendr_module! {
        mod extendr_vctr;
        fn format_extendr_vctr;
        fn print_extendr_vctr;
    }
}
