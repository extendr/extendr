use extendr_api::prelude::*;

#[test]
#[cfg(feature = "non-api")]
fn formula_test() {
    test! {
        // As one R! macro call
        let confint1 = R!("confint(lm(weight ~ group - 1, PlantGrowth))")?;

        // As many parameterized calls.
        let mut formula = lang!("~", sym!(weight), lang!("-", sym!(group), 1.0));
        formula.set_class(["formula"])?;
        let plant_growth = global!(PlantGrowth)?;
        let model = call!("lm", formula, plant_growth)?;
        let confint2 = call!("confint", model)?;

        assert_eq!(confint1.as_real_vector(), confint2.as_real_vector());
    }
}

#[test]
fn special_test() {
    test! {
        assert_eq!(call!("`+`", 1, 2)?, r!(3));
        let add = R!("`+`")?;
        assert!(add.is_primitive());
        assert_eq!(add.call(pairlist!(1, 2))?, r!(3));
    }
}
