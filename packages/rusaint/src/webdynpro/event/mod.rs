use self::ucf_parameters::UcfParameters;
use derive_builder::Builder;

use super::error::EventStrUnescapeError;
use std::fmt::Display;
use std::{borrow::Cow, collections::HashMap, num::ParseIntError};

const EVENT_SPECTATOR: &str = "~E001";
const EVENT_DATA_START: &str = "~E002";
const EVENT_DATA_END: &str = "~E003";
const EVENT_DATA_COLON: &str = "~E004";
const EVENT_DATA_COMMA: &str = "~E005";

/// 일반 문자열을 이벤트 큐에서 전송하는 형태로 변환합니다.
pub fn escape_str(text: &str) -> String {
    let chars = text.chars();

    let mut owned: Vec<u8> = vec![];

    for char in chars {
        let special = !matches!(char, '0'..='9' | 'a'..='z' | 'A'..='Z' | '-' | '.' | '_');

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

/// 이벤트 큐의 문자열을 일반 문자열으로 변환합니다.
pub fn unescape_str(text: &str) -> Result<Cow<str>, EventStrUnescapeError> {
    let bytes = text.as_bytes();

    let mut owned = None;
    let mut special: Option<Vec<u8>> = None;

    for pos in 0..bytes.len() {
        match bytes[pos] {
            b'~' => {
                special = Some(vec![]);
            }
            b'A'..=b'F' | b'0'..=b'9' => {
                if let Some(special) = special.as_mut() {
                    special.push(bytes[pos]);
                }
            }
            _ => special = None,
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

/// 엘리먼트에서 전송하는 이벤트
#[allow(missing_docs)]
#[derive(Builder, Clone, Debug)]
pub struct Event {
    event: String,
    control: String,
    #[builder(default)]
    parameters: HashMap<String, String>,
    #[builder(default)]
    ucf_parameters: UcfParameters,
    #[builder(default)]
    custom_parameters: HashMap<String, String>,
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        let custom_params = self.custom_parameters.iter().peekable();
        for (key, val) in custom_params {
            owned.push_str(key);
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(escape_str(val).as_str());
            if params.peek().is_some() {
                owned.push_str(EVENT_DATA_COMMA);
            }
        }
        owned.push_str(EVENT_DATA_END);
        write!(f, "{owned}")
    }
}

impl Event {
    /// 이벤트를 웹 리퀘스트에서 전송할 수 있는 형태의 문자열로 변환합니다.
    pub fn serialize(&self) -> String {
        self.to_string()
    }

    /// 이 이벤트를 큐에 저장할 수 있다면 참을 반환합니다.
    pub fn is_enqueable(&self) -> bool {
        self.ucf_parameters.is_enqueable()
    }

    /// 이 이벤트를 큐에 저장했을 때 바로 전송할 수 있다면 참을 반환합니다.
    pub fn is_submitable(&self) -> bool {
        self.ucf_parameters.is_submitable()
    }
}

pub(crate) mod event_queue;
/// 이벤트의 특성을 정의하는 [`UcfParameters`]
pub mod ucf_parameters;

#[cfg(test)]
mod test;
