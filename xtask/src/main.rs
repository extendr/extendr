use xshell::{cmd, Shell};


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let shell = Shell::new()?;
    let generate_docs = cmd!(
        shell,
        "cargo doc --workspace --no-deps --document-private-items --features full-functionality"
    );

    generate_docs.run()?;

    Ok(())
}
