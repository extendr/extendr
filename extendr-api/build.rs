use std::env;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(use_objsxp)");
    println!("cargo:rustc-check-cfg=cfg(use_r_newenv)");
    println!("cargo:rustc-check-cfg=cfg(use_r_ge_version_15)");
    println!("cargo:rustc-check-cfg=cfg(use_r_ge_version_16)");
    println!("cargo:rustc-check-cfg=cfg(use_r_ge_version_17)");
    println!("cargo:rustc-check-cfg=cfg(use_r_altlist)");

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

    // a few new features will be introduced in R 4.2
    // c.f. https://developer.r-project.org/Blog/public/2021/12/14/updating-graphics-devices-for-r-4.2.0/index.html
    if &*major >= "4" && &*minor >= "2" {
        println!("cargo:rustc-cfg=use_r_ge_version_15");
    }

    // Graphics engine version 16 was introduced in R 4.3
    if &*major >= "4" && &*minor >= "3" {
        println!("cargo:rustc-cfg=use_r_ge_version_16");
    }

    // Graphics engine version 17 was introduced in R 4.6
    if &*major >= "4" && &*minor >= "6" {
        println!("cargo:rustc-cfg=use_r_ge_version_17");
    }

    if &*major >= "4" && &*minor >= "3" {
        println!("cargo:rustc-cfg=use_r_altlist");
    }

    if &*major >= "4" && &*minor >= "4" {
        println!("cargo:rustc-cfg=use_objsxp");
    }
}
