
use extendr_api::*;
use ndarray::prelude::*;

/// Add two vectors using `ArrayView1` and `ArrayViewMut1`.
/// The result is stored in an R vector, which is returned directly
/// so that no copying occurs.
#[export_function]
fn add1(a: ArrayView1<f64>, b: ArrayView1<f64>) -> Robj {
    // determine the length of the smallest vector.
    let len = a.len().min(b.len());

    // Use rep! to create a new R vector
    let mut result = rep!(0., len);

    // Get a mutable view of the result
    let mut d = ArrayViewMut1::<f64>::from(result.as_f64_slice_mut().expect("must be a numeric vector"));

    // Use azip to add the vectors.
    // Note: this will panic if the vectors are not the same length.
    azip!((d in &mut d, a in &a, b in &b) *d = a + b);
    result
}

