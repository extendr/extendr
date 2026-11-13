use extendr_api::prelude::*;

#[derive(Debug, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
#[extendr(vctrs)]
pub struct Points(Vec<Option<Point>>);

impl Rvctr for Points {
    fn length(&self) -> usize {
        self.0.len()
    }

    fn format(&self) -> Strings {
        let mut out = Strings::new_with_na(self.0.len());
        for (i, p) in self.0.iter().enumerate() {
            if let Some(p) = p {
                out.set_elt(i, format!("({}, {})", p.x, p.y).into());
            }
        }
        out
    }

    fn subset(&self, idx: &[Option<usize>]) -> Self {
        Points(Rvctr::subset(&self.0, idx))
    }

    fn combine(&mut self, other: &Self) {
        self.0.extend_from_slice(&other.0)
    }

    fn is_na(&self, i: usize) -> bool {
        self.0[i].is_none()
    }
}

/// Create points from coordinates. `NA` in either coordinate gives a missing point.
/// @export
#[extendr]
fn points(x: Doubles, y: Doubles) -> Points {
    Points(
        x.iter()
            .zip(y.iter())
            .map(|(x, y)| {
                if x.is_na() || y.is_na() {
                    None
                } else {
                    Some(Point { x: x.0, y: y.0 })
                }
            })
            .collect(),
    )
}

/// Sum of the x coordinates, skipping missing points.
/// @export
#[extendr]
fn points_sum_x(x: &Points) -> f64 {
    x.0.iter().flatten().map(|p| p.x).sum()
}

#[derive(Debug, Clone)]
#[extendr(vctrs, r_name = "extendr_labels")]
pub struct Labels(Vec<Option<String>>);

impl Rvctr for Labels {
    fn length(&self) -> usize {
        self.0.len()
    }

    fn format(&self) -> Strings {
        Rvctr::format(&self.0)
    }

    fn subset(&self, idx: &[Option<usize>]) -> Self {
        Labels(Rvctr::subset(&self.0, idx))
    }

    fn combine(&mut self, other: &Self) {
        self.0.extend_from_slice(&other.0)
    }

    fn is_na(&self, i: usize) -> bool {
        Rvctr::is_na(&self.0, i)
    }
}

/// Create labels.
/// @export
#[extendr]
fn labels(x: Vec<String>) -> Labels {
    Labels(x.into_iter().map(Some).collect())
}

extendr_module! {
    mod vctrs;
    fn points;
    fn points_sum_x;
    fn labels;
    vctr Points;
    vctr Labels;
}
