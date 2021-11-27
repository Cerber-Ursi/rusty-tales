use std::error::Error;

mod index;

fn main() -> Result<(), Box<dyn Error>> {
    index::convert()?;
    Ok(())
}
