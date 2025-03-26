fn main() {
    // use R_HOME if provided
    let r_home = match std::env::var("R_HOME") {
        Ok(v) => v,
        // otherwise use the DEP_R_R_HOME from extendr-ffi build.rs
        Err(_) => std::env::var("DEP_R_R_HOME").expect("failed to get DEP_R_R_HOME"),
    };
    // set the environment variable accordingly
    println!("cargo:rustc-env=R_HOME={}", r_home);
}
