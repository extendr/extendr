use extendr_api::prelude::*;

#[test]
fn test_allocation() {
    test! {
        const COUNT: u64 = 2_000_000u64;
        let mut data: Vec<Robj> = vec![];
        for i in 1u64..COUNT {
            // if i % 1000 == 0 { println!("{}", i); }
            data.push(i.into());
        }

        for (ix, v) in data.iter().enumerate() {
            assert_eq!(Some((ix + 1) as f64), v.as_real())
        }

        let mut obj: Robj = List::from_names_and_values(&["A"], vec![data]).into();
        obj.set_attrib(
            row_names_symbol(),
            (1i32..=COUNT as i32).collect_robj(),
        )?;
        obj.set_class(&["data.frame"])?;
    }
}
