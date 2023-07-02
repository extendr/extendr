use extendr_api::prelude::*;
use rstest::rstest;

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
