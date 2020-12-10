use std::env;

fn main() {
    println!(
        "cargo:rustc-env=R_HOME={}",
        env::var("DEP_R_R_HOME").unwrap()
    );
}
