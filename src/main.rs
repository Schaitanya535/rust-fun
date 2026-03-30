use anyhow::Result;

fn main() -> Result<()> {
    rust_fun::playground::typestate::run()?;
    rust_fun::playground::typestate_typed::run()?;
    rust_fun::playground::dependency_injection::run()?;
    rust_fun::playground::let_else_exmaple::run()?;
    Ok(())
}
