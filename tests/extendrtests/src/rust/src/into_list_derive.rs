use extendr_api::prelude::*;

// Test struct with basic types
#[derive(Debug, IntoList)]
pub struct BasicStruct {
    pub int_field: i32,
    pub double_field: f64,
    pub bool_field: bool,
    pub string_field: String,
}

#[extendr]
fn make_basic_struct() -> Robj {
    BasicStruct {
        int_field: 42,
        double_field: std::f64::consts::PI,
        bool_field: true,
        string_field: String::from("hello from rust"),
    }
    .into()
}

// Test struct with R wrapper types
#[derive(Debug, IntoList)]
pub struct RWrapperStruct {
    pub doubles: Doubles,
    pub logicals: Logicals,
    pub strings: Strings,
    pub raw: Raw,
}

#[extendr]
fn make_rwrapper_struct() -> Robj {
    RWrapperStruct {
        doubles: Doubles::from_values([1.0, 2.0, 3.0]),
        logicals: Logicals::from_values([true, false, true]),
        strings: Strings::from_values(["alpha", "beta", "gamma"]),
        raw: Raw::from_bytes(&[0xDE, 0xAD, 0xBE, 0xEF]),
    }
    .into()
}

// Test struct with List field
#[derive(Debug, IntoList)]
pub struct WithList {
    pub name: String,
    pub data: List,
}

#[extendr]
fn make_with_list() -> Robj {
    WithList {
        name: String::from("my_list"),
        data: list!(x = 1, y = 2, z = 3),
    }
    .into()
}

// Test struct with Robj field
#[derive(Debug, IntoList)]
pub struct WithRobj {
    pub label: String,
    pub value: Robj,
}

#[extendr]
fn make_with_robj() -> Robj {
    WithRobj {
        label: String::from("answer"),
        value: r!(42),
    }
    .into()
}

// Test struct with Function field
#[derive(Debug, IntoList)]
pub struct WithFunction {
    pub func_name: String,
    pub func: Function,
}

#[extendr]
fn make_with_function() -> Robj {
    single_threaded(|| {
        WithFunction {
            func_name: String::from("sum"),
            func: R!("sum").unwrap().try_into().unwrap(),
        }
        .into()
    })
}

// Test struct with Pairlist field
#[derive(Debug, IntoList)]
pub struct WithPairlist {
    pub description: String,
    pub pairs: Pairlist,
}

#[extendr]
fn make_with_pairlist() -> Robj {
    WithPairlist {
        description: String::from("pairlist container"),
        pairs: pairlist!(a = 10, b = 20, c = 30),
    }
    .into()
}

// Test struct with Environment field
#[derive(Debug, IntoList)]
pub struct WithEnvironment {
    pub env_name: String,
    pub env: Environment,
}

#[extendr]
fn make_with_environment() -> Robj {
    single_threaded(|| {
        let env = Environment::new_with_parent(global_env());
        env.set_local(sym!(x), 100);
        env.set_local(sym!(y), "test");

        WithEnvironment {
            env_name: String::from("my_environment"),
            env,
        }
        .into()
    })
}

// Test struct with ignored fields
#[derive(Debug, IntoList)]
pub struct WithIgnoredFields {
    pub visible_name: String,
    pub visible_count: i32,
    #[into_list(ignore)]
    pub internal_ptr: *const u8,
    #[into_list(ignore)]
    pub private_buffer: Vec<u8>,
}

#[extendr]
fn make_with_ignored() -> Robj {
    WithIgnoredFields {
        visible_name: String::from("public data"),
        visible_count: 99,
        internal_ptr: std::ptr::null(),
        private_buffer: vec![1, 2, 3, 4, 5],
    }
    .into()
}

// Test struct with vector fields
#[derive(Debug, IntoList)]
pub struct WithVectors {
    pub int_vec: Vec<i32>,
    pub double_vec: Vec<f64>,
    pub string_vec: Vec<String>,
    pub bool_vec: Vec<bool>,
}

#[extendr]
fn make_with_vectors() -> Robj {
    WithVectors {
        int_vec: vec![1, 2, 3, 4, 5],
        double_vec: vec![1.1, 2.2, 3.3],
        string_vec: vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
        ],
        bool_vec: vec![true, false, true, true, false],
    }
    .into()
}

// Test complex nested structure
#[derive(Debug, IntoList)]
pub struct NestedInner {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, IntoList)]
pub struct NestedOuter {
    pub name: String,
    pub count: i32,
    pub nested_data: List,
}

#[extendr]
fn make_nested_struct() -> Robj {
    let inner = NestedInner { x: 10.5, y: 20.5 };
    let inner_list: List = Robj::from(inner).try_into().unwrap();

    NestedOuter {
        name: String::from("outer"),
        count: 2,
        nested_data: inner_list,
    }
    .into()
}

// Test struct similar to the PR example
#[derive(Debug, IntoList)]
pub struct FunctionMetadata {
    pub doc: &'static str,
    pub rust_name: &'static str,
    pub r_name: &'static str,
    pub return_type: &'static str,
    pub num_args: i32,
    #[into_list(ignore)]
    pub func_ptr: *const u8,
    pub is_hidden: bool,
}

#[extendr]
fn make_function_metadata() -> Robj {
    FunctionMetadata {
        doc: "Example function documentation",
        rust_name: "example_fn",
        r_name: "exampleFn",
        return_type: "Robj",
        num_args: 3,
        func_ptr: std::ptr::null(),
        is_hidden: false,
    }
    .into()
}

// Test with all R types in one struct
#[derive(Debug, IntoList)]
pub struct AllRTypes {
    pub doubles_field: Doubles,
    pub logicals_field: Logicals,
    pub raw_field: Raw,
    pub strings_field: Strings,
    pub list_field: List,
    pub robj_field: Robj,
    pub function_field: Function,
    pub pairlist_field: Pairlist,
    pub environment_field: Environment,
}

#[extendr]
fn make_all_r_types() -> Robj {
    single_threaded(|| {
        let env = Environment::new_with_parent(global_env());
        env.set_local(sym!(test), "value");

        AllRTypes {
            doubles_field: Doubles::from_values([1.0, 2.0]),
            logicals_field: Logicals::from_values([true, false]),
            raw_field: Raw::from_bytes(&[0x01, 0x02]),
            strings_field: Strings::from_values(["a", "b"]),
            list_field: list!(x = 1),
            robj_field: r!(42),
            function_field: R!("mean").unwrap().try_into().unwrap(),
            pairlist_field: pairlist!(a = 1),
            environment_field: env,
        }
        .into()
    })
}

extendr_module! {
    mod into_list_derive;
    fn make_basic_struct;
    fn make_rwrapper_struct;
    fn make_with_list;
    fn make_with_robj;
    fn make_with_function;
    fn make_with_pairlist;
    fn make_with_environment;
    fn make_with_ignored;
    fn make_with_vectors;
    fn make_nested_struct;
    fn make_function_metadata;
    fn make_all_r_types;
}
