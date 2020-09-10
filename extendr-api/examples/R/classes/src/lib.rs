
use extendr_api::*;

struct Person {
    pub name: String,
}

#[export_interface]
impl Person {
    #[export_interface(constructor)]
    fn new() -> Self {
        Self { name: "" }
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn name(&self) -> &str {
        self.name
    }
}

#[export_function]
fn aux_func(person: &Person) {
}


// Macro to generate exports
extendr_module! {
    impl Person;
    fn aux_func(person: &Person);
}

