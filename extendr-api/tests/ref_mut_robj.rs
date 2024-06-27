use extendr_api::prelude::*;

#[extendr]
fn generate_model_object() -> Robj {
    Robj::from(1).set_class(["ModelObject"]).unwrap().into()
}

#[extendr]
fn generate_model_object_old() -> Robj {
    let mut robj = Robj::from(1);
    robj.set_class(["ModelObject"]).unwrap();
    robj
}

#[extendr]
fn generate_model_object_list() -> Robj {
    List::new(3).set_class(["ModelObject"]).unwrap().into()
}

#[extendr]
fn generate_model_object_list_old() -> Robj {
    let mut robj = List::new(3);
    robj.set_class(["ModelObject"]).unwrap();
    robj.into_robj()
}
