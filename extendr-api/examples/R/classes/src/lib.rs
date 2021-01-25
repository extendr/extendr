
use extendr_api::prelude::*;

#[derive(Debug)]
struct Person {
    pub name: String,
}

#[extendr]
impl Person {
    fn new() -> Self {
        Self { name: "".to_string() }
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[extendr]
fn aux_func() {
}


// Macro to generate exports
extendr_module! {
    mod classes;
    impl Person;
    fn aux_func;
}

