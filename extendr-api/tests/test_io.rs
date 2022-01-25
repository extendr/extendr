use ::extendr_api::io::Load;

#[test]
fn test_save() {
    use extendr_api::{io::PstreamFormat, io::Save, test, Result, Robj};
    test! {
        let mut w = Vec::new();
        Robj::from(1).to_writer(&mut w, PstreamFormat::AsciiFormat, 3, None)?;
        assert!(w[0] == b'A');

        let mut w = Vec::new();
        Robj::from(1).to_writer(&mut w, PstreamFormat::BinaryFormat, 3, None)?;
        assert!(w[0] == b'B');

        // let path : std::path::PathBuf = "/tmp/1".into();
        // Robj::from(1).save(&path, PstreamFormat::AsciiFormat, 3, None)?;
        // let s = std::fs::read(path).unwrap();
        // assert!(s.starts_with(b"A\n"));
    }
}

#[test]
fn test_load() {
    use extendr_api::{io::PstreamFormat, test, Result, Robj};
    test! {
        let text = r#"A
3
262402
197888
5
UTF-8
13
1
1
"#;
        // let mut w = Vec::new();
        // Robj::from(1_i32).to_writer(&mut w, PstreamFormat::AsciiFormat, 3, None)?;
        // assert!(w[0] == b'A');
        // println!("{}", String::from_utf8(w.clone()).unwrap());

        let mut c = std::io::Cursor::new(text);

        let res = Robj::from_reader(&mut c, PstreamFormat::AsciiFormat, None);
        assert_eq!(res, Ok(Robj::from(1_i32)));
    }
}
