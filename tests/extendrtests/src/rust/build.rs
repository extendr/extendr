fn main() {
    // TODO: I couldn't find any nice way to add the condition based on the R version
    // except for using libR-sys just for "DEP_R_*" envvars.
    // let major = std::env::var("DEP_R_R_VERSION_MAJOR").unwrap();
    // let minor = std::env::var("DEP_R_R_VERSION_MINOR").unwrap();
    let mut major = 0;
    let mut minor = 0;
    let mut _patch = 0;
    let r_version = std::process::Command::new("R")
        .arg("CMD")
        .args(["config", "--version"])
        .output()
        .unwrap();
    assert!(r_version.status.success());

    use std::io::BufRead as _;
    if let Some(Ok(r_version_line)) = r_version.stdout.lines().next() {
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
        major = raw_r_version[0];
        minor = raw_r_version[1];
        _patch = raw_r_version[2];
    }
    // dbg!(major, minor);

    if major >= 4 && minor >= 3 {
        println!("cargo:rustc-cfg=use_r_altlist");
    }
    assert_ne!(major, 0, "rust version was not detected properly");
}
