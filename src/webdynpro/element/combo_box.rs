use indexmap::IndexMap;

use crate::webdynpro::{event::{WDEvent, WDEventBuilder, ucf_parameters::{UcfParametersBuilder, UcfResponseData, UcfAction}}, application::client::body::WDBody};

use super::{Element, Parseable, ElementParser};

pub struct ComboBox<'a> {
    id: &'a str
}

pub struct ComboBoxData {
    item_list_box_id: Option<String>, // 2
    key: Option<String>, // 3
    value: Option<String>, // 4
    label_text: Option<String>, // 7
    label_for: Option<String>, // 8
    component_type: Option<String>, // 10, originally "type"
    labelled_by: Option<String>, // 27
    described_by: Option<String> // 29
}

impl<'a> Element<'a> for ComboBox<'a> {
    const CONTROL_ID: &'static str = "CB";
}

impl<'a> Parseable<'a> for ComboBox<'a> {

    fn parser(&'a self, body: &'a crate::webdynpro::application::client::body::WDBody) -> ElementParser<'a, ComboBox<'a>> {
        ElementParser {
            component: self,
            id: &self.id,
            body
        }
    }
}

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