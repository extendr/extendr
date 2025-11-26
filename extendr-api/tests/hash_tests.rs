use extendr_api::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_obj(obj: &Robj) -> u64 {
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn env_hash_matches_as_list_and_changes_on_mutation() {
    test! {
        // Use a non-hash environment (pairlist-backed) for stable ordering.
        let env: Robj = R!("local({
            e <- new.env(hash = FALSE, parent = emptyenv());
            e$a <- 1L;
            e$b <- 'x';
            e
        })")?;
        let list_from_env = call!("as.list.environment", env.clone())?;

        let env_hash = hash_obj(&env);
        let list_hash = hash_obj(&list_from_env);
        assert_eq!(env_hash, list_hash);

        // Mutating a binding should alter the hash.
        let env_modified: Robj = R!("local({
            e <- new.env(hash = FALSE, parent = emptyenv());
            e$a <- 1L;
            e$b <- 'y';
            e
        })")?;
        assert_ne!(env_hash, hash_obj(&env_modified));
    }
}

#[test]
fn lang_hashes_like_pairlist_and_differs_on_arguments() {
    test! {
        let lang: Robj = R!("quote(f(a = 1, 2))")?;
        let as_pairlist = call!("as.pairlist", lang.clone())?;

        // Hashing LANGSXP is equivalent to hashing its underlying pairlist.
        assert_eq!(hash_obj(&lang), hash_obj(&as_pairlist));

        let lang_changed: Robj = R!("quote(f(a = 1, 3))")?;
        assert_ne!(hash_obj(&lang), hash_obj(&lang_changed));
    }
}

#[test]
fn expressions_hash_include_length_and_contents() {
    test! {
        let expr_one: Robj = R!("expression(1)")?;
        let expr_two: Robj = R!("expression(1, 2)")?;
        let expr_diff: Robj = R!("expression(2)")?;

        // Length matters.
        assert_ne!(hash_obj(&expr_one), hash_obj(&expr_two));

        // Contents matter even with same length.
        assert_ne!(hash_obj(&expr_one), hash_obj(&expr_diff));

        // Same expression hashes the same.
        assert_eq!(hash_obj(&expr_one), hash_obj(&expr_one));
    }
}
