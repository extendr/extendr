use extendr_api::prelude::*;
use extendr_engine::with_r;

#[test]
fn iterating_unamed_list() {
    with_r(|| {
        let unamed_list = List::from_values([1, 2, 3, 42, 100]);
        assert_eq!(unamed_list.iter().len(), 5);
        let unamed_list = List::from_values::<[i32; 0]>([]);
        assert_eq!(unamed_list.iter().len(), 0);

        dbg!(unamed_list.iter().collect::<Vec<_>>());
    });
}
