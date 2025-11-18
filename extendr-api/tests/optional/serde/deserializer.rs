use extendr_api::deserializer::from_robj;
use extendr_api::prelude::*;
use extendr_api::serializer::to_robj;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

////////////////////////////////////////////////////////////////////////////////
///
/// Deserialize from a Robj.
///
/// Like JSON, we can use a Robj as a storage format.
///
/// For example if creating vectors from a RDS file or returning a structure
/// or just doing a conversion.
///
#[test]
fn test_deserialize_robj() {
    test! {
        // In these tests, the wrapper is transparent and just tests the contents.
        // So Int(i32) is actually testing i32.

        #[derive(Deserialize, PartialEq, Debug)]
        struct Null;
        assert_eq!(from_robj::<Null>(&r!(NULL)), Ok(Null));
        assert_eq!(from_robj::<Null>(&r!(1)), Err(Error::ExpectedNull(r!(1))));

        #[derive(Deserialize, PartialEq, Debug)]
        struct Int(i32);
        assert_eq!(from_robj::<Int>(&r!(1)), Ok(Int(1)));
        assert_eq!(from_robj::<Int>(&r!(1.0)), Ok(Int(1)));
        assert_eq!(from_robj::<Int>(&r!(NULL)).is_err(), true);

        #[derive(Deserialize, PartialEq, Debug)]
        struct RInt(Rint);
        assert_eq!(from_robj::<RInt>(&r!(1)), Ok(RInt(1.into())));
        assert_eq!(from_robj::<RInt>(&r!(1.0)), Ok(RInt(1.into())));
        assert_eq!(from_robj::<RInt>(&r!(Rint::na())).is_err(), true);
        assert_eq!(from_robj::<RInt>(&r!(NULL)).is_err(), true);

        #[derive(Deserialize, PartialEq, Debug)]
        struct Float(f64);
        assert_eq!(from_robj::<Float>(&r!(1)), Ok(Float(1.0)));
        assert_eq!(from_robj::<Float>(&r!(1.0)), Ok(Float(1.0)));
        assert_eq!(from_robj::<Float>(&r!(NULL)).is_err(), true);

        #[derive(Deserialize, PartialEq, Debug)]
        struct RFloat(Rfloat);
        assert_eq!(from_robj::<RFloat>(&r!(1)), Ok(RFloat(1.0.into())));
        assert_eq!(from_robj::<RFloat>(&r!(1.0)), Ok(RFloat(1.0.into())));
        assert_eq!(from_robj::<RFloat>(&r!(Rfloat::na())).is_err(), true);
        assert_eq!(from_robj::<RFloat>(&r!(NULL)).is_err(), true);

        #[derive(Deserialize, PartialEq, Debug)]
        struct Bool(bool);
        assert_eq!(from_robj::<Bool>(&r!(TRUE)), Ok(Bool(true)));
        assert_eq!(from_robj::<Bool>(&r!(FALSE)), Ok(Bool(false)));
        assert_eq!(from_robj::<Bool>(&r!(NULL)).is_err(), true);

        #[derive(Deserialize, PartialEq, Debug)]
        struct RBool(Rbool);
        assert_eq!(from_robj::<RBool>(&r!(TRUE)), Ok(RBool(TRUE)));
        assert_eq!(from_robj::<RBool>(&r!(FALSE)), Ok(RBool(FALSE)));
        assert_eq!(from_robj::<RBool>(&r!(Rbool::na())).is_err(), true);
        assert_eq!(from_robj::<RBool>(&r!(NULL)).is_err(), true);

        #[derive(Deserialize, PartialEq, Debug)]
        struct VInt(Vec<i32>);
        assert_eq!(from_robj::<VInt>(&r!(list!(1, 2))), Ok(VInt(vec![1, 2])));
        assert_eq!(from_robj::<VInt>(&r!([1, 2])), Ok(VInt(vec![1, 2])));
        assert_eq!(from_robj::<VInt>(&r!([1, 2, i32::na()])).is_err(), true);

        // Any integer type will do.
        #[derive(Deserialize, PartialEq, Debug)]
        struct VInt16(Vec<i16>);
        assert_eq!(from_robj::<VInt16>(&r!(list!(1, 2))), Ok(VInt16(vec![1, 2])));
        assert_eq!(from_robj::<VInt16>(&r!([1, 2])), Ok(VInt16(vec![1, 2])));

        #[derive(Deserialize, PartialEq, Debug)]
        struct VFloat64(Vec<f64>);
        assert_eq!(from_robj::<VFloat64>(&r!(list!(1, 2))), Ok(VFloat64(vec![1., 2.])));
        assert_eq!(from_robj::<VFloat64>(&r!([1, 2])), Ok(VFloat64(vec![1., 2.])));

        #[derive(Deserialize, PartialEq, Debug)]
        struct VBool(Vec<bool>);
        assert_eq!(from_robj::<VBool>(&r!(list!(TRUE, FALSE))), Ok(VBool(vec![true, false])));
        assert_eq!(from_robj::<VBool>(&r!([TRUE, FALSE])), Ok(VBool(vec![true, false])));

        #[derive(Deserialize, PartialEq, Debug)]
        struct Str(String);
        assert_eq!(from_robj::<Str>(&r!("xyz")), Ok(Str("xyz".into())));

        #[derive(Deserialize, PartialEq, Debug)]
        struct StrSlice<'a>(&'a str);
        assert_eq!(from_robj::<StrSlice>(&r!("xyz")), Ok(StrSlice("xyz")));

        // Structs are mapped to named lists.
        #[derive(Deserialize, PartialEq, Debug)]
        struct Struct { a: i32, b: f64 }
        assert_eq!(from_robj::<Struct>(&r!(list!(a=1, b=2))), Ok(Struct{ a: 1, b: 2.0 }));

        // Enums are mapped to named lists of lists.
        #[derive(Deserialize, PartialEq, Debug)]
        enum Enum {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32 },
        }

        let j = r!("Unit");
        let expected = Enum::Unit;
        assert_eq!(expected, from_robj(&j).unwrap());

        // If the name is wrong:
        let j = r!("UnitX");
        assert_eq!(from_robj::<Enum>(&j).is_err(), true);

        let j = r!(list!(Newtype=1));
        let expected = Enum::Newtype(1);
        assert_eq!(expected, from_robj(&j).unwrap());

        let j = r!(list!(Tuple=list!(1, 2)));
        let expected = Enum::Tuple(1, 2);
        assert_eq!(expected, from_robj(&j).unwrap());

        let j = r!(list!(Struct=list!(a=1)));
        let expected = Enum::Struct { a: 1 };
        assert_eq!(expected, from_robj(&j).unwrap());

        // Many things will generate a Robj.
        // But note that the original Robj will not be copied verbatim.
        // The Deserialize trait for Robj can also be used to generate
        // JSON and other formats for Robj.
        #[derive(Deserialize, PartialEq, Debug)]
        struct MyRobj(Robj);
        assert_eq!(from_robj::<MyRobj>(&r!(TRUE)), Ok(MyRobj(r!(TRUE))));
        assert_eq!(from_robj::<MyRobj>(&r!(1)), Ok(MyRobj(r!(1))));
        assert_eq!(from_robj::<MyRobj>(&r!(1.0)), Ok(MyRobj(r!(1.0))));
        assert_eq!(from_robj::<MyRobj>(&r!("xyz")), Ok(MyRobj(r!("xyz"))));

        // Sequences are always converted to lists.
        assert_eq!(from_robj::<MyRobj>(&r!([TRUE, FALSE])), Ok(MyRobj(r!(list!(TRUE, FALSE)))));
        assert_eq!(from_robj::<MyRobj>(&r!([1, 2])), Ok(MyRobj(r!(list!(1, 2)))));

        // If you use a wrapper type, conversions are more specific.
        #[derive(Deserialize, PartialEq, Debug)]
        struct RIntegers(Integers);
        assert_eq!(from_robj::<RIntegers>(&r!(1)), Ok(RIntegers(Integers::from_values([1]))));
        assert_eq!(from_robj::<RIntegers>(&r!([1, 2])), Ok(RIntegers(Integers::from_values([1, 2]))));
        assert_eq!(from_robj::<RIntegers>(&r!(1.0)).is_err(), true);
        assert_eq!(from_robj::<RIntegers>(&r!("xyz")).is_err(), true);

        #[derive(Deserialize, PartialEq, Debug)]
        struct RDoubles(Doubles);
        assert_eq!(from_robj::<RDoubles>(&r!(1)), Ok(RDoubles(Doubles::from_values([1.0]))));
        // assert_eq!(from_robj::<RDoubles>(&r!([1, 2])), Ok(RDoubles(Doubles::from_values([1.0, 2.0]))));
        assert_eq!(from_robj::<RDoubles>(&r!([1.0, 2.0])), Ok(RDoubles(Doubles::from_values([1.0, 2.0]))));
        assert_eq!(from_robj::<RDoubles>(&r!("xyz")).is_err(), true);

        #[derive(Deserialize, PartialEq, Debug)]
        struct RLogicals(Logicals);
        assert_eq!(from_robj::<RLogicals>(&r!(TRUE)), Ok(RLogicals(Logicals::from_values([TRUE]))));
        assert_eq!(from_robj::<RLogicals>(&r!([TRUE, FALSE, NA_LOGICAL])), Ok(RLogicals(Logicals::from_values([TRUE, FALSE, NA_LOGICAL]))));
        assert_eq!(from_robj::<RLogicals>(&r!("xyz")).is_err(), true);

        // This requires a PR that is not yet merged.
        // #[derive(Deserialize, PartialEq, Debug)]
        // struct RStrings(Strings);
        // assert_eq!(from_robj::<RStrings>(&r!("xyz")), Ok(RStrings(Strings::from_values(["xyz"]))));
        // assert_eq!(from_robj::<RStrings>(&r!(["a", "b"])), Ok(RStrings(Strings::from_values(["a", "b"]))));
        // assert_eq!(from_robj::<RStrings>(&r!(0)).is_err(), true);
    }
}

