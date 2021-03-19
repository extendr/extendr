//! Polynomial approximation for efficient function generation.

/// A polynomial 
struct Polynomial {
    terms: Vec<f64>,
}

/// Find the first row of the divided differences.
pub (crate) fn divided_differences(x: &[f64], y: &mut [f64]) {
    let k = y.len();
    for i in 0..k-1 {
        for j in (i..k-1).rev() {
            y[j+1] = (y[j+1] - y[j]) / (x[j+1] - x[j-i]);
            // println!("{} {} ({}-{})/({}-{}) {}", i, j, j-i+1, j-i, j+1, j-i, y[j+1]);
        }
        // for z in 0..k {
        //     print!("{}/{} ", z, y[z]);
        // }
        // println!("");
    }
}

impl Polynomial {
    pub fn from_points(x: &[f64], y: &[f64]) -> Self {
        // https://en.wikipedia.org/wiki/Newton_polynomial

        let k = y.len();
        let mut dd = Vec::from(y);
        let mut newton = vec![0.; k];
        let mut terms = vec![0.; k];
        newton[0] = 1.;

        divided_differences(x, &mut *dd);

        // println!("x= {:?}", x);
        // println!("y= {:?}", y);
        // println!("dd={:?}", dd);

        let k = y.len();
        for i in 0..k {
            // println!("n={:?}", newton);
            for j in 0..k {
                terms[j] += newton[j] * dd[i];
            }

            // Multiply "newton" by (x - x[i])
            let c = -x[i];
            for i in (1..k).rev() {
                newton[i] = newton[i] * c + newton[i-1];
            }
            newton[0] *= c;
            // println!("t={:?}", terms);
        }
        Self { terms }
    }

    pub fn eval(&self, x: f64) -> f64 {
        let l = self.terms.len();
        let mut y = self.terms[l-1];
        for i in (0..l-1).rev() {
            y = y.mul_add(x, self.terms[i]);
        }
        y
    }

    pub fn terms(&self) -> &[f64] {
        &*self.terms
    }
}

mod test {
    use super::*;

    // #[test]
    // fn test_dd() {
    //     let x = &[-3.0/2., -3.0/4., 0., 3.0/4., 3.0/2.];
    //     let y = &mut [-14.1014, -0.931596, 0., 0.931596, 14.1014];
    
    //     divided_differences(x, y);
    // }
    
    #[test]
    fn test_poly() {
        // test the two examples from Wikipedia.
        let x = &[1., 2., 3., 4.];
        let y = &mut [6., 9., 2., 5.];

        let p = Polynomial::from_points(x, y);
        let y2 = (0..x.len()).map(|i| p.eval(x[i])).collect::<Vec<_>>();
        println!("x={:?} y={:?} y2={:?}", x, y, y2);

        let err = (0..x.len()).map(|i| (y2[i] - y[i]).abs()).collect::<Vec<_>>();
        assert!(!err.iter().any(|&e| e > 0.00001));

        let x = &[-3.0/2., -3.0/4., 0., 3.0/4., 3.0/2.];
        let y = &mut [-14.1014, -0.931596, 0., 0.931596, 14.1014];
    
        let p = Polynomial::from_points(x, y);
        let y2 = (0..x.len()).map(|i| p.eval(x[i])).collect::<Vec<_>>();
        println!("x={:?} y={:?} y2={:?}", x, y, y2);

        let err = (0..x.len()).map(|i| (y2[i] - y[i]).abs()).collect::<Vec<_>>();
        assert!(!err.iter().any(|&e| e > 0.00001));
    }
}

