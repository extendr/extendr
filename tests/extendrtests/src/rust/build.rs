use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(use_r_altlist)");
    println!("cargo:rerun-if-env-changed=R_HOME");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(use_r_altlist)");

    let r_home_env = std::env::var_os("R_HOME");
    let r_home_env = r_home_env.map(PathBuf::from);

    // prefer `R_HOME` if set, otherwise use the `PATH` available R
    let mut path_to_r = if let Some(r_home_env) = r_home_env {
        std::process::Command::new(r_home_env.join("bin/R"))
    } else {
        std::process::Command::new("R")
    };
    let r_version = path_to_r
        .arg("CMD")
        .args(["config", "--version"])
        .output()
        .expect("failed to run `R CMD config --version`");
    assert!(r_version.status.success());
    use std::io::BufRead as _;
    let r_version_line = r_version
        .stdout
        .lines()
        .next()
        .expect("there were no outputs from `R CMD config`")
        .expect("failed to read the line with R version");
    let raw_r_version = r_version_line
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        // ignore commit
        // only capture the first the major.minor.patch part of the version
        .split_ascii_whitespace()
        .next()
        .unwrap();
    let raw_r_version: Vec<u32> = raw_r_version
        .split('.')
        .map(|x| x.parse().unwrap())
        .collect();
    assert!(
        raw_r_version.len() >= 3,
        "R version was not detected properly"
    );
    let major = raw_r_version[0];
    let minor = raw_r_version[1];
    let patch = raw_r_version[2];

    println!("cargo:r_version_major={}", major); // Becomes DEP_R_R_VERSION_MAJOR
    println!("cargo:r_version_minor={}", minor); // Becomes DEP_R_R_VERSION_MINOR
    println!("cargo:r_version_patch={}", patch); // Becomes DEP_R_R_VERSION_PATCH

    if major >= 4 && minor >= 3 {
        println!("cargo:rustc-cfg=use_r_altlist");
    }
}
