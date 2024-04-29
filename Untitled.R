rextendr::rust_source(code = r"(
#[derive(Debug)]
pub struct Mat(pub RMatrix<f64>);

#[extendr]
impl Mat {
    fn new(mat: RMatrix<f64>) -> Self {
        Mat(mat)
    }

    fn get_mat(&self) -> &Robj {
      let robj = &self.0;
      unsafe { std::mem::transmute(robj) }
    }

    fn sum(&self) -> f64 {
        self.0
            .as_real_slice()
            .unwrap()
            .iter()
            .fold(0.0, |mut acc, next| {
                acc += next;
                acc
            })
    }
}
)")
a <- Mat$new(pracma::magic(4))

a$sum()
m <- a$get_mat()[]
m[3] <- 1000
a$get_mat()


