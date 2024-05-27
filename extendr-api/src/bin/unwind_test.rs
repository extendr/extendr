use std::panic::{self, AssertUnwindSafe};

struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Dropping resource: {}", self.name);
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

    println!("Continuing execution in outer_function");
}

fn main() {
    outer_function();
    println!("Program continues execution after outer_function");
}
