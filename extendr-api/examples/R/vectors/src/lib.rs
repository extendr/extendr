
use extendr_api::*;
use ndarray::prelude::*;

#[export_function]
fn add(a: ArrayView1<f64>, b: ArrayView1<f64>) -> Robj {
    let len = a.len().min(b.len());
    let mut result = rep!(0., len);
    let mut d = ArrayViewMut1::<f64>::from(result.as_f64_slice_mut().expect("must be a numeric vector"));
    azip!((d in &mut d, a in &a, b in &b) *d = a + b);
    result
}

