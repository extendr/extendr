use extendr_api::prelude::*;
use faer::Mat;

#[extendr]
fn faer_mat(x: Robj) -> Nullable<Robj> {
    let m = Mat::<f64>::from_robj(&x);
    match m {
        Ok(m) => Nullable::NotNull(m.into_robj()),
        Err(_) => Nullable::Null
    }
}

// Macro to generate exports
extendr_module! {
    fn faer_mat;

    mod optional_faer;
}
