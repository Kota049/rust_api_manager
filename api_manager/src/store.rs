#[cfg(test)]
mod tests;


use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::io::Error;
use super::constants::FILE_NAME;


pub async fn store(value: &Value)->Result<(),Error> {
    let value_string = serde_json::to_string_pretty(value)?;

    let mut file = File::create(FILE_NAME)?;
    writeln!(file,"{}",value_string)?;
    Ok(())
}