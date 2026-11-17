use extendr_api::prelude::*;

#[cfg(r_4_6)]
mod test;

#[cfg(r_4_6)]
extendr_module! {
    mod dots;
    use test;
}

#[cfg(not(r_4_6))]
extendr_module! {
    mod dots;
}
