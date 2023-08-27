use self::ucf_parameters::UcfParameters;
use derive_builder::Builder;
use indexmap::IndexMap;
use thiserror::Error;
use std::{borrow::Cow, num::ParseIntError, string::FromUtf16Error};

pub const EVENT_SPECTATOR: &str = "~E001";
pub const EVENT_DATA_START: &str = "~E002";
pub const EVENT_DATA_END: &str = "~E003";
pub const EVENT_DATA_COLON: &str = "~E004";
pub const EVENT_DATA_COMMA: &str = "~E005";


#[derive(Error, Debug)]
pub enum EventStrUnescapeError {
    #[error("Failed read hex string")]
    Int(#[from] ParseIntError),
    #[error("hex string is not valid")]
    Parse(#[from] FromUtf16Error),
    #[error("No form found in desired application")]
    NoForm
}

pub fn escape_str<'a>(text: &'a str) -> String {
    let chars = text.chars();

    let mut owned: Vec<u8> = vec![];

    for char in chars {
        let special = match char {
            '0'..='9' | 'a'..='z' | 'A'..='Z' | '-' | '.' | '_' => false,
            _ => true,
        };

        if special {
            owned.extend(
                format!("~{:04x}", char as u16)
                    .to_ascii_uppercase()
                    .as_bytes(),
            );
        } else {
            owned.push(char as u8);
        }
    }

    unsafe { String::from_utf8_unchecked(owned) }
}

fn decode_hex(s: &str) -> Result<Vec<u16>, ParseIntError> {
    (0..s.len())
        .step_by(4)
        .map(|i| u16::from_str_radix(&s[i..i + 4], 16))
        .collect()
}

pub fn unescape_str<'a>(text: &'a str) -> Result<Cow<'a, str>, EventStrUnescapeError> {
    let bytes = text.as_bytes();

    let mut owned = None;
    let mut special: Option<Vec<u8>> = None;

    for pos in 0..bytes.len() {
        match bytes[pos] {
            b'~' => { special = Some(vec![]); },
            b'A'..=b'F' | b'0'..=b'9' => {
                if special.is_some() {
                    special.as_mut().unwrap().push(bytes[pos]);
                }
            }
            _ => { special = None },
        };

        if let Some(ref v) = special {
            if v.len() == 4 {
                if owned.is_none() {
                    owned = Some(bytes[0..(pos - 4)].to_owned());
                }
                let s: String = unsafe { String::from_utf8_unchecked(v.to_vec()) };
                let result = String::from_utf16(&decode_hex(&s)?)?;
                owned.as_mut().unwrap().extend_from_slice(result.as_bytes());
                special = None;
            }
        } else if let Some(owned) = owned.as_mut() {
            owned.push(bytes[pos]);
        }
    }
    if let Some(owned) = owned {
        unsafe { Ok(Cow::Owned(String::from_utf8_unchecked(owned))) }
    } else {
        unsafe { Ok(Cow::Borrowed(std::str::from_utf8_unchecked(bytes))) }
    }
}

#[derive(Builder)]
pub struct WDEvent {
    event: String,
    control: String,
    #[builder(default)]
    parameters: IndexMap<String, String>,
    #[builder(default)]
    ucf_parameters: UcfParameters,
    #[builder(default)]
    custom_parameters: IndexMap<String, String>,
}

impl ToString for WDEvent {
    fn to_string(&self) -> String {
        let mut owned = format!("{}_{}", &self.control, &self.event).to_owned();
        owned.push_str(EVENT_DATA_START);
        let mut params = self.parameters.iter().peekable();
        while let Some((key, val)) = params.next() {
            owned.push_str(key);
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(escape_str(val).as_str());
            if params.peek().is_some() {
                owned.push_str(EVENT_DATA_COMMA);
            }
        }
        owned.push_str(EVENT_DATA_END);
        owned.push_str(&self.ucf_parameters.serialize());
        owned.push_str(EVENT_DATA_START);
        let mut custom_params = self.custom_parameters.iter().peekable();
        while let Some((key, val)) = custom_params.next() {
            owned.push_str(key);
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(escape_str(val).as_str());
            if params.peek().is_some() {
                owned.push_str(EVENT_DATA_COMMA);
            }
        }
        owned.push_str(EVENT_DATA_END);
        owned
    }
}

impl WDEvent {
    pub fn serialize(&self) -> String {
        self.to_string()
    }
}

pub mod event_queue;
pub mod ucf_parameters;

#[cfg(test)]
mod test;
