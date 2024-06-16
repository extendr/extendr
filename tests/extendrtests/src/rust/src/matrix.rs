use extendr_api::prelude::*;

// #[extendr]
// fn fetch_dimnames(x: RMatrix<f64>) -> Robj {
//     x.get_dimnames().into()
// }

#[extendr]
fn fetch_rownames(x: RMatrix<f64>) -> Robj {
    x.get_rownames()
}

#[extendr]
fn fetch_colnames(x: RMatrix<f64>) -> Robj {
    x.get_colnames()
}

#[extendr]
fn change_dimnames(mut x: RMatrix<f64>) -> Robj {
    let rownames = Strings::from_values(["AA", "BB", "CC"]);
    x.set_dimnames(list!(rownames, NULL));
    x.to_owned()
}

extendr_module! {
    mod matrix;
    // fn fetch_dimnames;
    fn fetch_colnames;
    fn fetch_rownames;
    fn change_dimnames;
}