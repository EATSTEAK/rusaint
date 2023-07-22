use std::collections::HashMap;

use crate::webdynpro::event_queue::{WDEvent, WDEventBuilder};

use super::Component;

pub struct Button<'a> {
    id: &'a str
}

impl<'a> Component<'a> for Button<'a> {}

impl<'a> Button<'a> {
    fn press_event(&self) -> WDEvent {
        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("id".to_string(), self.id.clone().to_string());
        WDEventBuilder::default()
            .event("Press")
            .control("Button")
            .parameters(parameters)
            .build()
            .unwrap()
    }
}