
use libR_sys::*;


#[no_mangle]
pub extern fn hello_wrapper() -> SEXP {
    hello();
    unsafe{R_NilValue}
}

fn hello() {
    println!("hello");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
