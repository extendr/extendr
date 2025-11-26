use extendr_api::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_obj(obj: &Robj) -> u64 {
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn names_and_tags_behavior() {
    test! {
        // Pairlist tags are included.
        let pl_named: Robj = R!("pairlist(a = 1L, b = 2L)")?;
        let pl_unnamed: Robj = R!("pairlist(1L, 2L)")?;
        assert_ne!(hash_obj(&pl_named), hash_obj(&pl_unnamed));

        // List names ignored.
        let list_named: Robj = R!("list(a = 1L, b = 2L)")?;
        let list_unnamed: Robj = R!("list(1L, 2L)")?;
        assert_eq!(hash_obj(&list_named), hash_obj(&list_unnamed));

        // Language tags included (LANGSXP hashed like pairlist with tags).
        let call_named: Robj = R!("quote(f(a = 1L, b = 2L))")?;
        let call_unnamed: Robj = R!("quote(f(1L, 2L))")?;
        assert_ne!(hash_obj(&call_named), hash_obj(&call_unnamed));
    }
}

#[test]
fn env_hash_reflects_bindings() {
    test! {
        // Two environments with identical bindings hash the same.
        let env1: Robj = R!("local({ e <- new.env(parent = emptyenv()); e$a <- 1L; e })")?;
        let env2: Robj = R!("local({ e <- new.env(parent = emptyenv()); e$a <- 1L; e })")?;
        assert_eq!(hash_obj(&env1), hash_obj(&env2));

        // Changing a binding changes the hash.
        let before: Robj = R!("local({ e <- new.env(parent = emptyenv()); e$a <- 1L; e })")?;
        let after: Robj = R!("local({ e <- new.env(parent = emptyenv()); e$a <- 2L; e })")?;
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

#[test]
fn charsxp_and_symsxp_hash_by_text() {
    test! {
        let char_a: Robj = R!("as.name('a')")?; // SYMSXP via name
        let char_a2: Robj = R!("as.name('a')")?;
        let char_b: Robj = R!("as.name('b')")?;
        assert_eq!(hash_obj(&char_a), hash_obj(&char_a2));
        assert_ne!(hash_obj(&char_a), hash_obj(&char_b));

        let charsxp_a: Robj = R!("as.character('a')[1]")?; // CHARSXP
        let charsxp_b: Robj = R!("as.character('b')[1]")?;
        assert_ne!(hash_obj(&charsxp_a), hash_obj(&charsxp_b));
        assert_eq!(hash_obj(&charsxp_a), hash_obj(&charsxp_a));
    }
}

#[test]
fn closures_hash_formals_body_and_env_pointer() {
    test! {
        let clo1: Robj = R!("local({ env <- new.env(parent = emptyenv()); env$f <- function(x) x + 1; env$f })")?;
        let clo2_same: Robj = R!("local({ env <- new.env(parent = emptyenv()); env$f <- function(x) x + 1; env$f })")?;
        let clo_body_diff: Robj = R!("local({ env <- new.env(parent = emptyenv()); env$f <- function(x) x + 2; env$f })")?;
        let clo_formals_diff: Robj = R!("local({ env <- new.env(parent = emptyenv()); env$f <- function(y) y + 1; env$f })")?;

        // Different environments => different hash even if body/formals match.
        assert_ne!(hash_obj(&clo1), hash_obj(&clo2_same));

        // Different body changes hash.
        assert_ne!(hash_obj(&clo1), hash_obj(&clo_body_diff));

        // Different formals changes hash.
        assert_ne!(hash_obj(&clo1), hash_obj(&clo_formals_diff));
    }
}
