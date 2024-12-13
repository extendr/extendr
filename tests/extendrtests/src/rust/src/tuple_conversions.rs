use extendr_api::prelude::*;

/// @export
#[extendr]
fn sum_triplet_ints(x: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32)) -> Rint {
    Rint::from(x.0 + x.1)
}

#[derive(Debug, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

impl TryFrom<Robj> for Point {
    type Error = extendr_api::Error;
    fn try_from(value: Robj) -> std::result::Result<Self, Self::Error> {
        let dbl_vec = Doubles::try_from(value)?;
        let x = dbl_vec[0].inner();
        let y = dbl_vec[1].inner();
        Ok(Point { x, y })
    }
}

impl TryFrom<&Robj> for Point {
    type Error = extendr_api::Error;
    fn try_from(value: &Robj) -> std::result::Result<Self, Self::Error> {
        value.clone().try_into()
    }
}

/// @export
#[extendr]
fn sum_points(x: (Point, Point)) -> Doubles {
    let Point { x: x1, y: y1 } = x.0;
    let Point { x: x2, y: y2 } = x.1;
    Doubles::from_values([x1 + x2, y1 + y2])
}

extendr_module! {
    mod tuple_conversions;
    fn sum_triplet_ints;
    fn sum_points;
}
