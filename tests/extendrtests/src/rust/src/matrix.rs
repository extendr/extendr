use extendr_api::prelude::*;

#[extendr]
fn fetch_dimnames(x: RMatrix<f64>) -> List {
    x.get_dimnames()
}

#[extendr]
fn fetch_rownames(x: RMatrix<f64>) -> Option<Strings> {
    x.get_rownames()
}

#[extendr]
fn fetch_colnames(x: RMatrix<f64>) -> Option<Strings> {
    x.get_colnames()
}

#[extendr]
fn change_dimnames(mut x: RMatrix<f64>) -> Robj {
    let rownames = Strings::from_values(["AA", "BB", "CC"]);
    x.set_dimnames(list!(rownames, NULL));
    x.to_owned()
}

#[extendr]
fn matrix_3d_return(x: RMatrix3D<f64>) -> RMatrix3D<f64> {
    x
}

#[extendr]
fn matrix_4d_return(x: RMatrix4D<f64>) -> RMatrix4D<f64> {
    x
}

#[extendr]
fn matrix_5d_return(x: RMatrix5D<f64>) -> RMatrix5D<f64> {
    x
}

extendr_module! {
    mod matrix;
    fn fetch_dimnames;
    fn fetch_colnames;
    fn fetch_rownames;
    fn change_dimnames;
    fn matrix_3d_return;
    fn matrix_4d_return;
    fn matrix_5d_return;
}
