use extendr_api::prelude::*;
use faer::{Mat, MatRef};

#[extendr]
fn mat_to_mat(x: Mat<f64>) -> Mat<f64> {
    x
}

#[extendr]
fn mat_to_rmat(x: Mat<f64>) -> RMatrix<f64> {
    RMatrix::<f64>::from(x)
}

#[extendr]
fn mat_to_robj(x: Mat<f64>) -> RObj {
    x.into_robj()
}

#[extendr]
fn mat_to_rmatfloat(x: Mat<f64>) -> RMatrix<RFloat> {
    RMatrix::<RFloat>::from(x)
}

// convert to Mat<f64> from other things
#[extendr]
fn rmat_to_mat(x: RMatrix<f64>) -> Mat<f64> {
    Mat::<f64>::from(x)
}

#[extendr]
fn robj_to_mat(x: RObj) -> Mat<f64> {
    Mat::<f64>::try_from(x).unwrap()
}

// MatRef input
#[extendr]
fn matref_to_mat(x: MatRef<'_, f64>) -> RObj {
    RMatrix::<f64>::from(x).into()
}

// Macro to generate exports
extendr_module! {
    mod optional_faer;
    fn mat_to_mat;
    fn mat_to_rmat;
    fn mat_to_robj;
    fn mat_to_rmatfloat;
    fn rmat_to_mat;
    fn robj_to_mat;
    fn matref_to_mat;
}
