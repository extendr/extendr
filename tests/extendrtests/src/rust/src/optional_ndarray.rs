use extendr_api::prelude::*;
use ndarray::{s, ArrayView2};

/// Calculate Euclidean distance matrix
/// Test case adopted from https://github.com/mikemahoney218/examplerust/blob/23d21b1ced4e24b7a7c00dd36290114dc1bbd113/src/rust/src/lib.rs#L5
/// @param a : Matrix of real values or `NULL`
/// @export
#[extendr]
fn euclidean_dist(a: Nullable<ArrayView2<Rfloat>>) -> Nullable<Doubles> {
    if let NotNull(a) = a {
        let nrow = a.nrows();

        let result = (0..(nrow - 1))
            .flat_map(|x| {
                ((x + 1)..nrow).map(move |y| {
                    let z = &a.slice(s![x, ..]) - &a.slice(s![y, ..]);
                    (&z * &z).iter().sum::<Rfloat>().sqrt()
                })
            })
            .collect();

        Nullable::NotNull(result)
    } else {
        Nullable::Null
    }
}

// Macro to generate exports
extendr_module! {
    fn euclidean_dist;

    mod optional_ndarray;
}
