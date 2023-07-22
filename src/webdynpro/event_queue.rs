use std::{collections::{HashMap, LinkedList}, ops::{Deref, DerefMut}};
use derive_builder::Builder;
use crate::webdynpro::model::UcfParameters;

pub const EVENT_SPECTATOR: &str = "~E001";
pub const EVENT_DATA_START: &str = "~E002";
pub const EVENT_DATA_END: &str = "~E003";
pub const EVENT_DATA_COLON: &str = "~E004";
pub const EVENT_DATA_COMMA: &str = "~E005";

#[derive(Builder)]
pub struct WDEvent<'a> {
    event: &'a str,
    control: &'a str,
    parameters: HashMap<String, String>,
    ucf_parameters: UcfParameters,
    custom_parameters: HashMap<String, String>
}

impl<'a> WDEvent<'a> {
    fn serialize() {
        // TODO: Implement Serialization
    }
}

pub struct WDEventQueue<'a>(LinkedList<WDEvent<'a>>);

impl<'a> Deref for WDEventQueue<'a> {
    type Target = LinkedList<WDEvent<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for WDEventQueue<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> WDEventQueue<'a> {
    fn serialize_and_clear(&mut self) {
        // TODO: Implement Serialization
    }

    fn add(&mut self, evt: WDEvent<'a>) {
        &self.push_back(evt);
    }

    fn remove(&mut self) -> Option<WDEvent<'a>> {
        self.pop_front()
    }
}