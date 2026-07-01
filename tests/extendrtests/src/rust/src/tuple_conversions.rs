use extendr_api::{error::Result, prelude::*};

#[extendr]
fn sum_triplet_ints(x: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32)) -> RInt {
    RInt::from(x.0 + x.1)
}

#[derive(Debug, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

impl TryFrom<RObj> for Point {
    type Error = extendr_api::Error;
    fn try_from(value: RObj) -> Result<Self> {
        let dbl_vec = Doubles::try_from(value)?;
        let x = dbl_vec[0].0;
        let y = dbl_vec[1].0;
        Ok(Point { x, y })
    }
}

impl TryFrom<&RObj> for Point {
    type Error = extendr_api::Error;
    fn try_from(value: &RObj) -> Result<Self> {
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
fn round_trip_array_rint(x: [RInt; 4]) -> [RInt; 4] {
    x
}

#[extendr]
fn round_trip_array_rfloat(x: [RFloat; 4]) -> [RFloat; 4] {
    x
}

#[extendr]
fn round_trip_array_rbool(x: [RBool; 4]) -> [RBool; 4] {
    x
}

#[extendr]
fn round_trip_array_rcplx(x: [RCplx; 4]) -> [RCplx; 4] {
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
