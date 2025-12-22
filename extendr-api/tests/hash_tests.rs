use extendr_api::prelude::*;
use extendr_engine::with_r_result;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

fn hash_any<T: Hash>(obj: &T) -> u64 {
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
        assert_ne!(hash_any(&pl_named), hash_any(&pl_unnamed));

        // List names ignored.
        let list_named: Robj = R!("list(a = 1L, b = 2L)")?;
        let list_unnamed: Robj = R!("list(1L, 2L)")?;
        assert_eq!(hash_any(&list_named), hash_any(&list_unnamed));

        // Language tags included (LANGSXP hashed like pairlist with tags).
        let call_named: Robj = R!("quote(f(a = 1L, b = 2L))")?;
        let call_unnamed: Robj = R!("quote(f(1L, 2L))")?;
        assert_ne!(hash_any(&call_named), hash_any(&call_unnamed));
    }
}

#[test]
fn env_hash_by_identity() {
    test! {
        // these are two enviornments that are identical in their contents
        // but then
        // Two different environments hash differently, even with identical bindings.
        // This matches R's identical() behavior which compares environments by pointer.
        let env1: Robj = R!("local({ e <- new.env(parent = emptyenv()); e$a <- 1L; e })")?;
        let env2: Robj = R!("local({ e <- new.env(parent = emptyenv()); e$a <- 1L; e })")?;
        assert_ne!(hash_any(&env1), hash_any(&env2));

        let env: Robj = R!("local({ e <- new.env(parent = emptyenv()); e$a <- 1L; e })")?;
        assert_eq!(hash_any(&env), hash_any(&env));

    }
}

#[test]
fn vectors_hash_by_ordered_values_only() {
    test! {
        let exprs1: Robj = R!("expression(1, 2)")?;
        let exprs2: Robj = R!("expression(1, 3)")?;
        let exprs_named: Robj = R!("`attr<-`(expression(1, 2), 'names', c('a','b'))")?;

        assert_ne!(hash_any(&exprs1), hash_any(&exprs2));
        // Names ignored.
        assert_eq!(hash_any(&exprs1), hash_any(&exprs_named));

        let vec1: Robj = R!("list(1L, 2L, 3L)")?;
        let vec2: Robj = R!("list(1L, 3L, 2L)")?;
        assert_ne!(hash_any(&vec1), hash_any(&vec2));
    }
}

#[test]
fn external_ptr_hashes_address() {
    test! {
        let ext1: Robj = ExternalPtr::new(1_i32).into();
        let ext2: Robj = ExternalPtr::new(1_i32).into();
        // Same value but different address => different hash.
        assert_ne!(hash_any(&ext1), hash_any(&ext2));
    }
}

#[test]
fn charsxp_and_symsxp_hash_by_text() {
    test! {
        let char_a: Robj = R!("as.name('a')")?; // SYMSXP via name
        let char_a2: Robj = R!("as.name('a')")?;
        let char_b: Robj = R!("as.name('b')")?;
        assert_eq!(hash_any(&char_a), hash_any(&char_a2));
        assert_ne!(hash_any(&char_a), hash_any(&char_b));

        let charsxp_a: Robj = R!("as.character('a')[1]")?; // CHARSXP
        let charsxp_b: Robj = R!("as.character('b')[1]")?;
        assert_ne!(hash_any(&charsxp_a), hash_any(&charsxp_b));
        assert_eq!(hash_any(&charsxp_a), hash_any(&charsxp_a));
    }
}

#[test]
fn closures_hash_formals_body_and_env_pointer() {
    test! {
        let clo1: Robj = R!("local({ env <- new.env(parent = emptyenv()); env$f <- function(x) x + 1; env$f })")?;
        let clo2_same: Robj = R!("local({ env <- new.env(parent = emptyenv()); env$f <- function(x) x + 1; env$f })")?;
        let clo_body_diff: Robj = R!("local({ env <- new.env(parent = emptyenv()); env$f <- function(x) x + 2; env$f })")?;
        let clo_formals_diff: Robj = R!("local({ env <- new.env(parent = emptyenv()); env$f <- function(y) y + 1; env$f })")?;

        assert_ne!(hash_any(&clo1), hash_any(&clo2_same));
        assert_ne!(hash_any(&clo1), hash_any(&clo_body_diff));
        assert_ne!(hash_any(&clo1), hash_any(&clo_formals_diff));
    }
}

#[test]
fn hash_envs_fail() -> Result<()> {
    with_r_result(|| {
        let env1: Robj = R!("local({ e <- new.env(parent = emptyenv()); e$a <- 1L; e })")?;
        let env2: Robj = R!("local({ e <- new.env(parent = emptyenv()); e$a <- 1L; e })")?;

        let mut set = HashSet::new();
        set.insert(env1);
        assert!(set.insert(env2));
        Ok(())
    })
}

