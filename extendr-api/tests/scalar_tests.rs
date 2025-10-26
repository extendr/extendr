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

    assert_eq!(a + b, Rint::from(30));
    assert_eq!(a - b, Rint::from(10));
    assert_eq!(a * b, Rint::from(200));
    assert_eq!(a / b, Rint::from(2));
    assert_eq!(-&a, Rint::from(-20));
    assert_eq!(!&a, Rint::from(-21));

    assert!(Rint::na().is_na());

    // NA lhs
    let a = Rint::na();
    let b = Rint::from(10);
    assert!((a + b).is_na());
    assert!((a - b).is_na());
    assert!((a * b).is_na());
    assert!((a / b).is_na());
    assert!((-a).is_na());
    assert!((!a).is_na());

    // NA rhs
    let a = Rint::from(10);
    let b = Rint::na();
    assert!((a + b).is_na());
    assert!((a - b).is_na());
    assert!((a * b).is_na());
    assert!((a / b).is_na());

    // Overflow
    let a = Rint::from(i32::MAX - 1);
    let b = Rint::from(10);
    assert!((a * b).is_na());
    assert!((Rint::from(1) / Rint::from(0)).is_na());
    assert!((Rint::from(-1) / Rint::na()).is_na());

    // Underflow
    let a = Rint::from(i32::MIN + 1);
    let b = Rint::from(-10);
    assert!((a + b).is_na());
}

#[test]
fn test_rint_opassign() {
    // LHS Rint, RHS Rint
    let mut a = Rint::from(20);
    a += Rint::from(10);
    assert_eq!(a, Rint::from(30));
    a -= Rint::from(20);
    assert_eq!(a, Rint::from(10));
    a *= Rint::from(20);
    assert_eq!(a, Rint::from(200));
    a /= Rint::from(100);
    assert_eq!(a, Rint::from(2));

    // LHS &mut Rint, RHS Rint
    let mut a = Rint::from(20);
    let mut b = &mut a;
    b += Rint::from(10);
    assert_eq!(b, &Rint::from(30));
    b -= Rint::from(20);
    assert_eq!(b, &Rint::from(10));
    b *= Rint::from(20);
    assert_eq!(b, &Rint::from(200));
    b /= Rint::from(100);
    assert_eq!(b, &Rint::from(2));

    // LHS Rint, RHS i32
    let mut a = Rint::from(20);
    a += 10;
    assert_eq!(a, Rint::from(30));
    a -= 20;
    assert_eq!(a, Rint::from(10));
    a *= 20;
    assert_eq!(a, Rint::from(200));
    a /= 100;
    assert_eq!(a, Rint::from(2));

    // LHS &mut Rint, RHS i32
    let mut a = Rint::from(20);
    let mut b = &mut a;
    b += 10;
    assert_eq!(b, &Rint::from(30));
    b -= 20;
    assert_eq!(b, &Rint::from(10));
    b *= 20;
    assert_eq!(b, &Rint::from(200));
    b /= 100;
    assert_eq!(b, &Rint::from(2));

    // LHS Option<i32>, RHS Rint
    let mut a = Some(20);
    a += Rint::from(10);
    assert_eq!(a, Some(30));
    a -= Rint::from(20);
    assert_eq!(a, Some(10));
    a *= Rint::from(20);
    assert_eq!(a, Some(200));
    a /= Rint::from(100);
    assert_eq!(a, Some(2));

    // LHS NA
    let mut a = Rint::na();
    a += Rint::from(10);
    assert!(a.is_na());
    a -= Rint::from(20);
    assert!(a.is_na());
    a *= Rint::from(20);
    assert!(a.is_na());
    a /= Rint::from(100);
    assert!(a.is_na());

    // RHS NA
    let mut a = Rint::from(20);
    a += Rint::na();
    assert!(a.is_na());
    let mut a = Rint::from(20);
    a -= Rint::na();
    assert!(a.is_na());
    let mut a = Rint::from(20);
    a *= Rint::na();
    assert!(a.is_na());
    let mut a = Rint::from(20);
    a /= Rint::na();
    assert!(a.is_na());

    // Overflow | LHS Rint, RHS Rint
    let mut a = Rint::from(i32::MAX - 1);
    a += Rint::from(10);
    assert!(a.is_na());
    let mut a = Rint::from(i32::MAX - 1);
    a *= Rint::from(10);
    assert!(a.is_na());

    let mut a = Rint::from(1);
    a /= Rint::from(0);
    assert!(a.is_na());
    let mut a = Rint::from(-1);
    a /= Rint::na();
    assert!(a.is_na());

    // Underflow | LHS Rint, RHS Rint
    let mut a = Rint::from(i32::MIN + 1);
    a += Rint::from(-10);
    assert!(a.is_na());
}

