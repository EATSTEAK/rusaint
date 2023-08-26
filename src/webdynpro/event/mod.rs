use std::{collections::HashMap, borrow::Cow};
use derive_builder::Builder;
use self::ucf_parameters::UcfParameters;

pub const EVENT_SPECTATOR: &str = "~E001";
pub const EVENT_DATA_START: &str = "~E002";
pub const EVENT_DATA_END: &str = "~E003";
pub const EVENT_DATA_COLON: &str = "~E004";
pub const EVENT_DATA_COMMA: &str = "~E005";


pub fn escape_str<'a>(text: &'a str) -> Cow<'a, str> {

    let bytes = text.as_bytes();

    let mut owned = None;

    for pos in 0..bytes.len() {
        let special = match bytes[pos] {
            b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'-' | b'.' | b'_' => None,
            _ => Some(bytes[pos])
        };

        if let Some(s) = special {
            if owned.is_none() {
                owned = Some(bytes[0..pos].to_owned());
            }
            owned.as_mut().unwrap().extend_from_slice(format!("~{:02x}", s).as_bytes());
        } else if let Some(owned) = owned.as_mut() {
            owned.push(bytes[pos]);
        }
    }

    if let Some(owned) = owned {
        unsafe { Cow::Owned(String::from_utf8_unchecked(owned)) }
    } else {
        unsafe { Cow::Borrowed(std::str::from_utf8_unchecked(bytes))}
    }
}

pub fn unescape<'a>(text: &'a str) -> Cow<'a, str> {

    let bytes = text.as_bytes();
    
    let mut owned = None;

    if let Some(owned) = owned {
        unsafe { Cow::Owned(String::from_utf8_unchecked(owned)) }
    } else {
        unsafe { Cow::Borrowed(std::str::from_utf8_unchecked(bytes))}
    }
}

#[derive(Builder)]
pub struct WDEvent<'a> {
    event: &'a str,
    control: &'a str,
    parameters: HashMap<String, String>,
    ucf_parameters: UcfParameters,
    custom_parameters: HashMap<String, String>
}

impl<'a> WDEvent<'a> {
    fn serialize() -> String {
        todo!("Implement Serialization");
    }
}

mod ucf_parameters;
pub(crate) mod event_queue;

#[cfg(test)]
mod test;