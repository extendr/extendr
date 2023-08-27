use extendr_api::prelude::*;

#[test]
fn formula_test() {
    with_r(|| {
        // As one R! macro call
        let confint1 = R!("confint(lm(weight ~ group - 1, PlantGrowth))").unwrap();

        // As many parameterized calls.
        let formula = lang!("~", sym!(weight), lang!("-", sym!(group), 1.0)).set_class(["formula"]).unwrap();
        let plant_growth = global!(PlantGrowth).unwrap();
        let model = call!("lm", formula, plant_growth).unwrap();
        let confint2 = call!("confint", model).unwrap();

        assert_eq!(confint1.as_real_vector(), confint2.as_real_vector());
    });
}

#[test]
fn special_test() {
    with_r(|| {
        assert_eq!(call!("`+`", 1, 2).unwrap(), r!(3));
        let add = R!("`+`").unwrap();
        assert!(add.is_primitive());
        assert_eq!(add.call(pairlist!(1, 2)).unwrap(), r!(3));
    });
}
