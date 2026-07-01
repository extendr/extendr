use crate as extendr_api;
use crate::conversions::try_into_int::ConversionError;
use crate::scalar::*;
use crate::*;

#[test]
fn test_try_from_robj() {
    test! {
        assert_eq!(<bool>::try_from(&RObj::from(true)), Ok(true));
        assert_eq!(<u8>::try_from(&RObj::from(1)), Ok(1));
        assert_eq!(<u16>::try_from(&RObj::from(1)), Ok(1));
        assert_eq!(<u32>::try_from(&RObj::from(1)), Ok(1));
        assert_eq!(<u64>::try_from(&RObj::from(1)), Ok(1));
        assert_eq!(<i8>::try_from(&RObj::from(1)), Ok(1));
        assert_eq!(<i16>::try_from(&RObj::from(1)), Ok(1));
        assert_eq!(<i32>::try_from(&RObj::from(1)), Ok(1));
        assert_eq!(<i64>::try_from(&RObj::from(1)), Ok(1));
        assert_eq!(<f32>::try_from(&RObj::from(1)), Ok(1.));
        assert_eq!(<f64>::try_from(&RObj::from(1)), Ok(1.));

        assert_eq!(<Vec::<i32>>::try_from(&RObj::from(1)), Ok(vec![1]));
        assert_eq!(<Vec::<f64>>::try_from(&RObj::from(1.)), Ok(vec![1.]));
        assert_eq!(<Vec::<RInt>>::try_from(RObj::from(1)), Ok(vec![RInt::from(1)]));
        assert_eq!(<Vec::<RFloat>>::try_from(RObj::from(1.)), Ok(vec![RFloat::from(1.0)]));
        assert_eq!(<Vec::<RBool>>::try_from(RObj::from(TRUE)), Ok(vec![TRUE]));
        assert_eq!(<Vec::<u8>>::try_from(RObj::from(0_u8)), Ok(vec![0_u8]));

        // conversion from non-integer-ish value to integer should fail
        let robj = RObj::from(1.5);
        assert_eq!(<i32>::try_from(robj.clone()), Err(Error::ExpectedWholeNumber(robj, ConversionError::NotIntegerish)));

        // conversion from out-of-limit value should fail
        let robj = RObj::from(32768);
        assert_eq!(<i16>::try_from(robj.clone()), Err(Error::OutOfLimits(robj)));
        let robj = RObj::from(-1);
        assert_eq!(<u32>::try_from(robj.clone()), Err(Error::OutOfLimits(robj)));

        let hello = RObj::from("hello");
        assert_eq!(<&str>::try_from(&hello), Ok("hello"));
        let hello = RObj::from("hello");
        assert_eq!(<String>::try_from(hello), Ok("hello".into()));

        // conversion from a vector to a scalar value

        let robj = RObj::from(vec![].as_slice() as &[i32]);
        assert_eq!(
            <i32>::try_from(robj.clone()),
            Err(Error::ExpectedNonZeroLength(robj))
        );
        assert!(
            <i32>::try_from(&RObj::from(vec![].as_slice() as &[i32])).is_err()
        );
        assert_eq!(
            <i32>::try_from(&RObj::from(vec![1].as_slice() as &[i32])),
            Ok(1)
        );
        assert!(
            <i32>::try_from(&RObj::from(vec![1, 2].as_slice() as &[i32])).is_err()
        );
        let robj = RObj::from(vec![1, 2].as_slice() as &[i32]);
        assert_eq!(
            <i32>::try_from(robj.clone()),
            Err(Error::ExpectedScalar(robj))
        );

        use std::collections::HashMap;
        let list = eval_string("list(a = 1L, b = 2L)").unwrap();
        let hmap1 = [("a".into(), 1.into()), ("b".into(), 2.into())]
            .iter()
            .cloned()
            .collect::<HashMap<String, RObj>>();
        let hmap2 = [("a", 1.into()), ("b", 2.into())]
            .iter()
            .cloned()
            .collect::<HashMap<&str, RObj>>();
        let hmap_owned = <HashMap<String, RObj>>::try_from(&list).unwrap();
        let hmap_borrowed = <HashMap<&str, RObj>>::try_from(&list).unwrap();
        assert_eq!(hmap_owned, hmap1);
        assert_eq!(hmap_borrowed, hmap2);

        assert_eq!(hmap_owned["a"], RObj::from(1));
        assert_eq!(hmap_owned["b"], RObj::from(2));

        assert_eq!(hmap_borrowed["a"], RObj::from(1));
        assert_eq!(hmap_borrowed["b"], RObj::from(2));
        let hmap_borrowed: HashMap<&str, RObj> = list.as_list().unwrap().try_into().unwrap();
        assert_eq!(hmap_borrowed, hmap2);

        let na_integer = eval_string("NA_integer_").unwrap();
        assert!(<i32>::try_from(&na_integer).is_err());
        assert_eq!(<Option<i32>>::try_from(&na_integer), Ok(None));
        assert_eq!(<Option<i32>>::try_from(&RObj::from(1)), Ok(Some(1)));
        assert!(<Option<i32>>::try_from(&RObj::from([1, 2])).is_err());

        let na_bool = eval_string("TRUE == NA").unwrap();
        assert!(<bool>::try_from(&na_bool).is_err());
        assert_eq!(<Option<bool>>::try_from(&na_bool), Ok(None));
        assert_eq!(<Option<bool>>::try_from(&RObj::from(true)), Ok(Some(true)));
        assert!(<Option<bool>>::try_from(&RObj::from([true, false])).is_err());

        let na_real = eval_string("NA_real_").unwrap();
        assert!(<f64>::try_from(&na_real).is_err());
        assert_eq!(<Option<f64>>::try_from(&na_real), Ok(None));
        assert_eq!(<Option<f64>>::try_from(&RObj::from(1.)), Ok(Some(1.)));
        assert!(<Option<f64>>::try_from(&RObj::from([1., 2.])).is_err());

        let na_string = eval_string("NA_character_").unwrap();
        assert!(<&str>::try_from(&na_string).is_err());
        assert_eq!(<Option<&str>>::try_from(&na_string), Ok(None));
        assert_eq!(<Option<&str>>::try_from(&RObj::from("1")), Ok(Some("1")));
        assert!(<Option<&str>>::try_from(&RObj::from(["1", "2"])).is_err());

        let na_string = eval_string("NA_character_").unwrap();
        assert!(<String>::try_from(&na_string).is_err());
        assert_eq!(<Option<String>>::try_from(&na_string), Ok(None));
        assert_eq!(
            <Option<String>>::try_from(&RObj::from("1")),
            Ok(Some("1".to_string()))
        );
        assert!(<Option<String>>::try_from(&RObj::from(["1", "2"])).is_err());

        assert_eq!(f64::from(RFloat::from(1.0)), 1.0);
        assert!(f64::from(RFloat::na()).is_na());

        assert_eq!(i32::from(RInt::from(1)), 1);
        assert!(RInt::from(i32::from(RInt::na())).is_na());

        assert_eq!(bool::from(RBool::from(true)), true);
        assert_eq!(bool::from(RBool::from(false)), false);

        assert_eq!(c64::from(RCplx::from(c64::new(1.0, 2.0))), c64::new(1.0, 2.0));
        assert!(c64::from(RCplx::na()).is_na());

        // TODO: once related todos resolved in try_from_robj.rs, add tests for
        // Doubles to Integer successful case (e.g. 1.0 to 1) and failing case
        // Integer to Doubles
        // NA handling
        let v: Result<Doubles> = r!(NA_REAL).try_into();
        let mut v: Vec<_> = v.unwrap().iter().collect();
        assert!(v.pop().unwrap().is_nan());
        assert_eq!(<Doubles>::try_from(r!([1.0, 2.0])).unwrap().iter().map(|v| v.0).collect::<Vec<f64>>(), vec![1.0, 2.0]);
        assert!(<Doubles>::try_from(r!([true])).is_err());

        assert_eq!(<Integers>::try_from(r!([1, 2])).unwrap().iter().map(|v| v.0).collect::<Vec<i32>>(), vec![1, 2]);
        assert!(<Integers>::try_from(r!([true])).is_err());

        assert_eq!(<Logicals>::try_from(r!([true, false])).unwrap().iter().collect::<Vec<RBool>>(), vec![TRUE, FALSE]);
        assert!(<Logicals>::try_from(r!([1])).is_err());

        let robj = RObj::from(1);
        assert_eq!(<&[RInt]>::try_from(&robj), Ok(&[RInt::from(1)][..]));
        let robj = RObj::from(1.);
        assert_eq!(<&[RFloat]>::try_from(&robj), Ok(&[RFloat::from(1.)][..]));
        let robj = RObj::from(TRUE);
        assert_eq!(<&[RBool]>::try_from(&robj), Ok(&[TRUE][..]));
        let robj = RObj::from(0_u8);
        assert_eq!(<&[u8]>::try_from(&robj), Ok(&[0_u8][..]));

        // Note the Vec<> cases use the same logic as the slices.
        let robj = RObj::from(1.0);
        assert_eq!(<&[RInt]>::try_from(&robj), Err(Error::ExpectedInteger(r!(1.0))));
        let robj = RObj::from(1);
        assert_eq!(<&[RFloat]>::try_from(&robj), Err(Error::ExpectedReal(r!(1))));
        let robj = RObj::from(());
        assert_eq!(<&[RBool]>::try_from(&robj), Err(Error::ExpectedLogical(r!(()))));
        assert_eq!(<&[u8]>::try_from(&robj), Err(Error::ExpectedRaw(r!(()))));
    }
}

