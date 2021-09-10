use extendr_api::prelude::*;

#[test]
fn test_rint() {
    let a = Rint::from(20);
    let b = Rint::from(10);
    assert_eq!(a + b, Rint::from(30));
    assert_eq!(a - b, Rint::from(10));
    assert_eq!(a * b, Rint::from(200));
    assert_eq!(a / b, Rint::from(2));
    assert_eq!(-a, Rint::from(-20));
    assert_eq!(!a, Rint::from(-21));

    assert_eq!(&a + b, Rint::from(30));
    assert_eq!(&a - b, Rint::from(10));
    assert_eq!(&a * b, Rint::from(200));
    assert_eq!(&a / b, Rint::from(2));
    assert_eq!(-&a, Rint::from(-20));
    assert_eq!(!&a, Rint::from(-21));

    // NA lhs
    let a = Rint::na();
    let b = Rint::from(10);
    assert_eq!(a + b, Rint::na());
    assert_eq!(a - b, Rint::na());
    assert_eq!(a * b, Rint::na());
    assert_eq!(a / b, Rint::na());
    assert_eq!(-a, Rint::na());
    assert_eq!(!a, Rint::na());

    // NA rhs
    let a = Rint::from(10);
    let b = Rint::na();
    assert_eq!(a + b, Rint::na());
    assert_eq!(a - b, Rint::na());
    assert_eq!(a * b, Rint::na());
    assert_eq!(a / b, Rint::na());

    // Overflow
    let a = Rint::from(i32::MAX - 1);
    let b = Rint::from(10);
    assert_eq!(a * b, Rint::na());
    assert_eq!(Rint::from(1) / Rint::from(0), Rint::na());
    assert_eq!(Rint::from(-1) / Rint::na(), Rint::na());

    // Underflow
    let a = Rint::from(i32::MIN + 1);
    let b = Rint::from(-10);
    assert_eq!(a + b, Rint::na());
}

#[test]
fn test_rfloat() {
    let a = Rfloat::from(20.);
    let b = Rfloat::from(10.);
    assert_eq!(a + b, Rfloat::from(30.));
    assert_eq!(a - b, Rfloat::from(10.));
    assert_eq!(a * b, Rfloat::from(200.));
    assert_eq!(a / b, Rfloat::from(2.));
    assert_eq!(-a, Rfloat::from(-20.));

    assert_eq!(&a + b, Rfloat::from(30.));
    assert_eq!(&a - b, Rfloat::from(10.));
    assert_eq!(&a * b, Rfloat::from(200.));
    assert_eq!(&a / b, Rfloat::from(2.));
    assert_eq!(-&a, Rfloat::from(-20.));

    // NA lhs
    let a = Rfloat::na();
    let b = Rfloat::from(10.);
    assert_eq!(a + b, Rfloat::na());
    assert_eq!(a - b, Rfloat::na());
    assert_eq!(a * b, Rfloat::na());
    assert_eq!(a / b, Rfloat::na());
    assert_eq!(-a, Rfloat::na());

    // NA rhs
    let a = Rfloat::from(10.);
    let b = Rfloat::na();
    assert_eq!(a + b, Rfloat::na());
    assert_eq!(a - b, Rfloat::na());
    assert_eq!(a * b, Rfloat::na());
    assert_eq!(a / b, Rfloat::na());

}
