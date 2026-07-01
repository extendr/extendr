use extendr_api::prelude::*;

#[test]
fn test_rint() {
    let a = RInt::from(20);
    let b = RInt::from(10);
    assert_eq!(a + b, RInt::from(30));
    assert_eq!(a - b, RInt::from(10));
    assert_eq!(a * b, RInt::from(200));
    assert_eq!(a / b, RInt::from(2));
    assert_eq!(-a, RInt::from(-20));
    assert_eq!(!a, RInt::from(-21));

    assert_eq!(a + b, RInt::from(30));
    assert_eq!(a - b, RInt::from(10));
    assert_eq!(a * b, RInt::from(200));
    assert_eq!(a / b, RInt::from(2));
    assert_eq!(-&a, RInt::from(-20));
    assert_eq!(!&a, RInt::from(-21));

    assert!(RInt::na().is_na());

    // NA lhs
    let a = RInt::na();
    let b = RInt::from(10);
    assert!((a + b).is_na());
    assert!((a - b).is_na());
    assert!((a * b).is_na());
    assert!((a / b).is_na());
    assert!((-a).is_na());
    assert!((!a).is_na());

    // NA rhs
    let a = RInt::from(10);
    let b = RInt::na();
    assert!((a + b).is_na());
    assert!((a - b).is_na());
    assert!((a * b).is_na());
    assert!((a / b).is_na());

    // Overflow
    let a = RInt::from(i32::MAX - 1);
    let b = RInt::from(10);
    assert!((a * b).is_na());
    assert!((RInt::from(1) / RInt::from(0)).is_na());
    assert!((RInt::from(-1) / RInt::na()).is_na());

    // Underflow
    let a = RInt::from(i32::MIN + 1);
    let b = RInt::from(-10);
    assert!((a + b).is_na());
}

#[test]
fn test_rint_opassign() {
    // LHS RInt, RHS RInt
    let mut a = RInt::from(20);
    a += RInt::from(10);
    assert_eq!(a, RInt::from(30));
    a -= RInt::from(20);
    assert_eq!(a, RInt::from(10));
    a *= RInt::from(20);
    assert_eq!(a, RInt::from(200));
    a /= RInt::from(100);
    assert_eq!(a, RInt::from(2));

    // LHS &mut RInt, RHS RInt
    let mut a = RInt::from(20);
    let mut b = &mut a;
    b += RInt::from(10);
    assert_eq!(b, &RInt::from(30));
    b -= RInt::from(20);
    assert_eq!(b, &RInt::from(10));
    b *= RInt::from(20);
    assert_eq!(b, &RInt::from(200));
    b /= RInt::from(100);
    assert_eq!(b, &RInt::from(2));

    // LHS RInt, RHS i32
    let mut a = RInt::from(20);
    a += 10;
    assert_eq!(a, RInt::from(30));
    a -= 20;
    assert_eq!(a, RInt::from(10));
    a *= 20;
    assert_eq!(a, RInt::from(200));
    a /= 100;
    assert_eq!(a, RInt::from(2));

    // LHS &mut RInt, RHS i32
    let mut a = RInt::from(20);
    let mut b = &mut a;
    b += 10;
    assert_eq!(b, &RInt::from(30));
    b -= 20;
    assert_eq!(b, &RInt::from(10));
    b *= 20;
    assert_eq!(b, &RInt::from(200));
    b /= 100;
    assert_eq!(b, &RInt::from(2));

    // LHS Option<i32>, RHS RInt
    let mut a = Some(20);
    a += RInt::from(10);
    assert_eq!(a, Some(30));
    a -= RInt::from(20);
    assert_eq!(a, Some(10));
    a *= RInt::from(20);
    assert_eq!(a, Some(200));
    a /= RInt::from(100);
    assert_eq!(a, Some(2));

    // LHS NA
    let mut a = RInt::na();
    a += RInt::from(10);
    assert!(a.is_na());
    a -= RInt::from(20);
    assert!(a.is_na());
    a *= RInt::from(20);
    assert!(a.is_na());
    a /= RInt::from(100);
    assert!(a.is_na());

    // RHS NA
    let mut a = RInt::from(20);
    a += RInt::na();
    assert!(a.is_na());
    let mut a = RInt::from(20);
    a -= RInt::na();
    assert!(a.is_na());
    let mut a = RInt::from(20);
    a *= RInt::na();
    assert!(a.is_na());
    let mut a = RInt::from(20);
    a /= RInt::na();
    assert!(a.is_na());

    // Overflow | LHS RInt, RHS RInt
    let mut a = RInt::from(i32::MAX - 1);
    a += RInt::from(10);
    assert!(a.is_na());
    let mut a = RInt::from(i32::MAX - 1);
    a *= RInt::from(10);
    assert!(a.is_na());

    let mut a = RInt::from(1);
    a /= RInt::from(0);
    assert!(a.is_na());
    let mut a = RInt::from(-1);
    a /= RInt::na();
    assert!(a.is_na());

    // Underflow | LHS RInt, RHS RInt
    let mut a = RInt::from(i32::MIN + 1);
    a += RInt::from(-10);
    assert!(a.is_na());
}

