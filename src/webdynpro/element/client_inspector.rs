use indexmap::IndexMap;

use crate::webdynpro::event::{WDEvent, WDEventBuilder, ucf_parameters::{UcfParametersBuilder, UcfResponseData, UcfCardinality}};

use super::Element;

pub struct ClientInspector<'a> {
    id: &'a str
}

impl<'a> Element<'a> for ClientInspector<'a> {}

impl<'a> ClientInspector<'a> {
    pub const fn new(id: &'a str) -> Self {
        Self {
            id
        }
    }

    pub fn notify(&self, data: &str) -> WDEvent {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        let ucf_params = UcfParametersBuilder::default()
            .response(Some(UcfResponseData::Delta))
            .cardinality(Some(UcfCardinality::Single))
            .build()
            .unwrap();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("Data".to_string(), data.to_string());
        WDEventBuilder::default()
            .control("ClientInspector".to_owned())
            .event("Notify".to_owned())
            .parameters(parameters)
            .ucf_parameters(ucf_params)
            .build()
            .unwrap()
    }
}