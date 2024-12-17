use extendr_api::prelude::*;

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

#[extendr]
fn sum_points(x: (Point, Point)) -> Doubles {
    let Point { x: x1, y: y1 } = x.0;
    let Point { x: x2, y: y2 } = x.1;
    Doubles::from_values([x1 + x2, y1 + y2])
}

#[extendr]
fn round_trip_array_u8(x: [u8; 4]) -> [u8; 4] {
    x
}

#[extendr]
fn round_trip_array_f64(x: [f64; 4]) -> [f64; 4] {
    x
}

#[extendr]
fn round_trip_array_i32(x: [i32; 4]) -> [i32; 4] {
    x
}

#[extendr]
fn round_trip_array_rint(x: [Rint; 4]) -> [Rint; 4] {
    x
}

#[extendr]
fn round_trip_array_rfloat(x: [Rfloat; 4]) -> [Rfloat; 4] {
    x
}

#[extendr]
fn round_trip_array_rbool(x: [Rbool; 4]) -> [Rbool; 4] {
    x
}

#[extendr]
fn round_trip_array_rcplx(x: [Rcplx; 4]) -> [Rcplx; 4] {
    x
}

extendr_module! {
    mod tuple_conversions;
    fn sum_triplet_ints;
    fn sum_points;
    fn round_trip_array_f64;
    fn round_trip_array_i32;
    fn round_trip_array_rbool;
    fn round_trip_array_rcplx;
    fn round_trip_array_rfloat;
    fn round_trip_array_rint;
    fn round_trip_array_u8;
}