#[test]
fn test_rfloat() {
    test! {
        let a = RFloat::from(20.);
        let b = RFloat::from(10.);
        assert_eq!(a + b, RFloat::from(30.));
        assert_eq!(a - b, RFloat::from(10.));
        assert_eq!(a * b, RFloat::from(200.));
        assert_eq!(a / b, RFloat::from(2.));
        assert_eq!(-a, RFloat::from(-20.));

        assert_eq!(a + b, RFloat::from(30.));
        assert_eq!(a - b, RFloat::from(10.));
        assert_eq!(a * b, RFloat::from(200.));
        assert_eq!(a / b, RFloat::from(2.));
        assert_eq!(-&a, RFloat::from(-20.));

        assert!(RFloat::na().is_na());

        // NA lhs
        let a = RFloat::na();
        let b = RFloat::from(10.);
        assert!((a + b).is_na());
        assert!((a - b).is_na());
        assert!((a * b).is_na());
        assert!((a / b).is_na());
        assert!((-a).is_na());

        // NA rhs
        let a = RFloat::from(10.);
        let b = RFloat::na();
        assert!((a + b).is_na());
        assert!((a - b).is_na());
        assert!((a * b).is_na());
        assert!((a / b).is_na());

        // Inf is a single value, so can be tested for equality
        let a = RFloat::from(f64::INFINITY);
        let b = RFloat::from(42.);
        assert_eq!(a + b, a);
        assert_eq!(a - b, a);
        assert_eq!(b - a, RFloat::from(f64::NEG_INFINITY));
        assert_eq!(a * b, a);
        assert_eq!(a / b, a);
        assert_eq!(-a, RFloat::from(f64::NEG_INFINITY));

        let a = RFloat::from(f64::NEG_INFINITY);
        assert_eq!(a + b, a);
        assert_eq!(a - b, a);
        assert_eq!(b - a, RFloat::from(f64::INFINITY));
        assert_eq!(a * b, a);
        assert_eq!(a / b, a);
        assert_eq!(-a, RFloat::from(f64::INFINITY));

        // Operations with NaN produce NaN
        let a = RFloat::from(f64::NAN);
        assert!((a + b).is_nan());
        assert!((a - b).is_nan());
        assert!((a * b).is_nan());
        assert!((a / b).is_nan());
        assert!((-a).is_nan());

        // Signs
        assert!(RFloat::from(0.).is_sign_positive());
        assert!(RFloat::from(f64::INFINITY).is_sign_positive());

        assert!(RFloat::from(-0.).is_sign_negative());
        assert!(RFloat::from(f64::NEG_INFINITY).is_sign_negative());

        // Infinity
        assert!(RFloat::from(f64::INFINITY).is_infinite());
        assert!(RFloat::from(f64::NEG_INFINITY).is_infinite());
        assert!(!RFloat::from(0.).is_infinite());

        // Some more, testing mixed binary operators
        assert!((RFloat::from(f64::INFINITY) + 1.).is_infinite());
        assert!((42. - RFloat::from(f64::INFINITY)).is_sign_negative());

        // Absolute value
        assert_eq!(RFloat::from(-42.).abs(), RFloat::from(42.));
        assert_eq!(RFloat::from(42.).abs(), RFloat::from(42.));
        assert_eq!(RFloat::from(0.).abs(), RFloat::from(0.));
    }
}

