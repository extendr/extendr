use std::env;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(use_objsxp)");
    println!("cargo:rustc-check-cfg=cfg(use_r_newenv)");
    println!("cargo:rustc-check-cfg=cfg(use_r_altlist)");
    println!("cargo:rustc-check-cfg=cfg(r_4_4)");
    println!("cargo:rustc-check-cfg=cfg(r_4_5)");

    // The R version information is needed to handle the API differences
    // between versions. `These DEP_R_R_VERSION_*` are provided by extendr-ffi
    // (for more details, please refer to extendr-ffi's `build.rs`).
    // The current approach is to add a config flag per feature. When there are
    // too many features, we might need to consider switching to per-version
    // config flags (e.g. `r410`).
    let major = env::var("DEP_R_R_VERSION_MAJOR").unwrap();
    let minor = env::var("DEP_R_R_VERSION_MINOR").unwrap();
    // let patch = env::var("DEP_R_R_VERSION_PATCH").unwrap();

    // R_NewEnv is available as of R 4.1.0
    if &*major >= "4" && &*minor >= "1" {
        println!("cargo:rustc-cfg=use_r_newenv");
    }

    if &*major >= "4" && &*minor >= "3" {
        println!("cargo:rustc-cfg=use_r_altlist");
    }

    if &*major >= "4" && &*minor >= "4" {
        println!("cargo:rustc-cfg=use_objsxp");
        println!("cargo:rustc-cfg=r_4_4");
    }

    if &*major >= "4" && &*minor >= "5" {
        println!("cargo:rustc-cfg=r_4_5");
    }
}
