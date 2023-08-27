use std::collections::HashMap;

use indexmap::IndexMap;

use crate::webdynpro::event::{WDEvent, WDEventBuilder};

use super::Component;

pub struct Button<'a> {
    id: &'a str
}

impl<'a> Component<'a> for Button<'a> {}

impl<'a> Button<'a> {
    fn press_event(&self) -> WDEvent {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        parameters.insert("id".to_string(), self.id.clone().to_string());
        WDEventBuilder::default()
            .event("Press")
            .control("Button")
            .parameters(parameters)
            .build()
            .unwrap()
    }
}