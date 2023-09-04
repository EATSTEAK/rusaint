use indexmap::IndexMap;

use crate::webdynpro::event::{WDEvent, WDEventBuilder, ucf_parameters::{UcfParametersBuilder, UcfResponseData, UcfAction}};

use super::Element;

pub struct ComboBox<'a> {
    id: &'a str
}

impl<'a> Element<'a> for ComboBox<'a> {}

impl<'a> ComboBox<'a> {
    
    pub const fn new(id: &'a str) -> Self {
        Self {
            id
        }
    }

    pub fn select(&self, key: &str, by_enter: bool) -> WDEvent {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        let ucf_params = UcfParametersBuilder::default()
            .response(Some(UcfResponseData::Delta))
            .action(Some(UcfAction::Submit))
            .build()
            .unwrap();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("Key".to_string(), key.to_string());
        parameters.insert("ByEnter".to_string(), by_enter.to_string());
        WDEventBuilder::default()
            .control("ComboBox".to_owned())
            .event("Select".to_owned())
            .parameters(parameters)
            .ucf_parameters(ucf_params)
            .build()
            .unwrap()
    }
}