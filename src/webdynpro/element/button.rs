use std::collections::HashMap;

use indexmap::IndexMap;
use serde::Deserialize;

use crate::webdynpro::{
    application::client::body::Body,
    event::{
        ucf_parameters::UcfParameters,
        Event,
    }, error::{BodyError, ElementError},
};

use super::{Element, ElementDef, EventParameterMap};

pub struct Button<'a> {
    id: &'a str,
    lsdata: Option<ButtonLSData>,
    lsevents: Option<HashMap<String, (UcfParameters, IndexMap<String, String>)>>,
}

#[derive(Deserialize, Default)]
#[allow(unused)]
pub struct ButtonLSData {
    #[serde(rename = "0")]
    text: Option<String>,
    #[serde(rename = "1")]
    text_design: Option<String>,
    #[serde(rename = "2")]
    design: Option<String>,
    #[serde(rename = "3")]
    width: Option<String>,
    #[serde(rename = "4")]
    tooltip: Option<String>,
    #[serde(rename = "5")]
    enabled: Option<bool>,
    #[serde(rename = "6")]
    has_button_caption: Option<bool>,
    #[serde(rename = "7")]
    visibility: Option<String>,
    #[serde(rename = "8")]
    show_help: Option<bool>,
    #[serde(rename = "9")]
    down: Option<bool>,
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
    #[serde(rename = "17")]
    image_first: Option<bool>,
    #[serde(rename = "18")]
    access_key: Option<String>,
    #[serde(rename = "19")]
    hotkey: Option<String>,
    #[serde(rename = "20")]
    up: Option<bool>,
    #[serde(rename = "21")]
    text_overflow: Option<bool>,
    #[serde(rename = "22")]
    fixed_height: Option<bool>,
    // Note: this field originally named as "type"
    #[serde(rename = "23")]
    element_type: Option<String>,
    #[serde(rename = "24")]
    drag_source_info: Option<String>,
    #[serde(rename = "25")]
    semantic_color: Option<String>,
    #[serde(rename = "26")]
    interaction_behaviour: Option<String>,
    #[serde(rename = "27")]
    custom_style: Option<String>,
    #[serde(rename = "28")]
    custom_data: Option<String>,
    #[serde(rename = "29")]
    wrapping: Option<bool>,
    #[serde(rename = "30")]
    height: Option<String>,
    #[serde(rename = "31")]
    content_visibility: Option<String>,
}

impl<'a> Element for Button<'a> {
    const CONTROL_ID: &'static str = "B";

    const ELEMENT_NAME: &'static str = "Button";

    type ElementLSData = ButtonLSData;

    fn lsdata(&self) -> Option<&ButtonLSData> {
        self.lsdata.as_ref()
    }

    fn lsevents(&self) -> Option<&EventParameterMap> {
        self.lsevents.as_ref()
    }
}

impl<'a> ElementDef<'a, Button<'a>> {
    pub fn elem(&self, body: &'_ Body) -> Result<Button<'a>, BodyError> {
        Button::from_body(self, body)
    }
}

impl<'a> Button<'a> {

    pub fn new(
        id: &'a str,
        lsdata: Option<ButtonLSData>,
        lsevents: Option<EventParameterMap>,
    ) -> Self {
        Self {
            id,
            lsdata,
            lsevents,
        }
    }

    pub fn from_body(elem_def: &ElementDef<'a, Self>, body: &'_ Body) -> Result<Self, BodyError> {
        let selector = &elem_def.selector().or(Err(BodyError::InvalidSelector))?;
        let lsdata_obj = Self::lsdata_elem(selector, body.document())?;
        let lsdata = serde_json::from_value::<ButtonLSData>(lsdata_obj).or(Err(ElementError::InvalidLSData))?;
        let lsevents = Self::lsevents_elem(selector, body.document())?;
        Ok(Self::new(elem_def.id, Some(lsdata), Some(lsevents)))
    }

    pub fn press(&self) -> Result<Event, ElementError> {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        self.fire_event("Press", parameters)
    }
}