#[test]
fn test_to_robj() {
    test! {
        assert_eq!(RObj::from(true), RObj::from([RBool::from(true)]));
        //assert_eq!(RObj::from(1_u8), RObj::from(1));
        assert_eq!(RObj::from(1_u16), RObj::from(1));
        assert_eq!(RObj::from(1_u32), RObj::from(1.));
        assert_eq!(RObj::from(1_u64), RObj::from(1.));
        assert_eq!(RObj::from(1_usize), RObj::from(1.));
        assert_eq!(RObj::from(1_i8), RObj::from(1));
        assert_eq!(RObj::from(1_i16), RObj::from(1));
        assert_eq!(RObj::from(1_i32), RObj::from(1));
        assert_eq!(RObj::from(1_i64), RObj::from(1.));
        assert_eq!(RObj::from(1.0_f32), RObj::from(1.));
        assert_eq!(RObj::from(1.0_f64), RObj::from(1.));

        // check large values
        assert_eq!(RObj::from(i64::MAX), RObj::from(i64::MAX as f64));
        assert_eq!(RObj::from(i64::MIN), RObj::from(i64::MIN as f64));

        // check NaN and Inf
        assert_eq!(RObj::from(f64::NAN), R!("NaN")?);
        assert_eq!(RObj::from(f64::INFINITY), R!("Inf")?);
        assert_eq!(RObj::from(-f64::INFINITY), R!("-Inf")?);

        let ab = RObj::from(vec!["a", "b"]);
        let ab2 = RObj::from(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(ab, ab2);

        assert_eq!(RObj::from(Some(1)), RObj::from(1));
        assert!(!RObj::from(Some(1)).is_na());
        assert!(RObj::from(<Option<i32>>::None).is_na());

        assert_eq!(RObj::from(Some(true)), RObj::from(true));
        assert!(!RObj::from(Some(true)).is_na());
        assert!(RObj::from(<Option<bool>>::None).is_na());

        assert_eq!(RObj::from(Some(1.)), RObj::from(1.));
        assert!(!RObj::from(Some(1.)).is_na());
        assert!(RObj::from(<Option<f64>>::None).is_na());

        assert_eq!(RObj::from(Some("xyz")), RObj::from("xyz"));
        assert!(!RObj::from(Some("xyz")).is_na());
        assert!(RObj::from(<Option<&str>>::None).is_na());
    }
}

#[test]
fn parse_test() {
    test! {
    let p = Expressions::from_str("print(1L);print(1L);")?;
    let q = Expressions::from_values(&[
        r!(Language::from_values(&[r!(Symbol::from_string("print")), r!(1)])),
        r!(Language::from_values(&[r!(Symbol::from_string("print")), r!(1)]))
    ]);
    assert_eq!(p, q);

    let p = eval_string("1L + 1L")?;
    assert_eq!(p, RObj::from(2));
    }
}

#[test]
fn output_iterator_test() {
    test! {
        // Allocation where size is known in advance.
        let robj = (0..3).collect_robj();
        assert_eq!(robj.as_integer_vector().unwrap(), vec![0, 1, 2]);

        let robj = [0, 1, 2].iter().collect_robj();
        assert_eq!(robj.as_integer_vector().unwrap(), vec![0, 1, 2]);

        let robj = (0..3).map(|x| x % 2 == 0).collect_robj();
        assert_eq!(robj.as_logical_vector().unwrap(), vec![TRUE, FALSE, TRUE]);

        let robj = [TRUE, FALSE, TRUE].iter().collect_robj();
        assert_eq!(robj.as_logical_vector().unwrap(), vec![TRUE, FALSE, TRUE]);

        let robj = (0..3).map(|x| x as f64).collect_robj();
        assert_eq!(robj.as_real_vector().unwrap(), vec![0., 1., 2.]);

        let robj = [0., 1., 2.].iter().collect_robj();
        assert_eq!(robj.as_real_vector().unwrap(), vec![0., 1., 2.]);

        let robj = (0..3).map(|x| format!("{}", x)).collect_robj();
        assert_eq!(robj.as_str_vector(), Some(vec!["0", "1", "2"]));

        let robj = ["0", "1", "2"].iter().collect_robj();
        assert_eq!(robj.as_str_vector(), Some(vec!["0", "1", "2"]));

        // Fallback allocation where size is not known in advance.
        let robj = (0..3).filter(|&x| x != 1).collect_robj();
        assert_eq!(robj.as_integer_vector().unwrap(), vec![0, 2]);

        let robj = (0..3).filter(|&x| x != 1).map(|x| x as f64).collect_robj();
        assert_eq!(robj.as_real_vector().unwrap(), vec![0., 2.]);

        let robj = (0..3)
            .filter(|&x| x != 1)
            .map(|x| format!("{}", x))
            .collect_robj();
        assert_eq!(robj.as_str_vector(), Some(vec!["0", "2"]));
    }
}

// Test that we can use Iterators as the input to functions.
// eg.
// #[extendr]
// fn fred(a: Doubles, b: Doubles) -> RObj {
// }
#[test]
fn input_iterator_test() {
    test! {
        let src: &[&str] = &["1", "2", "3"];
        let robj = RObj::from(src);
        let iter = <StrIter>::try_from(&robj).unwrap();
        assert_eq!(iter.collect::<Vec<_>>(), src);

        let src = &[RObj::from(1), RObj::from(2), RObj::from(3)];
        let robj = RObj::from(List::from_values(src));
        let iter = <ListIter>::try_from(&robj).unwrap();
        assert_eq!(iter.collect::<Vec<_>>(), src);

        let src: &[i32] = &[1, 2, 3];
        let robj = RObj::from(src);
        let iter = <Integers>::try_from(&robj).unwrap().iter();
        assert_eq!(iter.collect::<Vec<_>>(), src);

        let src: &[f64] = &[1., 2., 3.];
        let robj = RObj::from(src);
        let iter = <Doubles>::try_from(&robj).unwrap().iter();
        assert_eq!(iter.collect::<Vec<_>>(), src);

        /*
        let src: &[RBool] = &[TRUE, FALSE, TRUE];
        let robj = RObj::from(src);
        let iter = <Logical>::try_from(&robj).unwrap();
        assert_eq!(iter.collect::<Vec<_>>(), src);
        */
    }
}

#[test]
fn test_rstr_rtype() {
    test! {
    let robj = r!(RStr::from("hello"));
    assert_eq!(RStr::try_from(robj).unwrap().rtype(), RType::RStr);
    }
}
