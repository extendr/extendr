use extendr_api::optional::vctrs::{concat, restore, EXTENDR_PTR};
use extendr_api::prelude::*;

#[derive(Debug, Clone, PartialEq)]
#[extendr(vctrs)]
pub struct Ints(Vec<Option<i32>>);

impl Rvctr for Ints {
    fn length(&self) -> usize {
        self.0.len()
    }

    fn format(&self) -> Strings {
        Rvctr::format(&self.0)
    }

    fn subset(&self, idx: &[Option<usize>]) -> Self {
        Ints(Rvctr::subset(&self.0, idx))
    }

    fn combine(&mut self, other: &Self) {
        self.0.extend_from_slice(&other.0)
    }

    fn is_na(&self, i: usize) -> bool {
        Rvctr::is_na(&self.0, i)
    }
}

#[derive(Debug, Clone)]
#[extendr(vctrs, r_name = "my_ints")]
pub struct Renamed(Vec<Option<i32>>);

impl Rvctr for Renamed {
    fn length(&self) -> usize {
        self.0.len()
    }

    fn format(&self) -> Strings {
        Rvctr::format(&self.0)
    }

    fn subset(&self, idx: &[Option<usize>]) -> Self {
        Renamed(Rvctr::subset(&self.0, idx))
    }

    fn combine(&mut self, other: &Self) {
        self.0.extend_from_slice(&other.0)
    }
}

extendr_module! {
    mod vctrs_test;
    vctr Ints;
    vctr Renamed;
}

#[test]
fn vctr_round_trip() {
    test! {
        let robj: Robj = Ints(vec![Some(1), None, Some(3)]).into();
        assert_eq!(robj.class().map(|c| c.collect::<Vec<_>>()), Some(vec!["Ints", "extendr_vctr", "vctrs_vctr"]));
        assert_eq!(robj.as_integer_slice(), Some([1, i32::MIN, 3].as_slice()));
        assert!(robj.get_attrib(EXTENDR_PTR).is_some());

        let x: &Ints = (&robj).try_into()?;
        assert_eq!(x, &Ints(vec![Some(1), None, Some(3)]));
        let owned: Ints = (&robj).try_into()?;
        assert_eq!(owned, Ints(vec![Some(1), None, Some(3)]));

        let renamed: Robj = Renamed(vec![Some(1)]).into();
        assert_eq!(renamed.class().map(|c| c.collect::<Vec<_>>()), Some(vec!["my_ints", "extendr_vctr", "vctrs_vctr"]));
        assert!(<&Ints>::try_from(&renamed).is_err());
        assert!(<&Ints>::try_from(&r!(1)).is_err());
    }
}

#[test]
fn vctr_restore_and_concat() {
    test! {
        let x = Ints(vec![Some(1), None, Some(3)]);
        let idx = Integers::from_values([Rint::from(3), Rint::na(), Rint::from(1), Rint::from(9)]);
        assert_eq!(restore(&idx, &x), Ints(vec![Some(3), None, Some(1), None]));

        let xs = List::from_values([Robj::from(x.clone()), Robj::from(()), Robj::from(Ints(vec![Some(4)]))]);
        assert_eq!(concat::<Ints>(&xs)?, Some(Ints(vec![Some(1), None, Some(3), Some(4)])));
        assert_eq!(concat::<Ints>(&List::new(0))?, None);
        assert!(concat::<Ints>(&List::from_values([r!(1)])).is_err());
    }
}

#[test]
fn vctr_r_wrappers() {
    test! {
        let wrappers = get_vctrs_test_metadata().make_r_wrappers(true, "pkg").map_err(|e| Error::Other(e.to_string()))?;
        for line in [
            "`format.Ints` <- function(x, ...) .Call(wrap__vctr_format__Ints, x)",
            "`as.character.Ints` <- function(x, ...) format(x, ...)",
            "#' @exportS3Method vctrs::vec_restore\n`vec_restore.Ints` <- function(x, to, ...) .Call(wrap__vctr_restore__Ints, x, to)",
            "`c.Ints` <- function(...) .Call(wrap__vctr_c__Ints, list(...))",
            "`format.my_ints` <- function(x, ...) .Call(wrap__vctr_format__Renamed, x)",
        ] {
            assert!(wrappers.contains(line), "missing:\n{line}\nin:\n{wrappers}");
        }

        let wrappers = get_vctrs_test_metadata().make_r_wrappers(false, "pkg").map_err(|e| Error::Other(e.to_string()))?;
        assert!(wrappers.contains(".Call(\"wrap__vctr_c__Ints\", list(...), PACKAGE = \"pkg\")"));
    }
}
