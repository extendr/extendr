library(rextendr)
rextendr::rust_source
getOption("rextendr.patch.crates_io")
rextendr::rust_function("
fn faer_mat(x: Robj) -> Robj {
    use faer::Mat;
    let m = Mat::<f64>::from_robj(&x).unwrap();
    m.into_robj()
}
", quiet = FALSE, dependencies = c(faer = "*"))

# r"(faer = "*")"
