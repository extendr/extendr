use extendr_api::prelude::*;

#[extendr(use_try_from = true)]
fn type_aware_sum(input: Either<Integers, Doubles>) -> Either<Rint, Rfloat> {
    match input {
        Left(left) => Left(left.iter().sum()),
        Right(right) => Right(right.iter().sum()),
    }
}

// Macro to generate exports
extendr_module! {
    fn type_aware_sum;

    mod optional_either;
}
