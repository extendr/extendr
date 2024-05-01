use extendr_api::prelude::*;
use faer::{Mat, MatRef};

//#[extendr]
//fn faer_mat(x: Robj) -> Nullable<Robj> {
//    let m = Mat::<f64>::from_robj(&x);
//    match m {
//        Ok(m) => Nullable::NotNull(m.into_robj()),
//        Err(_) => Nullable::Null
//    }
//}

#[extendr]
fn mat_to_mat(x: Mat<f64>) -> Mat<f64> {
    x
}

#[extendr]
fn mat_to_rmat(x: Mat<f64>) -> RMatrix<f64> {
    RMatrix::<f64>::from(x)
}

#[extendr]
fn mat_to_robj(x: Mat<f64>) -> Robj {
    x.into_robj()
}

#[extendr]
fn mat_to_rmatfloat(x: Mat<f64>) -> RMatrix<Rfloat> {
    RMatrix::<Rfloat>::from(x)
}

// convert to Mat<f64> from other things
#[extendr]
fn rmat_to_mat(x: RMatrix<f64>) -> Mat<f64> {
    Mat::<f64>::from(x)
}

#[extendr]
fn robj_to_mat(x: Robj) -> Mat<f64> {
    Mat::<f64>::try_from(x).unwrap()
}

// MatRef input
fn matref_to_mat(x: MatRef<'_, f64>) -> Robj {
    RMatrix::<f64>::from(x).into()
}

// Macro to generate exports
extendr_module! {
    //fn faer_mat;
    fn mat_to_mat;
    fn mat_to_rmat;
    fn mat_to_robj;
    fn mat_to_rmatfloat;
    fn rmat_to_mat;
    fn robj_to_mat;
    fn matref_to_mat;

    mod optional_faer;
}
