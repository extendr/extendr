
use extendr_api::*;

struct Person {
    pub name: String,
}

#[extendr]
impl Person {
    fn new() -> Self {
        Self { name: String::new() }
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[extendr]
fn aux_func(person: &Person) {
}


// Macro to generate exports
extendr_module! {
    impl Person;
    fn aux_func(person: &Person);
}

