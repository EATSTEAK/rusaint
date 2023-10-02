use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell, collections::HashMap};

use crate::webdynpro::{
    element::{define_element_interactable, Interactable},
    event::Event,
};

define_element_interactable! {
    Link<"LN", "Link"> {},
    LinkLSData {
        tooltip: String => "0",
        text: String => "1",
        has_reference: bool => "2",
        enabled: bool => "3",
        has_link_caption: bool => "4",
        visibility: String => "5",
        label_text: String => "6",
        emphasized: bool => "7",
        access_key: String => "8",
        hotkey: String => "9",
        custom_data: String => "10",
        custom_style: String => "11",
        labelled_by: String => "12",
    }
}

impl<'a> Link<'a> {
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }

    pub fn activate(&self, ctrl: bool, shift: bool) -> Result<Event> {
        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("Ctrl".to_string(), ctrl.to_string());
        parameters.insert("Shift".to_string(), shift.to_string());
        self.fire_event("Activate".to_string(), parameters)
    }

    pub fn double_click(&self) -> Result<Event> {
        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        self.fire_event("DoubleClick".to_string(), parameters)
    }
}