#[test]
fn test_rfloat() {
    test! {
        let a = Rfloat::from(20.);
        let b = Rfloat::from(10.);
        assert_eq!(a + b, Rfloat::from(30.));
        assert_eq!(a - b, Rfloat::from(10.));
        assert_eq!(a * b, Rfloat::from(200.));
        assert_eq!(a / b, Rfloat::from(2.));
        assert_eq!(-a, Rfloat::from(-20.));

        assert_eq!(a + b, Rfloat::from(30.));
        assert_eq!(a - b, Rfloat::from(10.));
        assert_eq!(a * b, Rfloat::from(200.));
        assert_eq!(a / b, Rfloat::from(2.));
        assert_eq!(-&a, Rfloat::from(-20.));

        assert!(Rfloat::na().is_na());

        // NA lhs
        let a = Rfloat::na();
        let b = Rfloat::from(10.);
        assert!((a + b).is_na());
        assert!((a - b).is_na());
        assert!((a * b).is_na());
        assert!((a / b).is_na());
        assert!((-a).is_na());

        // NA rhs
        let a = Rfloat::from(10.);
        let b = Rfloat::na();
        assert!((a + b).is_na());
        assert!((a - b).is_na());
        assert!((a * b).is_na());
        assert!((a / b).is_na());

        // Inf is a single value, so can be tested for equality
        let a = Rfloat::from(f64::INFINITY);
        let b = Rfloat::from(42.);
        assert_eq!(a + b, a);
        assert_eq!(a - b, a);
        assert_eq!(b - a, Rfloat::from(f64::NEG_INFINITY));
        assert_eq!(a * b, a);
        assert_eq!(a / b, a);
        assert_eq!(-a, Rfloat::from(f64::NEG_INFINITY));

        let a = Rfloat::from(f64::NEG_INFINITY);
        assert_eq!(a + b, a);
        assert_eq!(a - b, a);
        assert_eq!(b - a, Rfloat::from(f64::INFINITY));
        assert_eq!(a * b, a);
        assert_eq!(a / b, a);
        assert_eq!(-a, Rfloat::from(f64::INFINITY));

        // Operations with NaN produce NaN
        let a = Rfloat::from(f64::NAN);
        assert!((a + b).is_nan());
        assert!((a - b).is_nan());
        assert!((a * b).is_nan());
        assert!((a / b).is_nan());
        assert!((-a).is_nan());

        // Signs
        assert!(Rfloat::from(0.).is_sign_positive());
        assert!(Rfloat::from(f64::INFINITY).is_sign_positive());

        assert!(Rfloat::from(-0.).is_sign_negative());
        assert!(Rfloat::from(f64::NEG_INFINITY).is_sign_negative());

        // Infinity
        assert!(Rfloat::from(f64::INFINITY).is_infinite());
        assert!(Rfloat::from(f64::NEG_INFINITY).is_infinite());
        assert!(!Rfloat::from(0.).is_infinite());

        // Some more, testing mixed binary operators
        assert!((Rfloat::from(f64::INFINITY) + 1.).is_infinite());
        assert!((42. - Rfloat::from(f64::INFINITY)).is_sign_negative());

        // Absolute value
        assert_eq!(Rfloat::from(-42.).abs(), Rfloat::from(42.));
        assert_eq!(Rfloat::from(42.).abs(), Rfloat::from(42.));
        assert_eq!(Rfloat::from(0.).abs(), Rfloat::from(0.));
    }
}

