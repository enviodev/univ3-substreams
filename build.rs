use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("SwapContract", "./abi/SwapContract.json")?
        .generate()?
        .write_to_file("src/abi/SwapContract.rs")?;

    Ok(())
}
