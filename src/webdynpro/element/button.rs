use indexmap::IndexMap;

use crate::webdynpro::{event::{WDEvent, WDEventBuilder, ucf_parameters::{UcfParametersBuilder, UcfResponseData, UcfAction}}, application::client::body::WDBody};

use super::{Element, Parseable};

pub struct Button<'a> {
    id: &'a str
}

impl<'a> Element<'a> for Button<'a> {}

impl<'a> Button<'a> {
    
    pub const fn new(id: &'a str) -> Self {
        Self {
            id
        }
    }

    pub fn press(&self) -> WDEvent {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        let ucf_params = UcfParametersBuilder::default()
            .response(Some(UcfResponseData::Delta))
            .action(Some(UcfAction::Submit))
            .build()
            .unwrap();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        WDEventBuilder::default()
            .control("Button".to_owned())
            .event("Press".to_owned())
            .parameters(parameters)
            .ucf_parameters(ucf_params)
            .build()
            .unwrap()
    }
}