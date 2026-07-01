use extendr_api::prelude::*;

// Makes the middle value the answer to the universe
#[extendr]
fn middle_zero(integers: &mut [RInt]) {
    let middle = integers.len() / 2;
    integers[middle] = 42_i32.into();
}

#[extendr]
fn logicals_sum(logicals: &[RBool]) -> i32 {
    logicals.iter().fold(0, |acc, next| {
        let bool = next.is_true();
        acc + bool as i32
    })
}

#[extendr]
fn floats_mean(x: &[RFloat]) -> f64 {
    let n = x.len();

    let x_sum = x.iter().fold(0.0, |acc, next| {
        if next.is_na() {
            acc
        } else {
            let v = next.0;
            acc + v
        }
    });

    x_sum / n as f64
}

extendr_module! {
    mod typedsliceargs;
    fn middle_zero;
    fn logicals_sum;
    fn floats_mean;
}
