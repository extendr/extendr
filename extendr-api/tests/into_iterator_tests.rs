use extendr_api::prelude::*;
use extendr_engine::with_r;

#[test]
fn iterating_unamed_list() {
    with_r(|| {
        let unamed_list = List::from_values([1, 2, 3, 42, 100]);
        assert!(unamed_list.names().is_none());
        assert_eq!(unamed_list.iter().len(), 5);

        let empty_strings = unamed_list.iter().map(|(si, _)| si).collect::<Strings>();
        let na_strs = (0..5).into_iter().map(|_| Rstr::na()).collect::<Strings>();
        assert_eq!(empty_strings, na_strs);

        let unamed_list = List::from_values::<[i32; 0]>([]);
        assert_eq!(unamed_list.iter().len(), 0);

        assert!(unamed_list.iter().collect::<Vec<_>>().is_empty());
    });
}
