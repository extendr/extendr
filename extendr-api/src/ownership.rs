//! Maintain ownership of R objects.
//!
//! This provides the functions [protect] and [unprotect].
//! A single preserved vector holds ownership of all protected objects.
//!
//! Objects are reference counted, so multiple calls are possible,
//! unlike `R_PreserveObject`.
//!
//! This module exports two functions, `protect(sexp)` and `unprotect(sexp)`.

use once_cell::sync::Lazy;
use std::collections::hash_map::{Entry, HashMap};
use std::sync::Mutex;

use libR_sys::{
    R_NilValue, R_PreserveObject, R_ReleaseObject, R_xlen_t, Rf_allocVector, Rf_protect,
    Rf_unprotect, LENGTH, SET_VECTOR_ELT, SEXP, VECSXP, VECTOR_ELT,
};

static OWNERSHIP: Lazy<Mutex<Ownership>> = Lazy::new(|| Mutex::new(Ownership::new()));

pub(crate) unsafe fn protect(sexp: SEXP) {
    let mut own = OWNERSHIP.lock().expect("protect failed");
    own.protect(sexp);
}

pub(crate) unsafe fn unprotect(sexp: SEXP) {
    let mut own = OWNERSHIP.lock().expect("unprotect failed");
    own.unprotect(sexp);
}

pub const INITIAL_PRESERVATION_SIZE: usize = 100000;
pub const EXTRA_PRESERVATION_SIZE: usize = 100000;

struct Object {
    refcount: usize,
    index: usize,
}

// A reference counted object with an index in the preservation vector.
struct Ownership {
    // A growable vector containing all owned objects.
    preservation: usize,

    // An incrementing count of objects through the vector.
    cur_index: usize,

    // The size of the vector.
    max_index: usize,

    // A hash map from SEXP address to object.
    objects: HashMap<usize, Object>,
}

impl Ownership {
    fn new() -> Self {
        unsafe {
            let preservation = Rf_allocVector(VECSXP, INITIAL_PRESERVATION_SIZE as R_xlen_t);
            R_PreserveObject(preservation);
            Ownership {
                preservation: preservation as usize,
                cur_index: 0,
                max_index: INITIAL_PRESERVATION_SIZE,
                objects: HashMap::with_capacity(INITIAL_PRESERVATION_SIZE),
            }
        }
    }

    unsafe fn protect(&mut self, sexp: SEXP) {
        Rf_protect(sexp);

        if self.cur_index == self.max_index {
            self.garbage_collect();
        }

        let sexp_usize = sexp as usize;
        let Ownership {
            ref mut preservation,
            ref mut cur_index,
            ref mut max_index,
            ref mut objects,
        } = *self;

        let mut entry = objects.entry(sexp_usize);
        let preservation_sexp = *preservation as SEXP;
        match entry {
            Entry::Occupied(ref mut occupied) => {
                if occupied.get().refcount == 0 {
                    // Address re-used - re-set the sexp.
                    SET_VECTOR_ELT(preservation_sexp, occupied.get().index as R_xlen_t, sexp);
                }
                occupied.get_mut().refcount += 1;
            }
            Entry::Vacant(vacant) => {
                let index = *cur_index;
                SET_VECTOR_ELT(preservation_sexp, index as R_xlen_t, sexp);
                *cur_index += 1;
                assert!(index != *max_index);
                let refcount = 1;
                vacant.insert(Object { refcount, index });
            }
        }

        Rf_unprotect(1);
    }

