use extendr_api::prelude::*;
use extendr_api::Rvctr;

#[derive(Debug, Clone, Rvctr)]
pub struct Points(Vec<Option<(f64, f64)>>);

#[extendr]
fn make_points(x: Doubles, y: Doubles) -> Points {
    Points(
        x.iter()
            .zip(y.iter())
            .map(|(x, y)| match x.is_na() || y.is_na() {
                true => None,
                false => Some((x.0, y.0)),
            })
            .collect(),
    )
}

extendr_module! {
    mod optional_vctrs;
    fn make_points;
    use vctr_Points;
}
