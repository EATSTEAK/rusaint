use indexmap::IndexMap;

use crate::webdynpro::{event::{WDEvent, WDEventBuilder, ucf_parameters::{UcfParametersBuilder, UcfResponseData, UcfAction}}, application::client::body::WDBody};

use super::{Element, Parseable, ElementParser};

pub struct SapTable<'a> {
    id: &'a str
}

pub struct SapTableData {
    title_text: Option<String>, // 0
    accessibility_description: Option<String>, // 1
    row_count: Option<u32>, // 2
    col_count: Option<u32>, // 3
}

impl<'a> Element<'a> for SapTable<'a> {
    const CONTROL_ID: &'static str = "ST";
}

impl<'a> Parseable<'a> for SapTable<'a> {

    fn parser(&'a self, body: &'a WDBody) -> ElementParser<'a, SapTable<'a>> {
        ElementParser {
            component: &self,
            id: &self.id,
            body
        }
    }
}

impl<'a> SapTable<'a> {
    
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
            .control("Table".to_owned())
            .event("Press".to_owned())
            .parameters(parameters)
            .ucf_parameters(ucf_params)
            .build()
            .unwrap()
    }
}