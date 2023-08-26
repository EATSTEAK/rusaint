use std::{collections::HashMap, borrow::Cow};
use derive_builder::Builder;
use self::ucf_parameters::UcfParameters;

pub const EVENT_SPECTATOR: &str = "~E001";
pub const EVENT_DATA_START: &str = "~E002";
pub const EVENT_DATA_END: &str = "~E003";
pub const EVENT_DATA_COLON: &str = "~E004";
pub const EVENT_DATA_COMMA: &str = "~E005";


pub fn escape<'a>(text: &'a str) -> Cow<'a, str> {

    let bytes = text.as_bytes();

    let mut owned = None;

    for pos in 0..bytes.len() {
        
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