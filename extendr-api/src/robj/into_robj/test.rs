use super::*;
use crate as extendr_api;

#[test]
fn test_vec_rint_to_robj() {
    test! {
        let int_vec = vec![3,4,0,-2];
        let int_vec_robj: Robj = int_vec.clone().into();
        // unsafe { extendr_ffi::Rf_PrintValue(int_vec_robj.get())}
        assert_eq!(int_vec_robj.as_integer_slice().unwrap(), &int_vec);

        let rint_vec = vec![Rint::from(3), Rint::from(4), Rint::from(0), Rint::from(-2)];
        let rint_vec_robj: Robj = rint_vec.into();
        // unsafe { extendr_ffi::Rf_PrintValue(rint_vec_robj.get())}
        assert_eq!(rint_vec_robj.as_integer_slice().unwrap(), &int_vec);
    }
}

#[test]
fn test_collect_rarray_matrix() {
    test! {
        // Check that collect_rarray works the same as R's matrix() function
        let rmat = (1i32..=16).collect_rarray([4, 4]);
        assert!(rmat.is_ok());
        assert_eq!(Robj::from(rmat), R!("matrix(1:16, nrow=4)").unwrap());
    }
}

#[test]
fn test_collect_rarray_tensor() {
    test! {
        // Check that collect_rarray works the same as R's array() function
        let rmat = (1i32..=16).collect_rarray([2, 4, 2]);
        assert!(rmat.is_ok());
        assert_eq!(Robj::from(rmat), R!("array(1:16, dim=c(2, 4, 2))").unwrap());
    }
}

#[test]
fn test_collect_rarray_matrix_failure() {
    test! {
        // Check that collect_rarray fails when given an invalid shape
        let rmat = (1i32..=16).collect_rarray([3, 3]);
        assert!(rmat.is_err());
        let msg = rmat.unwrap_err().to_string();
        assert!(msg.contains('9'));
        assert!(msg.contains("dimension"));
    }
}

#[test]
fn test_collect_tensor_failure() {
    test! {
        // Check that collect_rarray fails when given an invalid shape
        let rmat = (1i32..=16).collect_rarray([3, 3, 3]);
        assert!(rmat.is_err());
        let msg = rmat.unwrap_err().to_string();
        assert!(msg.contains("27"));
        assert!(msg.contains("dimension"));
    }
}

#[test]
#[cfg(all(feature = "result_condition", not(feature = "result_list")))]
fn test_result_condition() {
    use crate::prelude::*;
    fn my_err_f() -> std::result::Result<f64, f64> {
        Err(42.0) // return err float
    }

    test! {
              assert_eq!(
                r!(my_err_f()),
                R!(
    "structure(list(message = 'extendr_err',
        value = 42.0), class = c('extendr_error', 'error', 'condition'))"
                ).unwrap()
            );
        }
}

#[test]
#[cfg(feature = "result_list")]
fn test_result_list() {
    use crate::prelude::*;
    fn my_err_f() -> std::result::Result<f64, String> {
        Err("We have water in the engine room!".to_string())
    }

    fn my_ok_f() -> std::result::Result<f64, String> {
        Ok(123.123)
    }

    test! {
        assert_eq!(
            r!(my_err_f()),
            R!("x=list(ok=NULL, err='We have water in the engine room!')
                    class(x)='extendr_result'
                    x"
            ).unwrap()
        );
        assert_eq!(
            r!(my_ok_f()),
            R!("x = list(ok=123.123, err=NULL)
                    class(x)='extendr_result'
                    x"
            ).unwrap()
        );
    }
}