#[test]
fn test_deserialize_complex_named_list() {
    test! {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct RunStartFile {
            model: String,
        }

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct OutputFileHash {
            path: String,
            hash: String,
        }

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct RRunEndFile {
            start: String,
            end: String,
            runtime_ms: f64,
            files_copied: HashSet<String>,
            output_files_rewrites: HashMap<String, String>,
            output_files_hashes: Vec<OutputFileHash>,
        }

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct ModelMetadata {
            name: String,
        }

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct RLineageTree {
            nodes: HashMap<String, ModelMetadata>,
            metadata: HashMap<String, (RunStartFile, Option<RRunEndFile>)>,
        }

        let mut files_copied = HashSet::new();
        files_copied.insert("input.lst".to_string());

        let mut output_files_rewrites = HashMap::new();
        output_files_rewrites.insert("out.lst".to_string(), "rewritten.lst".to_string());

        let end_file = RRunEndFile {
            start: "2024-01-01T00:00:00Z".into(),
            end: "2024-01-01T01:00:00Z".into(),
            runtime_ms: 3600_000.0,
            files_copied,
            output_files_rewrites,
            output_files_hashes: vec![OutputFileHash {
                path: "out.lst".into(),
                hash: "abc123".into(),
            }],
        };

        let lineage = RLineageTree {
            nodes: [("node_a".to_string(), ModelMetadata { name: "model_a".into() })]
                .into_iter()
                .collect(),
            metadata: [(
                "node_a".to_string(),
                (
                    RunStartFile {
                        model: "model_a.mod".into(),
                    },
                    Some(end_file),
                ),
            )]
            .into_iter()
            .collect(),
        };

        let robj = to_robj(&lineage)?;
        let round_trip: RLineageTree = from_robj(&robj)?;
        assert_eq!(lineage, round_trip);
    }
}
