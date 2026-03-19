use anyhow::Result;

fn main() -> Result<()> {
    rust_fun::playground::typestate::run()?;
    rust_fun::playground::typestate_typed::run()?;
    Ok(())
}
