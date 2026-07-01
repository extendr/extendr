use extendr_api::prelude::*;

#[extendr]
fn type_aware_sum(input: Either<Integers, Doubles>) -> Either<RInt, RFloat> {
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
