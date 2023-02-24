//!
//!
//! See [User-supplied RNG](https://stat.ethz.ch/R-manual/R-patched/library/base/html/Random-user.html)

use crate::prelude::*;

#[cfg(test)]
mod tests {
    use libR_sys::{exp_rand, norm_rand, unif_rand, GetRNGstate, PutRNGstate, R_unif_index};

    use super::*;

    #[test]
    fn test_r_rng() {
        test! {
        // {
            unsafe { GetRNGstate() };
            unsafe {
                R!(r#"set.seed(20230205)"#).unwrap();
                let param = 10e3;
                let x = [
                    R_unif_index(param),
                    R_unif_index(param),
                    R_unif_index(param),
                    R_unif_index(param),
                    R_unif_index(param),
                ];
                R!(r#"set.seed(20230205)"#).unwrap();
                let y = [
                    R_unif_index(param),
                    R_unif_index(param),
                    R_unif_index(param),
                    R_unif_index(param),
                    R_unif_index(param),
                ];
                assert_eq!(y, x);
                // if the rng state isn't retrieved, then distribution is
                // 0 always
                assert_ne!(x, [0., 0., 0., 0., 0.]);

                // println!("{}", R_unif_index(5.4));
                // println!("{}", norm_rand());
                // println!("{}", exp_rand());
            }
            unsafe { PutRNGstate() };

            // R_sample_kind
            // GetRNGstate
            // PutRNGstate
            // unif_rand
            // R_unif_index
            // norm_rand
            // exp_rand
        }
    }
}
