#![cfg(test)]
use extendr_api::prelude::*;
use extendr_engine::with_r;
use std::result::Result;

#[test]
fn non_api_promise() -> Result<(), Box<dyn std::error::Error>> {
    with_r(|| {
        let _special = r!(Primitive::from_string("if"));
        let _builtin = r!(Primitive::from_string("+"));
        Ok(())
    })
}

#[test]
fn environment() -> Result<(), Box<dyn std::error::Error>> {
    with_r(|| {
        let names_and_values = (0..100).map(|i| (format!("n{}", i), i));
        let env = Environment::from_pairs(Environment::global(), names_and_values);
        let robj = r!(env);
        let names_and_values = robj.as_environment().unwrap().iter().collect::<Vec<_>>();
        assert_eq!(names_and_values.len(), 100);

        let small_env = Environment::new_with_capacity(Environment::global(), 1);
        small_env.set_local(sym!(x), 1);
        let names_and_values = small_env
            .as_environment()
            .unwrap()
            .iter()
            .collect::<Vec<_>>();
        assert_eq!(names_and_values, vec![("x", r!(1))]);

        let large_env = Environment::new_with_capacity(Environment::global(), 1000);
        large_env.set_local(sym!(x), 1);
        let names_and_values = large_env
            .as_environment()
            .unwrap()
            .iter()
            .collect::<Vec<_>>();
        assert_eq!(names_and_values, vec![("x", r!(1))]);
        Ok(())
    })
}

#[test]
fn non_api_rinternals_promise() -> Result<(), Box<dyn std::error::Error>> {
    with_r(|| {
        let iris_dataframe = Environment::global()
            .find_var(sym!(iris))
            .unwrap()
            .eval_promise()
            .unwrap();
        assert!(iris_dataframe.is_frame());
        assert_eq!(iris_dataframe.len(), 5);

        // Note: this may crash on some versions of windows which don't support unwinding.
        //assert_eq!(global_env().find_var(sym!(imnotasymbol)), None);
        Ok(())
    })
}

#[test]
fn non_api_getsexp_rtype() -> Result<(), Box<dyn std::error::Error>> {
    with_r(|| {
        assert_eq!(r!(Primitive::from_string("if")).rtype(), Rtype::Special);
        assert_eq!(r!(Primitive::from_string("+")).rtype(), Rtype::Builtin);
        Ok(())
    })
}

#[test]
fn non_api_base_env() -> Result<(), Box<dyn std::error::Error>> {
    with_r(|| {
        Environment::global().set_local(sym!(x), "hello");
        assert_eq!(
            Environment::global().local(sym!(+)),
            Ok(r!(Primitive::from_string("+")))
        );
        Ok(())
    })
}