#[test]
#[cfg(feature = "num-complex")]
fn test_rcplx() {
    test! {
        let a = Rcplx::from((20., 300.));
        let b = Rcplx::from((10., 400.));
        assert_eq!(a + b, Rcplx::from((30., 700.)));
        assert_eq!(a - b, Rcplx::from((10., -100.)));
        assert_eq!(a * b, Rcplx::from((-119800.0, 11000.0)));

        let a = Rcplx::from(20.);
        let b = Rcplx::from(10.);
        assert_eq!(a / b, Rcplx::from(2.));
        assert_eq!(-a, Rcplx::from(-20.));

        assert_eq!(a + b, Rcplx::from(30.));
        assert_eq!(a - b, Rcplx::from(10.));
        assert_eq!(a * b, Rcplx::from(200.));
        assert_eq!(a / b, Rcplx::from(2.));
        assert_eq!(-a, Rcplx::from(-20.));

        assert!(Rcplx::na().is_na());

        // NA lhs
        let a = Rcplx::na();
        let b = Rcplx::from(10.);
        assert!((a + b).is_na());
        assert!((a - b).is_na());
        assert!((a * b).is_na());
        assert!((a / b).is_na());
        assert!((-a).is_na());

        // NA rhs
        let a = Rcplx::from(10.);
        let b = Rcplx::na();
        assert!((a + b).is_na());
        assert!((a - b).is_na());
        assert!((a * b).is_na());
        assert!((a / b).is_na());

        // Inf is a single value, so can be tested for equality
        let a = Rcplx::from(f64::INFINITY);
        let b = Rcplx::from(42.);
        assert_eq!(a + b, a);
        assert_eq!(a - b, a);
        assert_eq!(b - a, Rcplx::from(f64::NEG_INFINITY));
        // assert_eq!(a * b, a);
        // assert_eq!(a / b, a);
        assert_eq!(-a, Rcplx::from(f64::NEG_INFINITY));

        let a = Rcplx::from(f64::NEG_INFINITY);
        assert_eq!(a + b, a);
        assert_eq!(a - b, a);
        assert_eq!(b - a, Rcplx::from(f64::INFINITY));
        // assert_eq!(a * b, a);
        // assert_eq!(a / b, a);
        assert_eq!(-a, Rcplx::from(f64::INFINITY));

        // Operations with NaN produce NaN
        let a = Rcplx::from(f64::NAN);
        assert!((a + b).is_nan());
        assert!((a - b).is_nan());
        assert!((a * b).is_nan());
        assert!((a / b).is_nan());
        assert!((-a).is_nan());

        // Infinity
        assert!(Rcplx::from(f64::INFINITY).is_infinite());
        assert!(Rcplx::from(f64::NEG_INFINITY).is_infinite());
        assert!(!Rcplx::from(0.).is_infinite());

        // Some more, testing mixed binary operators
        assert!((Rcplx::from(f64::INFINITY) + Rcplx::from(1.)).is_infinite());
    }
}