#[test]
#[cfg(feature = "num-complex")]
fn test_rcplx() {
    test! {
        let a = RCplx::new(20., 300.);
        let b = RCplx::new(10., 400.);
        assert_eq!(a + b, RCplx::new(30., 700.));
        assert_eq!(a - b, RCplx::new(10., -100.));
        assert_eq!(a * b, RCplx::new(-119800.0, 11000.0));

        let a = RCplx::from(20.);
        let b = RCplx::from(10.);
        assert_eq!(a / b, RCplx::from(2.));
        assert_eq!(-a, RCplx::from(-20.));

        assert_eq!(a + b, RCplx::from(30.));
        assert_eq!(a - b, RCplx::from(10.));
        assert_eq!(a * b, RCplx::from(200.));
        assert_eq!(a / b, RCplx::from(2.));
        assert_eq!(-a, RCplx::from(-20.));

        assert!(RCplx::na().is_na());

        // NA lhs
        let a = RCplx::na();
        let b = RCplx::from(10.);
        assert!((a + b).is_na());
        assert!((a - b).is_na());
        assert!((a * b).is_na());
        assert!((a / b).is_na());
        assert!((-a).is_na());

        // NA rhs
        let a = RCplx::from(10.);
        let b = RCplx::na();
        assert!((a + b).is_na());
        assert!((a - b).is_na());
        assert!((a * b).is_na());
        assert!((a / b).is_na());

        // Inf is a single value, so can be tested for equality
        let a = RCplx::from(f64::INFINITY);
        let b = RCplx::from(42.);
        assert_eq!(a + b, a);
        assert_eq!(a - b, a);
        assert_eq!(b - a, RCplx::from(f64::NEG_INFINITY));
        // assert_eq!(a * b, a);
        // assert_eq!(a / b, a);
        assert_eq!(-a, RCplx::from(f64::NEG_INFINITY));

        let a = RCplx::from(f64::NEG_INFINITY);
        assert_eq!(a + b, a);
        assert_eq!(a - b, a);
        assert_eq!(b - a, RCplx::from(f64::INFINITY));
        // assert_eq!(a * b, a);
        // assert_eq!(a / b, a);
        assert_eq!(-a, RCplx::from(f64::INFINITY));

        // Operations with NaN produce NaN
        let a = RCplx::from(f64::NAN);
        assert!((a + b).is_nan());
        assert!((a - b).is_nan());
        assert!((a * b).is_nan());
        assert!((a / b).is_nan());
        assert!((-a).is_nan());

        // Infinity
        assert!(RCplx::from(f64::INFINITY).is_infinite());
        assert!(RCplx::from(f64::NEG_INFINITY).is_infinite());
        assert!(!RCplx::from(0.).is_infinite());

        // Some more, testing mixed binary operators
        assert!((RCplx::from(f64::INFINITY) + RCplx::from(1.)).is_infinite());
    }
}

#[test]
fn test_rfloat_opassign() {
    test! {
        // LHS RFloat, RHS RFloat
        let mut a = RFloat::from(20.);
        a += RFloat::from(10.);
        assert_eq!(a, RFloat::from(30.));
        a -= RFloat::from(20.);
        assert_eq!(a, RFloat::from(10.));
        a *= RFloat::from(20.);
        assert_eq!(a, RFloat::from(200.));
        a /= RFloat::from(100.);
        assert_eq!(a, RFloat::from(2.));

        // LHS &mut RFloat, RHS RFloat
        let mut a = RFloat::from(20.);
        let mut b = &mut a;
        b += RFloat::from(10.);
        assert_eq!(b, &RFloat::from(30.));
        b -= RFloat::from(20.);
        assert_eq!(b, &RFloat::from(10.));
        b *= RFloat::from(20.);
        assert_eq!(b, &RFloat::from(200.));
        b /= RFloat::from(100.);
        assert_eq!(b, &RFloat::from(2.));

        // LHS RFloat, RHS f64
        let mut a = RFloat::from(20.);
        a += 10.;
        assert_eq!(a, RFloat::from(30.));
        a -= 20.;
        assert_eq!(a, RFloat::from(10.));
        a *= 20.;
        assert_eq!(a, RFloat::from(200.));
        a /= 100.;
        assert_eq!(a, RFloat::from(2.));

        // LHS &mut RFloat, RHS f64
        let mut a = RFloat::from(20.);
        let mut b = &mut a;
        b += 10.;
        assert_eq!(b, &RFloat::from(30.));
        b -= 20.;
        assert_eq!(b, &RFloat::from(10.));
        b *= 20.;
        assert_eq!(b, &RFloat::from(200.));
        b /= 100.;
        assert_eq!(b, &RFloat::from(2.));

        // LHS Option<f64>, RHS RFloat
        let mut a = Some(20.);
        a += RFloat::from(10.);
        assert_eq!(a, Some(30.));
        a -= RFloat::from(20.);
        assert_eq!(a, Some(10.));
        a *= RFloat::from(20.);
        assert_eq!(a, Some(200.));
        a /= RFloat::from(100.);
        assert_eq!(a, Some(2.));

        // LHS NA
        let mut a = RFloat::na();
        a += RFloat::from(10.);
        assert!(a.is_na());
        a -= RFloat::from(20.);
        assert!(a.is_na());
        a *= RFloat::from(20.);
        assert!(a.is_na());
        a /= RFloat::from(100.);
        assert!(a.is_na());

        // RHS NA
        let mut a = RFloat::from(20.);
        a += RFloat::na();
        assert!(a.is_na());
        let mut a = RFloat::from(20.);
        a -= RFloat::na();
        assert!(a.is_na());
        let mut a = RFloat::from(20.);
        a *= RFloat::na();
        assert!(a.is_na());
        let mut a = RFloat::from(20.);
        a /= RFloat::na();
        assert!(a.is_na());

        // Inf is a single value, so can be tested for equality
        let mut a = RFloat::from(f64::INFINITY);
        let mut b = RFloat::from(42.);
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

        let mut a = RFloat::from(f64::NEG_INFINITY);
        let mut b = RFloat::from(42.);
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
        let mut a = RFloat::from(f64::NAN);
        let mut b = RFloat::from(42.);
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
