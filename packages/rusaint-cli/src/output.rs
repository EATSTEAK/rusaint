use std::{fs::File, io::Write};

use serde::Serialize;

pub fn write_json<T: Serialize>(
    file_name: &str,
    data: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    let sanitized = file_name.replace(['/', '\\', '\0', ':', '*', '?', '"', '<', '>', '|'], "_");
    let json = serde_json::to_string_pretty(data)?;
    let mut file = File::create(format!("{sanitized}.json"))?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
