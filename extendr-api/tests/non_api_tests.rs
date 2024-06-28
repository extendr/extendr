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

#[cfg(test)]
fn non_api_rinternals_promise() {
    with_r(|| {
        let iris_dataframe = global_env()
            .find_var(sym!(iris))
            .unwrap()
            .eval_promise()
            .unwrap();
        assert_eq!(iris_dataframe.is_frame(), true);
        assert_eq!(iris_dataframe.len(), 5);

        // Note: this may crash on some versions of windows which don't support unwinding.
        //assert_eq!(global_env().find_var(sym!(imnotasymbol)), None);
    });
}

#[cfg(test)]
fn non_api_getsexp_rtype() {
    with_r(|| {
        assert_eq!(r!(Primitive::from_string("if")).rtype(), Rtype::Special);
        assert_eq!(r!(Primitive::from_string("+")).rtype(), Rtype::Builtin);
    });
}

#[cfg(test)]
fn non_api_rmacros() {
    with_r(|| {
        // The "iris" dataset is a dataframe.
        assert_eq!(global!(iris)?.is_frame(), true);
    });
}

/// In `extendr-api/lib.rs` there was a section with this
///
/// > You can call R functions and primitives using the [call!] macro.
#[cfg(test)]
fn non_api_lib_README() {
    with_r(|| {
        // As one R! macro call
        let confint1 = R!("confint(lm(weight ~ group - 1, PlantGrowth))")?;

        // As many parameterized calls.
        let mut formula = lang!("~", sym!(weight), lang!("-", sym!(group), 1.0));
        formula.set_class(["formula"])?;
        let plant_growth = global!(PlantGrowth)?;
        let model = call!("lm", formula, plant_growth)?;
        let confint2 = call!("confint", model)?;

        assert_eq!(confint1.as_real_vector(), confint2.as_real_vector());
    });
}

#[cfg(test)]
fn non_api_base_env() {
    with_r(|| {
        global_env().set_local(sym!(x), "hello");
        assert_eq!(
            base_env().local(sym!(+)),
            Ok(r!(Primitive::from_string("+")))
        );
    });
}