#[test]
fn wrapper_types_in_hashset() -> Result<()> {
    with_r_result(|| {
        use extendr_api::wrapper::*;

        let list1 = List::from_values([1, 2, 3]);
        let list2 = List::from_values([1, 2, 3]);
        let mut set = HashSet::new();
        set.insert(list1.clone());
        // Same values hash the same
        assert!(!set.insert(list2));

        // Test Doubles
        let doubles1 = Doubles::from_values([1.0, 2.0, 3.0]);
        let doubles2 = Doubles::from_values([1.0, 2.0, 3.0]);
        let mut set = HashSet::new();
        set.insert(doubles1.clone());
        assert!(!set.insert(doubles2));

        // Test Integers
        let ints1 = Integers::from_values([1, 2, 3]);
        let ints2 = Integers::from_values([1, 2, 3]);
        let mut set = HashSet::new();
        set.insert(ints1.clone());
        assert!(!set.insert(ints2));

        // Test Strings
        let strs1 = Strings::from_values(["a", "b", "c"]);
        let strs2 = Strings::from_values(["a", "b", "c"]);
        let mut set = HashSet::new();
        set.insert(strs1.clone());
        assert!(!set.insert(strs2));

        // Test Logicals
        let logicals1 = Logicals::from_values([true, false, true]);
        let logicals2 = Logicals::from_values([true, false, true]);
        let mut set = HashSet::new();
        set.insert(logicals1.clone());
        assert!(!set.insert(logicals2));

        let complexes1 = Complexes::from_values([c64::new(1.0, 2.0), c64::new(3.0, 4.0)]);
        let complexes2 = Complexes::from_values([c64::new(1.0, 2.0), c64::new(3.0, 4.0)]);
        let mut set = HashSet::new();
        set.insert(complexes1.clone());
        assert!(!set.insert(complexes2));

        let raw1 = Raw::from_bytes(&[1u8, 2u8, 3u8]);
        let raw2 = Raw::from_bytes(&[1u8, 2u8, 3u8]);
        let mut set = HashSet::new();
        set.insert(raw1.clone());
        assert!(!set.insert(raw2));

        Ok(())
    })
}

#[test]
fn scalar_types_in_hashset() -> Result<()> {
    with_r_result(|| {
        use extendr_api::prelude::*;

        let mut set = HashSet::new();
        set.insert(Rint::from(42));
        assert!(!set.insert(Rint::from(42)));
        assert!(set.insert(Rint::from(43)));

        // Test Rfloat
        let mut set = HashSet::new();
        set.insert(Rfloat::from(42.0));
        assert!(!set.insert(Rfloat::from(42.0)));
        assert!(set.insert(Rfloat::from(2.71)));

        // Test Rbool
        let mut set = HashSet::new();
        set.insert(Rbool::from(true));
        assert!(!set.insert(Rbool::from(true)));
        assert!(set.insert(Rbool::from(false)));

        // Test Rstr
        let mut set = HashSet::new();
        set.insert(Rstr::from("hello"));
        assert!(!set.insert(Rstr::from("hello")));
        assert!(set.insert(Rstr::from("world")));

        Ok(())
    })
}

#[test]
fn wrapper_hash_consistency_with_robj() -> Result<()> {
    // wrappers should match the created Robj
    with_r_result(|| {
        use extendr_api::wrapper::*;

        let list = List::from_values([1, 2, 3]);
        assert_eq!(hash_any(&list), hash_any(&Robj::from(list.clone())));

        let doubles = Doubles::from_values([1.0, 2.0, 3.0]);
        assert_eq!(hash_any(&doubles), hash_any(&Robj::from(doubles.clone())));

        let ints = Integers::from_values([1, 2, 3]);
        assert_eq!(hash_any(&ints), hash_any(&Robj::from(ints.clone())));

        let strs = Strings::from_values(["a", "b", "c"]);
        assert_eq!(hash_any(&strs), hash_any(&Robj::from(strs.clone())));

        let logicals = Logicals::from_values([true, false, true]);
        assert_eq!(hash_any(&logicals), hash_any(&Robj::from(logicals.clone())));

        let complexes = Complexes::from_values([c64::new(1.0, 2.0), c64::new(3.0, 4.0)]);
        assert_eq!(
            hash_any(&complexes),
            hash_any(&Robj::from(complexes.clone()))
        );

        let raw = Raw::from_bytes(&[1u8, 2u8, 3u8]);
        assert_eq!(hash_any(&raw), hash_any(&Robj::from(raw.clone())));

        Ok(())
    })
}

#[test]
fn rstr_hash_consistency_with_robj() -> Result<()> {
    with_r_result(|| {
        use extendr_api::prelude::*;

        let rstr = Rstr::from("hello");
        assert_eq!(hash_any(&rstr), hash_any(&Robj::from(rstr.clone())));

        Ok(())
    })
}

#[test]
fn test_pairlist_hash() -> Result<()> {
    with_r_result(|| {
        use extendr_api::prelude::*;

        assert_eq!(hash_any(&Pairlist::new()), hash_any(&Pairlist::new()));

        let pairs = (0..100).map(|i| (format!("n{}", i), i));
        let pairlist = Pairlist::from_pairs(pairs.clone());
        let p2 = Pairlist::from_pairs(pairs);
        assert_eq!(hash_any(&pairlist), hash_any(&p2));
        Ok(())
    })
}

#[test]
fn test_function_hash() -> Result<()> {
    with_r_result(|| {
        use extendr_api::prelude::*;
        // Closures are functions.
        let expr = R!("function(a = 1, b) {c <- a + b}")?;
        let func = expr.as_function().unwrap();
        let f2 = expr.as_function().unwrap();
        assert_eq!(hash_any(&func), hash_any(&f2));
        Ok(())
    })
}

#[test]
fn test_symbol_hash() -> Result<()> {
    with_r_result(|| {
        use extendr_api::prelude::*;
        let wombat = sym!(wombat);
        let wombat2 = sym!(wombat);
        assert_eq!(hash_any(&wombat), hash_any(&wombat2));
        Ok(())
    })
}

#[test]
fn test_language_hash() -> Result<()> {
    with_r_result(|| {
        use extendr_api::prelude::*;
        let call_to_c = lang!("c", 1., 2., 3.).as_language().unwrap();

        let call_to_c2 = call_to_c.clone();
        assert_eq!(hash_any(&call_to_c), hash_any(&call_to_c2));
        Ok(())
    })
}
