use extendr_api::prelude::*;
use rstest::rstest;

// Tests without NA do not require `test!` macro

#[rstest]
#[case(Rfloat::from(2.0), Rfloat::from(1.0))]
#[case(Rint::from(2), Rint::from(1))]
#[case(Rbool::from(true), Rbool::from(false))]
fn left_gt_right<T, U>(#[case] left: T, #[case] right: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    assert!(left > right);
}

#[rstest]
#[case(Rfloat::from(2.0), Rfloat::from(1.0))]
#[case(Rfloat::from(2.0), Rfloat::from(2.0))]
#[case(Rint::from(2), Rint::from(1))]
#[case(Rint::from(2), Rint::from(2))]
#[case(Rbool::from(true), Rbool::from(true))]
#[case(Rbool::from(false), Rbool::from(false))]
fn left_gte_right<T, U>(#[case] left: T, #[case] right: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    assert!(left >= right);
}

#[rstest]
#[case(Rfloat::from(1.0), Rfloat::from(2.0))]
#[case(Rint::from(1), Rint::from(2))]
#[case(Rbool::from(false), Rbool::from(true))]
fn left_lt_right<T, U>(#[case] left: T, #[case] right: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    assert!(left < right);
}

#[rstest]
#[case(Rfloat::from(1.0), Rfloat::from(2.0))]
#[case(Rfloat::from(2.0), Rfloat::from(2.0))]
#[case(Rint::from(1), Rint::from(2))]
#[case(Rint::from(2), Rint::from(2))]
#[case(Rbool::from(true), Rbool::from(true))]
#[case(Rbool::from(false), Rbool::from(false))]
fn left_lte_right<T, U>(#[case] left: T, #[case] right: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    assert!(left <= right);
}

#[rstest]
#[case(Rfloat::from(2.0), Rfloat::from(2.0))]
#[case(Rint::from(2), Rint::from(2))]
#[case(Rbool::from(true), Rbool::from(true))]
#[case(Rbool::from(false), Rbool::from(false))]
fn left_eq_right<T, U>(#[case] left: T, #[case] right: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    assert!(left == right);
    assert!(right == left);
}

#[rstest]
#[case(Rfloat::from(1.0), Rfloat::from(2.0))]
#[case(Rint::from(1), Rint::from(2))]
#[case(Rbool::from(true), Rbool::from(false))]
fn left_neq_right<T, U>(#[case] left: T, #[case] right: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    assert!(left != right);
    assert!(right != left);
}

// `NA` should be created in `test!` macro block

#[rstest]
#[case(Rfloat::from(1.0))]
#[case(Rint::from(1))]
#[case(Rbool::from(true))]
fn left_gt_or_gte_right_na<T, U>(#[case] left: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    test! {
        let right = T::na();
        assert_eq!(left > right, false);
        assert_eq!(left >= right, false);
    }
}

#[rstest]
#[case(Rfloat::from(1.0))]
#[case(Rint::from(1))]
#[case(Rbool::from(true))]
fn left_lt_or_lte_right_na<T, U>(#[case] left: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    test! {
        let right = T::na();
        assert_eq!(left < right, false);
        assert_eq!(left <= right, false);
    }
}

#[rstest]
#[case(Rfloat::from(1.0))]
#[case(Rint::from(1))]
#[case(Rbool::from(true))]
fn left_na_lt_or_lte_right<T, U>(#[case] right: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    test! {
        let left = T::na();
        assert_eq!(left < right, false);
        assert_eq!(left <= right, false);
    }
}

#[rstest]
#[case(Rfloat::from(1.0))]
#[case(Rint::from(1))]
#[case(Rbool::from(true))]
fn left_na_gt_or_gte_right<T, U>(#[case] right: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    test! {
        let left = T::na();
        assert_eq!(left > right, false);
        assert_eq!(left >= right, false);
    }
}

#[rstest]
#[case(Rfloat::from(1.0))]
#[case(Rint::from(1))]
#[case(Rbool::from(true))]
#[case(Rbool::from(false))]
fn na_vs_value<T, U>(#[case] value: T)
where
    T: Scalar<U> + PartialOrd + PartialEq + Copy,
    U: PartialEq + Copy,
{
    test! {
        let na = T::na();
        assert_eq!(value.partial_cmp(&na), None);
        assert_eq!(na.partial_cmp(&value), None);
        assert_eq!(na.partial_cmp(&na), None);
    }
}
