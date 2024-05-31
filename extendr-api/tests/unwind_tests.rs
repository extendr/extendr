#[allow(unused_imports)]
use std::{
    panic::{self, AssertUnwindSafe},
    ptr,
};

use extendr_api::{ownership, single_threaded};
use extendr_engine::with_r;
use libR_sys::{R_ContinueUnwind, SEXP};
#[allow(unused_imports)]
use libR_sys::{
    R_MakeUnwindCont, R_NilValue, R_UnwindProtect, R_tryCatchError, R_withCallingErrorHandler,
    Rboolean, Rf_PrintValue, Rf_error,
};
use std::cell::RefCell;

#[path = "../src/split.rs"]
mod split;
use split::split_closure;

thread_local! {
    // FIXME: ensure that the number match the number of resources in the test
    static RESOURCE_TOTAL: RefCell<i32> = const { RefCell::new(4) } ;
}

#[derive(Debug)]
struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Dropping resource: {}", self.name);
        RESOURCE_TOTAL.with(|x| x.replace_with(|x| *x - 1));
    }
}

/// Equiv. to converter or rust function that needs to be wrapped
fn inner_function() {
    let _inner_res1 = Resource {
        name: String::from("inner_res1"),
    };
    let _inner_res2 = Resource {
        name: String::from("inner_res2"),
    };
    println!("About to panic inside inner_function");
    panic!("Panic inside inner_function");
}

/// Equiv. to the C-Wrapper
fn outer_function() {
    let _outer_res1 = Resource {
        name: String::from("outer_res1"),
    };
    let _outer_res2 = Resource {
        name: String::from("outer_res2"),
    };

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        inner_function();
    }));

    match result {
        Ok(_) => println!("No panic occurred in inner_function"),
        Err(err) => println!("Caught a panic in outer_function: {:?}", err),
    }
    with_r(|| unsafe {
        let mut f = || -> SEXP {
            Rf_error(c"eror".as_ptr());
            R_NilValue
        };
        let (s, cfn) = split_closure(&mut f);

        let fun = Some(cfn);
        let data = s;
        let mut clean_closure = |jump: Rboolean| {
            println!("anything?");
            dbg!(jump);
            if jump.into() {
                panic!()
            }
        };
        let (cleandata, cleanfun) = split_closure(&mut clean_closure);
        let cleanfun = Some(cleanfun);
        let cont = R_MakeUnwindCont();
        single_threaded(|| {
            R_UnwindProtect(fun, data, cleanfun, cleandata, cont);
            // R_ContinueUnwind(cont);
        })
    });

    println!("Continuing execution in outer_function");
}

#[test]
#[should_panic]
fn unwinding_rust() {
    outer_function();
    // actually, the program does not continue after this...
    println!("Program continues execution after outer_function");
    assert_eq!(RESOURCE_TOTAL.take(), 0);
    let ownership_lock = ownership::OWNERSHIP.lock();
    if let Ok(ownership) = ownership_lock {
        assert_eq!(ownership.total_protected(), 0);
    }
}

#[test]
fn unwinding_rust_2() {
    let outer_function_result = panic::catch_unwind(AssertUnwindSafe(|| {
        outer_function();
    }));
    match outer_function_result {
        Ok(_) => {}
        Err(_) => {
            dbg!("it did panic");
        }
    }
    println!("Program continues execution after outer_function");
    assert_eq!(RESOURCE_TOTAL.take(), 0);
    let ownership_lock = ownership::OWNERSHIP.lock();
    if let Ok(ownership) = ownership_lock {
        assert_eq!(ownership.total_protected(), 0);
    }
}

// Does this run before tests?
// fn main() {
//     outer_function();
//     println!("Program continues execution after outer_function");
//     assert_eq!(RESOURCE_TOTAL.take(), 0);
// }

#[test]
fn test_move_ownership_idea() {
    fn cwrapper(a: Resource, b: Resource) -> () {
        fn scope_wrapper(a: Resource, b: Resource) -> () {
            dbg!(a, b);
        }
        scope_wrapper(a, b);
        dbg!("resources must have dropped prior to this point");
    }

    let a = Resource {
        name: "Alice".into(),
    };
    let b = Resource { name: "Bob".into() };

    cwrapper(a, b);
}

#[test]
fn test_move_ownership_idea_closure() {
    fn cwrapper(a: Resource, b: Resource, c: i32) -> () {
        let result = (move || -> Result<(), Box<dyn std::error::Error>> {
            let (_a, _b, _c) = (a, b, c);
            Ok(())
        })();

        dbg!("resources must have dropped prior to this point");
    }

    let a = Resource {
        name: "Alice".into(),
    };
    let b = Resource { name: "Bob".into() };

    cwrapper(a, b, 42);
}
