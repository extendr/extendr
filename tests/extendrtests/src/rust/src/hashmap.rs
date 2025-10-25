use extendr_api::prelude::*;
use std::collections::HashMap;

#[extendr]
fn test_hm_string(mut x: HashMap<String, Robj>) -> List {
    x.insert("inserted_value".to_string(), List::new(0).into());
    List::from_hashmap(x).unwrap()
}

#[extendr]
fn test_hm_i32(mut x: HashMap<String, i32>) -> List {
    x.insert("inserted_value".to_string(), 314);
    List::from_hashmap(x).unwrap()
}

struct Point {
    x: f64,
    y: f64,
}

impl TryFrom<Robj> for Point {
    type Error = Error;

    fn try_from(value: Robj) -> std::result::Result<Self, Self::Error> {
        let inner_vec = Doubles::try_from(value)?;
        let x = inner_vec[0].inner();
        let y = inner_vec[1].inner();
        Ok(Point { x, y })
    }
}

impl From<Point> for Doubles {
    fn from(value: Point) -> Self {
        Doubles::from_values([value.x, value.y])
    }
}

impl From<Point> for Robj {
    fn from(value: Point) -> Self {
        Robj::from(Doubles::from(value))
    }
}
#[extendr]
fn test_hm_custom_try_from(mut x: HashMap<&str, Point>) -> List {
    x.insert("inserted_value", Point { x: 3.0, y: 0.1415 });
    List::from_hashmap(x).unwrap()
}

#[extendr]
fn test_robj_from_hashmap() -> Robj {
    let solar_distance = HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);

    solar_distance.into()
}

#[extendr]
fn test_robj_from_btreemap() -> Robj {
    let solar_distance = BTreeMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);

    solar_distance.into()
}

extendr_module! {
    mod hashmap;
    fn test_hm_string;
    fn test_hm_i32;
    fn test_hm_custom_try_from;
    fn test_robj_from_hashmap;
    fn test_robj_from_btreemap;
}
