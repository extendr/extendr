use std::env;

fn main() {
    let major = env::var("DEP_R_R_VERSION_MAJOR").unwrap();
    let minor = env::var("DEP_R_R_VERSION_MINOR").unwrap();
    // let patch = env::var("DEP_R_R_VERSION_PATCH").unwrap();

    // R_NewEnv is available as of R 4.1.0
    if &*major >= "4" && &*minor >= "1" {
        println!("cargo:rustc-cfg=use_r_newenv");
    }
}
