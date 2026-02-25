use std::{fs::File, io::Write};

use serde::Serialize;

pub fn write_json<T: Serialize>(
    file_name: &str,
    data: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(data)?;
    let mut file = File::create(format!("{file_name}.json"))?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
