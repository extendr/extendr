use extendr_api::prelude::*;
use extendr_api::vctrs::Vctr;
use extendr_api::Result;
use extendr_api::Rvctr;

#[derive(Debug, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Rvctr)]
pub struct Points(Vec<Option<Point>>);

#[extendr]
fn make_points(x: Doubles, y: Doubles) -> Points {
    Points(
        x.iter()
            .zip(y.iter())
            .map(|(x, y)| match x.is_na() || y.is_na() {
                true => None,
                false => Some(Point { x: x.0, y: y.0 }),
            })
            .collect(),
    )
}

#[extendr]
fn my_centroid(points: &Points) -> Points {
    let (n, x, y) = points
        .0
        .iter()
        .flatten()
        .fold((0.0, 0.0, 0.0), |(n, x, y), p| (n + 1.0, x + p.x, y + p.y));
    Points(vec![(n > 0.0).then(|| Point { x: x / n, y: y / n })])
}

#[derive(Clone)]
pub struct Line {
    start: Point,
    end: Point,
}

impl std::fmt::Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Line(({}, {}) ({}, {}))",
            self.start.x, self.start.y, self.end.x, self.end.y
        )
    }
}

#[extendr]
fn make_lines(start: &Points, end: &Points) -> Vctr<Vec<Option<Line>>> {
    let lines = start
        .0
        .iter()
        .zip(end.0.iter())
        .map(|(start, end)| match (start, end) {
            (Some(start), Some(end)) => Some(Line {
                start: start.clone(),
                end: end.clone(),
            }),
            _ => None,
        })
        .collect::<Vec<_>>();
    Vctr::from(lines)
}

#[extendr]
fn line_lengths(lines: Vctr<Vec<Option<Line>>>) -> Result<Doubles> {
    let lines = lines.try_into_inner()?;
    Ok(lines
        .iter()
        .map(|line| match line {
            Some(line) => {
                Rfloat::from((line.end.x - line.start.x).hypot(line.end.y - line.start.y))
            }
            None => Rfloat::na(),
        })
        .collect())
}

extendr_module! {
    mod optional_vctrs;
    fn make_points;
    fn my_centroid;
    fn make_lines;
    fn line_lengths;
    use Points;
}