    pub unsafe fn unprotect(&mut self, sexp: SEXP) {
        let sexp_usize = sexp as usize;
        let Ownership {
            preservation,
            cur_index: _,
            max_index: _,
            ref mut objects,
        } = *self;

        let mut entry = objects.entry(sexp_usize);
        match entry {
            Entry::Occupied(ref mut occupied) => {
                let object = occupied.get_mut();
                if object.refcount == 0 {
                    panic!("Attempt to unprotect an already unprotected object.")
                } else {
                    object.refcount -= 1;
                    if object.refcount == 0 {
                        // Clear the preservation vector, but keep the hash table entry.
                        // It is hard to clear the hash table entry here because we don't
                        // have a ref to objects anymore and it is faster to clear them up en-masse.
                        let preservation_sexp = preservation as SEXP;
                        SET_VECTOR_ELT(preservation_sexp, object.index as R_xlen_t, R_NilValue);
                    }
                }
            }
            Entry::Vacant(_) => {
                panic!("Attempt to unprotect a never protected object.")
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn ref_count(&mut self, sexp: SEXP) -> usize {
        let Ownership {
            preservation: _,
            cur_index: _,
            max_index: _,
            ref mut objects,
        } = *self;

        let sexp_usize = sexp as usize;
        let mut entry = objects.entry(sexp_usize);
        match entry {
            Entry::Occupied(ref mut occupied) => occupied.get().refcount,
            Entry::Vacant(_) => 0,
        }
    }

    // Garbage collect the tracking structures.
    unsafe fn garbage_collect(&mut self) {
        // println!("garbage_collect {} {}", self.cur_index, self.max_index);
        let new_size = self.cur_index * 2 + EXTRA_PRESERVATION_SIZE;
        let new_sexp = Rf_allocVector(VECSXP, new_size as R_xlen_t);
        R_PreserveObject(new_sexp);
        let old_sexp = self.preservation as SEXP;

        let mut new_objects = HashMap::with_capacity(new_size);

        // copy non-null elements to new vector and hashmap.
        let mut j = 0;
        for (addr, object) in self.objects.iter() {
            if object.refcount != 0 {
                SET_VECTOR_ELT(new_sexp, j as R_xlen_t, *addr as SEXP);
                new_objects.insert(
                    *addr,
                    Object {
                        refcount: object.refcount,
                        index: j,
                    },
                );
                j += 1;
            }
        }
        // println!("j={}", j);

        R_ReleaseObject(old_sexp);
        self.preservation = new_sexp as usize;
        self.cur_index = j;
        self.max_index = new_size;
        self.objects = new_objects;
    }

    // Check the consistency of the model.
    #[allow(dead_code)]
    unsafe fn check_objects(&mut self) {
        let preservation_sexp = self.preservation as SEXP;
        assert_eq!(self.max_index, LENGTH(preservation_sexp) as usize);

        // println!("\ncheck");

        for (addr, object) in self.objects.iter() {
            assert!(object.index < self.max_index);
            let elt = VECTOR_ELT(preservation_sexp, object.index as R_xlen_t);
            // println!(
            //     "refcount={:?} index={:?} elt={:?}",
            //     object.refcount, object.index, elt
            // );
            if object.refcount != 0 {
                // A non-zero refcount implies the object is in the vector.
                assert_eq!(elt, *addr as SEXP);
            } else {
                // A zero refcount implies the object is NULL in the vector.
                assert_eq!(elt, R_NilValue);
            }
        }

        // println!("check 2");
        for i in 0..self.max_index {
            let elt = VECTOR_ELT(preservation_sexp, i as R_xlen_t);
            if elt == R_NilValue {
                assert_eq!(self.ref_count(elt), 0);
            } else {
                assert!(self.ref_count(elt) != 0);
            }
        }
        // println!("/check");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    use libR_sys::{Rf_ScalarInteger, Rf_protect, Rf_unprotect};

    #[test]
    fn basic_test() {
        test! {
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
        }
    }

    #[test]
    fn collection_test() {
        test! {
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

                let sexps = (0..test_size).map(|i| {
                    let sexp = Rf_ScalarInteger(1);
                    SET_VECTOR_ELT(sexp_pres, i as R_xlen_t, sexp);
                    sexp
                }).collect::<Vec<_>>();

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
        }
    }
}
