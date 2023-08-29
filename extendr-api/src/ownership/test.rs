use super::*;
use crate::*;
use extendr_engine::with_r;
use libR_sys::{Rf_ScalarInteger, Rf_protect, Rf_unprotect};

#[test]
fn basic_test() {
    with_r(|| {
        single_threaded(|| unsafe {
            {
                let mut own = OWNERSHIP.lock().expect("lock failed");
                own.check_objects();
            }

            let sexp1 = Rf_protect(Rf_ScalarInteger(1));
            let sexp2 = Rf_protect(Rf_ScalarInteger(2));
            protect(sexp1);
            {
                let mut own = OWNERSHIP.lock().expect("lock failed");
                own.check_objects();
                assert_eq!(own.ref_count(sexp1), 1);
                assert_eq!(own.ref_count(sexp2), 0);
            }

            protect(sexp1);
            {
                let mut own = OWNERSHIP.lock().expect("lock failed");
                own.check_objects();
                assert_eq!(own.ref_count(sexp1), 2);
                assert_eq!(own.ref_count(sexp2), 0);
            }

            unprotect(sexp1);
            {
                let mut own = OWNERSHIP.lock().expect("lock failed");
                own.check_objects();
                assert_eq!(own.ref_count(sexp1), 1);
                assert_eq!(own.ref_count(sexp2), 0);
            }

            unprotect(sexp1);
            {
                let mut own = OWNERSHIP.lock().expect("lock failed");
                own.check_objects();
                assert_eq!(own.ref_count(sexp1), 0);
                assert_eq!(own.ref_count(sexp2), 0);
            }

            protect(sexp2);
            {
                let mut own = OWNERSHIP.lock().expect("lock failed");
                own.check_objects();
                assert_eq!(own.ref_count(sexp1), 0);
                assert_eq!(own.ref_count(sexp2), 1);
            }

            protect(sexp1);
            {
                let mut own = OWNERSHIP.lock().expect("lock failed");
                own.check_objects();
                assert_eq!(own.ref_count(sexp1), 1);
                assert_eq!(own.ref_count(sexp2), 1);
            }
            Rf_unprotect(2);
        });
    });
}

#[test]
fn collection_test() {
    with_r(|| {
        single_threaded(|| unsafe {
            {
                let mut own = OWNERSHIP.lock().expect("protect failed");
                own.check_objects();
            }

            // Force a garbage collect.
            let test_size = INITIAL_PRESERVATION_SIZE + EXTRA_PRESERVATION_SIZE * 5;

            // Make some test objects.
            let sexp_pres = Rf_allocVector(VECSXP, test_size as R_xlen_t);
            Rf_protect(sexp_pres);

            let sexps = (0..test_size)
                .map(|i| {
                    let sexp = Rf_ScalarInteger(1);
                    SET_VECTOR_ELT(sexp_pres, i as R_xlen_t, sexp);
                    sexp
                })
                .collect::<Vec<_>>();

            for (i, sexp) in sexps.iter().enumerate() {
                protect(*sexp);
                if i % 2 == 0 {
                    unprotect(*sexp);
                }
            }

            {
                let mut own = OWNERSHIP.lock().expect("protect failed");
                own.check_objects();
                own.garbage_collect();
                own.check_objects();
            }

            Rf_unprotect(1);
        });
    });
}
