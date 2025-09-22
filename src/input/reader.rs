use anyhow::Result;
use std::io::Read;

pub fn read_input() -> Result<String> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(input)
}
