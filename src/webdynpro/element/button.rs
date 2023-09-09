use indexmap::IndexMap;
use serde::Deserialize;

use crate::webdynpro::{event::{WDEvent, WDEventBuilder, ucf_parameters::{UcfParametersBuilder, UcfResponseData, UcfAction, UcfParameters}}, application::client::body::{WDBody, WDBodyError}};

use super::{Element, Parseable, ElementParser};

pub struct Button<'a> {
    id: &'a str
}

#[derive(Deserialize)]
pub struct ButtonData {
    #[serde(rename = "0")]
    text: Option<String>,
    #[serde(rename = "4")]
    tooltip: Option<String>,
    #[serde(rename = "5")]
    enabled: Option<bool>,
    #[serde(rename = "10")]
    has_icon: Option<bool>,
    #[serde(rename = "11")]
    disabled_icon_src: Option<String>,
    #[serde(rename = "12")]
    up_icon_src: Option<String>,
    #[serde(rename = "13")]
    down_icon_src: Option<String>,
    #[serde(rename = "14")]
    has_popup_menu: Option<bool>,
    #[serde(rename = "15")]
    popup_menu_id: Option<String>,
    #[serde(rename = "16")]
    has_popup_menu_section: Option<bool>,
    // Note: this field originally named as "type"
    #[serde(rename = "23")]
    element_type: Option<String>
}

impl<'a> Element<'a> for Button<'a> {
    const CONTROL_ID: &'static str = "B";
}

impl<'a> Parseable<'a> for Button<'a> {

    fn parser(&'a self, body: &'a WDBody) -> ElementParser<'a, Self> {
        ElementParser {
            component: &self,
            id: &self.id,
            body
        }
    }
}

impl<'a> ElementParser<'a, Button<'a>> {
    fn lsdata(&'a self) -> Result<ButtonData, WDBodyError> {
        let raw_data = self.raw_lsdata()?;
        return serde_json::from_str::<ButtonData>(&raw_data).or(Err(WDBodyError::Invalid));
    }
}

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