#[allow(unused_imports)]
use std::{
    panic::{self, AssertUnwindSafe},
    ptr,
};

use extendr_api::ownership;
use extendr_engine::with_r;
use libR_sys::{R_ContinueUnwind, SEXP};
#[allow(unused_imports)]
use libR_sys::{
    R_MakeUnwindCont, R_NilValue, R_UnwindProtect, R_tryCatchError, R_withCallingErrorHandler,
    Rboolean, Rf_PrintValue, Rf_error,
};
use split::split_closure;
use std::cell::RefCell;

#[path = "../split.rs"]
mod split;

thread_local! {
    static RESOURCE_TOTAL: RefCell<i32> = const { RefCell::new(4) } ;
}

struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Dropping resource: {}", self.name);
        RESOURCE_TOTAL.with(|x| x.replace_with(|x| *x - 1));
    }
}

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

        // I guess this is the standard
        // Rf_error(c"eror".as_ptr());

        // doesn't work
        // R_withCallingErrorHandler(Some(cfn), s, None, ptr::null_mut());

        // this "works"?
        // let result = R_tryCatchError(Some(cfn), s, None, ptr::null_mut());
        // Rf_PrintValue(result);

        let fun = Some(cfn);
        let data = s;
        let mut clean_closure = |jump: Rboolean| unsafe {
            println!("anything?");
            dbg!(jump);
            if jump.into() {
                panic!()
            }
        };
        let (cleandata, cleanfun) = split_closure(&mut clean_closure);
        let cleanfun = Some(cleanfun);
        // DOESN'T WORK
        // let cleanfun = None;
        // let cleandata = ptr::null_mut();
        // let cont = R_NilValue; // doesn't work at all
        let cont = R_MakeUnwindCont();
        R_UnwindProtect(fun, data, cleanfun, cleandata, cont);
        R_ContinueUnwind(cont);

        // print how many active objects
        // let ownership_lock = ownership::OWNERSHIP.lock();
        // if let Ok(ownership) = ownership_lock {
        //     dbg!(ownership.total_protected());
        // }
    });

    println!("Continuing execution in outer_function");
}

#[test]
#[should_panic]
fn unwinding_rust() {
    outer_function();
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

fn main() {
    outer_function();
    println!("Program continues execution after outer_function");
    assert_eq!(RESOURCE_TOTAL.take(), 0);
}
