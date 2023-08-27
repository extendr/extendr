use super::*;

#[test]
fn test_collect_rarray_matrix() {
    with_r(|| {
        // Check that collect_rarray works the same as R's matrix() function
        let rmat = (1i32..=16).collect_rarray([4, 4]);
        assert!(rmat.is_ok());
        assert_eq!(Robj::from(rmat), R!("matrix(1:16, nrow=4)").unwrap());
    });
}

#[test]
fn test_collect_rarray_tensor() {
    with_r(|| {
        // Check that collect_rarray works the same as R's array() function
        let rmat = (1i32..=16).collect_rarray([2, 4, 2]);
        assert!(rmat.is_ok());
        assert_eq!(Robj::from(rmat), R!("array(1:16, dim=c(2, 4, 2))").unwrap());
    });
}

#[test]
fn test_collect_rarray_matrix_failure() {
    with_r(|| {
        // Check that collect_rarray fails when given an invalid shape
        let rmat = (1i32..=16).collect_rarray([3, 3]);
        assert!(rmat.is_err());
        let msg = rmat.unwrap_err().to_string();
        assert!(msg.contains("9"));
        assert!(msg.contains("dimension"));
    });
}

#[test]
fn test_collect_tensor_failure() {
    with_r(|| {
        // Check that collect_rarray fails when given an invalid shape
        let rmat = (1i32..=16).collect_rarray([3, 3, 3]);
        assert!(rmat.is_err());
        let msg = rmat.unwrap_err().to_string();
        assert!(msg.contains("27"));
        assert!(msg.contains("dimension"));
    });
}
