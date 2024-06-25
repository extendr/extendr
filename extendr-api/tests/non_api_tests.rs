use extendr_api::prelude::*;
use extendr_engine::with_r;

#[cfg(test)]
fn non_api_promise() {
    with_r(|| {
        let special = r!(Primitive::from_string("if"));
        let builtin = r!(Primitive::from_string("+"));
    });
}

#[cfg(test)]
fn environment() {
    with_r(|| {
        let names_and_values = (0..100).map(|i| (format!("n{}", i), i));
        let env = Environment::from_pairs(global_env(), names_and_values);
        let robj = r!(env);
        let names_and_values = robj.as_environment().unwrap().iter().collect::<Vec<_>>();
        assert_eq!(names_and_values.len(), 100);

        let small_env = Environment::new_with_capacity(global_env(), 1);
        small_env.set_local(sym!(x), 1);
        let names_and_values = small_env
            .as_environment()
            .unwrap()
            .iter()
            .collect::<Vec<_>>();
        assert_eq!(names_and_values, vec![("x", r!(1))]);

        let large_env = Environment::new_with_capacity(global_env(), 1000);
        large_env.set_local(sym!(x), 1);
        let names_and_values = large_env
            .as_environment()
            .unwrap()
            .iter()
            .collect::<Vec<_>>();
        assert_eq!(names_and_values, vec![("x", r!(1))]);
    });
}
