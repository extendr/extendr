/*!
Represent a Rust collection as a [vctrs](https://vctrs.r-lib.org) vector in R.

Enable with the `vctrs` feature. Implement [`Rvctr`] for a type, mark it with
`#[extendr(vctrs)]` and register it with `vctr TypeName;` in
[`extendr_module!`](crate::extendr_module):

```ignore
use extendr_api::prelude::*;

#[derive(Debug, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
#[extendr(vctrs)]
pub struct Points(Vec<Option<Point>>);

impl Rvctr for Points {
    fn length(&self) -> usize {
        self.0.len()
    }

    fn format(&self) -> Strings {
        self.0
            .iter()
            .map(|p| match p {
                Some(p) => format!("({}, {})", p.x, p.y),
                None => "NA".to_string(),
            })
            .collect()
    }

    fn subset(&self, idx: &[Option<usize>]) -> Self {
        Points(Rvctr::subset(&self.0, idx))
    }

    fn combine(&mut self, other: &Self) {
        self.0.extend_from_slice(&other.0)
    }

    fn is_na(&self, i: usize) -> bool {
        self.0[i].is_none()
    }
}

#[extendr]
fn points(x: &[f64], y: &[f64]) -> Points {
    Points(x.iter().zip(y).map(|(&x, &y)| Some(Point { x, y })).collect())
}

extendr_module! {
    mod mypkg;
    fn points;
    vctr Points;
}
```

In R, the object is an integer vector of class `c("Points", "extendr_vctr", "vctrs_vctr")`
whose `extendr_ptr` attribute holds the Rust value. Each element stores its
1-based position, or `NA` where [`Rvctr::is_na`] is `true`, so `length()` and
`is.na()` work without calling into Rust.

The generated R wrappers register these S3 methods for the class:

- `format()` and `as.character()`, which call [`Rvctr::format`]
- `vctrs::vec_restore()`, which calls [`Rvctr::subset`]; this makes `[`,
  `[[`, `rev()`, `rep()`, `head()`, `vctrs::vec_slice()` and data frame
  subsetting work
- `c()`, which calls [`Rvctr::combine`]

The package that uses this feature must list vctrs in `Imports`. Because
values from different objects live behind different pointers,
`vctrs::vec_c()` reports the two inputs as incompatible; use `c()` instead.
*/

use crate::scalar::Rint;
use crate::*;

/// Name of the attribute that holds the external pointer.
pub const EXTENDR_PTR: &str = "extendr_ptr";

/// A Rust collection that can be represented as a vctrs vector in R.
///
/// Every R operation creates a new value, so the methods take `&self`.
/// Implementations must keep [`Rvctr::format`] and [`Rvctr::subset`]
/// consistent with [`Rvctr::length`].
pub trait Rvctr: Clone + 'static {
    /// Number of elements.
    fn length(&self) -> usize;

    /// One string per element, used by `format()` and `print()`.
    fn format(&self) -> Strings;

    /// Select elements by 0-based position. `None` selects a missing element.
    fn subset(&self, idx: &[Option<usize>]) -> Self;

    /// Append the elements of `other`.
    fn combine(&mut self, other: &Self);

    /// Whether the element at `i` is missing.
    fn is_na(&self, i: usize) -> bool {
        let _ = i;
        false
    }
}

impl<T: Clone + std::fmt::Debug + 'static> Rvctr for Vec<Option<T>> {
    fn length(&self) -> usize {
        self.len()
    }

    fn format(&self) -> Strings {
        let mut out = Strings::new(self.len());
        for (i, x) in self.iter().enumerate() {
            let elt = match x {
                Some(x) => Rstr::from(format!("{x:?}")),
                None => Rstr::na(),
            };
            out.set_elt(i, elt);
        }
        out
    }

    fn subset(&self, idx: &[Option<usize>]) -> Self {
        idx.iter()
            .map(|i| i.and_then(|i| self.get(i).cloned().flatten()))
            .collect()
    }

    fn combine(&mut self, other: &Self) {
        self.extend_from_slice(other)
    }

    fn is_na(&self, i: usize) -> bool {
        !matches!(self.get(i), Some(Some(_)))
    }
}

/// Wrap `value` in a vctrs vector of class `class`.
#[doc(hidden)]
pub fn into_vctr<T: Rvctr>(value: T, class: &str) -> Robj {
    let n = value.length();
    let positions = (0..n).map(|i| {
        if value.is_na(i) {
            Rint::na()
        } else {
            Rint::from(i as i32 + 1)
        }
    });
    let mut data: Robj = Integers::from_values(positions).into();
    let ptr = ExternalPtr::new(value);
    data.set_attrib(EXTENDR_PTR, ptr)
        .and_then(|x| x.set_class([class, "extendr_vctr", "vctrs_vctr"]))
        .map(|x| x.clone())
        .unwrap_or_else(|e| panic!("failed to build vctr of class `{class}`: {e}"))
}

/// Borrow the Rust value behind a vctrs vector.
#[doc(hidden)]
pub fn try_vctr_ref<T: Rvctr>(robj: &Robj) -> Result<&T> {
    let ptr = robj
        .get_attrib(EXTENDR_PTR)
        .ok_or_else(|| Error::ExpectedExternalPtr(robj.clone()))?;
    let ptr: &ExternalPtr<T> = (&ptr).try_into()?;
    let value: *const T = ptr.try_addr()?;
    // SAFETY: the external pointer is an attribute of `robj`, so the value
    // lives at least as long as `robj` is borrowed.
    Ok(unsafe { &*value })
}

/// Implementation of `vctrs::vec_restore()`: `x` holds the positions
/// selected from `to`.
#[doc(hidden)]
pub fn restore<T: Rvctr>(x: &Integers, to: &T) -> T {
    let n = to.length();
    let idx: Vec<Option<usize>> = x
        .iter()
        .map(|i| {
            if i.is_na() || i.0 < 1 || i.0 as usize > n {
                None
            } else {
                Some(i.0 as usize - 1)
            }
        })
        .collect();
    to.subset(&idx)
}

/// Implementation of `c()`: concatenate every non-`NULL` element of `xs`.
#[doc(hidden)]
pub fn concat<T: Rvctr>(xs: &List) -> Result<Option<T>> {
    let mut out: Option<T> = None;
    for (i, x) in xs.values().enumerate() {
        if x.is_null() {
            continue;
        }
        let x = try_vctr_ref::<T>(&x)
            .map_err(|e| Error::Other(format!("can't combine argument {}: {e}", i + 1)))?;
        match out.as_mut() {
            Some(out) => out.combine(x),
            None => out = Some(x.clone()),
        }
    }
    Ok(out)
}
