#[test]
fn dataframe_tests() {
    use extendr_api::prelude::*;
    use extendr_api::wrapper::Dataframe;
    test! {
        let x = [(1_i32, 1.0_f64), (2, 2.0), (3, 3.0)];

        let df : Dataframe<(i32, f64)> = Dataframe::try_from_values(x).unwrap();
        let df2 = data_frame!(a=[1, 2, 3], b=[1., 2., 3.]);
        assert_eq!(df.as_robj(), &df2);
    }
}