#[test]
fn test_rfloat_opassign() {
    test! {
        // LHS Rfloat, RHS Rfloat
        let mut a = Rfloat::from(20.);
        a += Rfloat::from(10.);
        assert_eq!(a, Rfloat::from(30.));
        a -= Rfloat::from(20.);
        assert_eq!(a, Rfloat::from(10.));
        a *= Rfloat::from(20.);
        assert_eq!(a, Rfloat::from(200.));
        a /= Rfloat::from(100.);
        assert_eq!(a, Rfloat::from(2.));

        // LHS &mut Rfloat, RHS Rfloat
        let mut a = Rfloat::from(20.);
        let mut b = &mut a;
        b += Rfloat::from(10.);
        assert_eq!(b, &Rfloat::from(30.));
        b -= Rfloat::from(20.);
        assert_eq!(b, &Rfloat::from(10.));
        b *= Rfloat::from(20.);
        assert_eq!(b, &Rfloat::from(200.));
        b /= Rfloat::from(100.);
        assert_eq!(b, &Rfloat::from(2.));

        // LHS Rfloat, RHS f64
        let mut a = Rfloat::from(20.);
        a += 10.;
        assert_eq!(a, Rfloat::from(30.));
        a -= 20.;
        assert_eq!(a, Rfloat::from(10.));
        a *= 20.;
        assert_eq!(a, Rfloat::from(200.));
        a /= 100.;
        assert_eq!(a, Rfloat::from(2.));

        // LHS &mut Rfloat, RHS f64
        let mut a = Rfloat::from(20.);
        let mut b = &mut a;
        b += 10.;
        assert_eq!(b, &Rfloat::from(30.));
        b -= 20.;
        assert_eq!(b, &Rfloat::from(10.));
        b *= 20.;
        assert_eq!(b, &Rfloat::from(200.));
        b /= 100.;
        assert_eq!(b, &Rfloat::from(2.));

        // LHS Option<f64>, RHS Rfloat
        let mut a = Some(20.);
        a += Rfloat::from(10.);
        assert_eq!(a, Some(30.));
        a -= Rfloat::from(20.);
        assert_eq!(a, Some(10.));
        a *= Rfloat::from(20.);
        assert_eq!(a, Some(200.));
        a /= Rfloat::from(100.);
        assert_eq!(a, Some(2.));

        // LHS NA
        let mut a = Rfloat::na();
        a += Rfloat::from(10.);
        assert!(a.is_na());
        a -= Rfloat::from(20.);
        assert!(a.is_na());
        a *= Rfloat::from(20.);
        assert!(a.is_na());
        a /= Rfloat::from(100.);
        assert!(a.is_na());

        // RHS NA
        let mut a = Rfloat::from(20.);
        a += Rfloat::na();
        assert!(a.is_na());
        let mut a = Rfloat::from(20.);
        a -= Rfloat::na();
        assert!(a.is_na());
        let mut a = Rfloat::from(20.);
        a *= Rfloat::na();
        assert!(a.is_na());
        let mut a = Rfloat::from(20.);
        a /= Rfloat::na();
        assert!(a.is_na());

        // Inf is a single value, so can be tested for equality
        let mut a = Rfloat::from(f64::INFINITY);
        let mut b = Rfloat::from(42.);
        a += b;
        assert_eq!(a, f64::INFINITY);
        a -= b;
        assert_eq!(a, f64::INFINITY);
        a *= b;
        assert_eq!(a, f64::INFINITY);
        a /= b;
        assert_eq!(a, f64::INFINITY);
        b -= a;
        assert_eq!(b, f64::NEG_INFINITY);

        let mut a = Rfloat::from(f64::NEG_INFINITY);
        let mut b = Rfloat::from(42.);
        a += b;
        assert_eq!(a, f64::NEG_INFINITY);
        a -= b;
        assert_eq!(a, f64::NEG_INFINITY);
        a *= b;
        assert_eq!(a, f64::NEG_INFINITY);
        a /= b;
        assert_eq!(a, f64::NEG_INFINITY);
        b -= a;
        assert_eq!(b, f64::INFINITY);

        // Operations with NaN produce NaN
        let mut a = Rfloat::from(f64::NAN);
        let mut b = Rfloat::from(42.);
        a += b;
        assert!(a.is_nan());
        a -= b;
        assert!(a.is_nan());
        a *= b;
        assert!(a.is_nan());
        a /= b;
        assert!(a.is_nan());
        b -= a;
        assert!(b.is_nan());
    }
}
