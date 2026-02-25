use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use clap::ValueEnum;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Human,
    Json,
}

pub fn write_output<T: Serialize>(
    format: &OutputFormat,
    output: Option<&Path>,
    data: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    let text = match format {
        OutputFormat::Json => serde_json::to_string_pretty(data)?,
        OutputFormat::Human => {
            let value = serde_json::to_value(data)?;
            let mut buf = String::new();
            format_value(&value, 0, &mut buf);
            buf
        }
    };

    match output {
        Some(path) => {
            let mut file = File::create(path)?;
            file.write_all(text.as_bytes())?;
            file.write_all(b"\n")?;
        }
        None => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            handle.write_all(text.as_bytes())?;
            handle.write_all(b"\n")?;
        }
    }

    Ok(())
}

fn format_value(value: &Value, indent: usize, buf: &mut String) {
    match value {
        Value::Null => buf.push('-'),
        Value::Bool(b) => buf.push_str(&b.to_string()),
        Value::Number(n) => buf.push_str(&n.to_string()),
        Value::String(s) => buf.push_str(s),
        Value::Array(arr) => format_array(arr, indent, buf),
        Value::Object(obj) => format_object(obj, indent, buf),
    }
}

fn format_array(arr: &[Value], indent: usize, buf: &mut String) {
    if arr.is_empty() {
        buf.push_str("(없음)");
        return;
    }

    let all_objects = arr.iter().all(|v| v.is_object());

    if all_objects {
        for (i, item) in arr.iter().enumerate() {
            if i > 0 {
                buf.push('\n');
            }
            let prefix = make_indent(indent);
            buf.push_str(&format!("{prefix}[{}]\n", i + 1));
            if let Value::Object(obj) = item {
                format_object_fields(obj, indent + 2, buf);
            }
        }
    } else {
        for (i, item) in arr.iter().enumerate() {
            if i > 0 {
                buf.push('\n');
            }
            let prefix = make_indent(indent);
            buf.push_str(&prefix);
            format_value(item, indent, buf);
        }
    }
}

fn format_object(obj: &serde_json::Map<String, Value>, indent: usize, buf: &mut String) {
    if obj.is_empty() {
        buf.push_str("(없음)");
        return;
    }
    format_object_fields(obj, indent, buf);
}

fn format_object_fields(obj: &serde_json::Map<String, Value>, indent: usize, buf: &mut String) {
    let max_key_width = obj.keys().map(|k| unicode_width(k)).max().unwrap_or(0);
    let prefix = make_indent(indent);

    let mut first = true;
    for (key, value) in obj {
        if !first {
            buf.push('\n');
        }
        first = false;

        match value {
            Value::Object(_) | Value::Array(_) if !is_simple_value(value) => {
                buf.push_str(&format!("{prefix}{key}:"));
                buf.push('\n');
                format_value(value, indent + 2, buf);
            }
            _ => {
                let padding = max_key_width - unicode_width(key);
                buf.push_str(&format!("{prefix}{key}:{} ", " ".repeat(padding)));
                format_value(value, indent + 2, buf);
            }
        }
    }
}

fn is_simple_value(value: &Value) -> bool {
    match value {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => true,
        Value::Array(arr) => arr.is_empty(),
        Value::Object(obj) => obj.is_empty(),
    }
}

fn unicode_width(s: &str) -> usize {
    s.chars().map(|c| if c.is_ascii() { 1 } else { 2 }).sum()
}

fn make_indent(indent: usize) -> String {
    " ".repeat(indent)
}
