use std::env;

fn main() {
    // TODO: I couldn't find any nice way to add the condition based on the R version
    // except for using libR-sys just for "DEP_R_*" envvars.
    let major = env::var("DEP_R_R_VERSION_MAJOR").unwrap();
    let minor = env::var("DEP_R_R_VERSION_MINOR").unwrap();

    if &*major >= "4" && &*minor >= "3" {
        println!("cargo:rustc-cfg=use_r_altlist");
    }
}
