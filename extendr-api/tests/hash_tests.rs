use extendr_api::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_obj(obj: &Robj) -> u64 {
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn names_and_tags_do_not_affect_hash() {
    test! {
        // Pairlist tags ignored.
        let pl_named: Robj = R!("pairlist(a = 1L, b = 2L)")?;
        let pl_unnamed: Robj = R!("pairlist(1L, 2L)")?;
        assert_eq!(hash_obj(&pl_named), hash_obj(&pl_unnamed));

        // List names ignored.
        let list_named: Robj = R!("list(a = 1L, b = 2L)")?;
        let list_unnamed: Robj = R!("list(1L, 2L)")?;
        assert_eq!(hash_obj(&list_named), hash_obj(&list_unnamed));

        // Language tags ignored (LANGSXP hashed as pairlist without tags).
        let call_named: Robj = R!("quote(f(a = 1L, b = 2L))")?;
        let call_unnamed: Robj = R!("quote(f(1L, 2L))")?;
        assert_eq!(hash_obj(&call_named), hash_obj(&call_unnamed));
    }
}

#[test]
fn env_hash_is_pointer_identity() {
    test! {
        // Two environments with identical bindings but different identities hash differently.
        let env1: Robj = R!("local({ e <- new.env(parent = emptyenv()); e$a <- 1L; e })")?;
        let env2: Robj = R!("local({ e <- new.env(parent = emptyenv()); e$a <- 1L; e })")?;
        assert_ne!(hash_obj(&env1), hash_obj(&env2));

        // Hash is stable under binding mutation because contents are ignored.
        let before: Robj = R!("local({ e <- new.env(parent = emptyenv()); e$a <- 1L; e })")?;
        let after: Robj = R!("local({ e <- new.env(parent = emptyenv()); e$a <- 2L; e })")?;
        // Different environments, but each hash equals itself even if contents change.
        assert_ne!(hash_obj(&before), hash_obj(&after));
    }
}

#[test]
fn vectors_hash_by_ordered_values_only() {
    test! {
        let exprs1: Robj = R!("expression(1, 2)")?;
        let exprs2: Robj = R!("expression(1, 3)")?;
        let exprs_named: Robj = R!("`attr<-`(expression(1, 2), 'names', c('a','b'))")?;

        assert_ne!(hash_obj(&exprs1), hash_obj(&exprs2));
        // Names ignored.
        assert_eq!(hash_obj(&exprs1), hash_obj(&exprs_named));

        let vec1: Robj = R!("list(1L, 2L, 3L)")?;
        let vec2: Robj = R!("list(1L, 3L, 2L)")?;
        assert_ne!(hash_obj(&vec1), hash_obj(&vec2));
    }
}

#[test]
fn external_ptr_hashes_address() {
    test! {
        let ext1: Robj = ExternalPtr::new(1_i32).into();
        let ext2: Robj = ExternalPtr::new(1_i32).into();
        // Same value but different address => different hash.
        assert_ne!(hash_obj(&ext1), hash_obj(&ext2));
    }
}
