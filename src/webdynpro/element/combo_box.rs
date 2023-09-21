use indexmap::IndexMap;
use serde::Deserialize;

use crate::webdynpro::{event::Event, application::client::body::Body, error::{BodyError, ElementError}};

use super::{Element, EventParameterMap, ElementDef};

pub struct ComboBox<'a> {
    id: &'a str,
    lsdata: Option<ComboBoxLSData>,
    lsevents: Option<EventParameterMap>
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct ComboBoxLSData {
    #[serde(rename = "0")]
    behavior: Option<String>,
    #[serde(rename = "1")]
    allow_virtual_typing: Option<String>,
    #[serde(rename = "2")]
    item_list_box_id: Option<String>,
    #[serde(rename = "3")]
    key: Option<String>,
    #[serde(rename = "4")]
    value: Option<String>,
    #[serde(rename = "5")]
    visibility: Option<String>,
    #[serde(rename = "6")]
    container_width_set: Option<bool>,
    #[serde(rename = "7")]
    label_text: Option<String>,
    #[serde(rename = "8")]
    label_for: Option<String>,
    #[serde(rename = "9")]
    ime_mode: Option<String>,
    #[serde(rename = "10")]
    component_type: Option<String>, // originally "type"
    #[serde(rename = "11")]
    show_help_button_always: Option<String>,
    #[serde(rename = "12")]
    access_key: Option<String>,
    #[serde(rename = "13")]
    suggest_filter: Option<String>,
    #[serde(rename = "14")]
    display_as_text: Option<bool>,
    #[serde(rename = "15")]
    hide_field_help: Option<bool>,
    #[serde(rename = "16")]
    show_help_button: Option<bool>,
    #[serde(rename = "17")]
    suggest_auto_complete: Option<bool>,
    #[serde(rename = "18")]
    suggest_filter_condition: Option<String>,
    #[serde(rename = "19")]
    field_help_floating: Option<bool>,
    #[serde(rename = "20")]
    custom_data: Option<String>,
    #[serde(rename = "21")]
    custom_style: Option<String>,
    #[serde(rename = "22")]
    text_style: Option<String>,
    #[serde(rename = "23")]
    semantic_color: Option<String>,
    #[serde(rename = "24")]
    embedding_behaviour: Option<String>,
    #[serde(rename = "25")]
    sap_table_field_design: Option<String>,
    #[serde(rename = "26")]
    field_help_embedding: Option<bool>,
    #[serde(rename = "27")]
    labelled_by: Option<String>,
    #[serde(rename = "28")]
    tab_behaviour: Option<String>,
    #[serde(rename = "29")]
    described_by: Option<String>
}

impl<'a> Element for ComboBox<'a> {
    const CONTROL_ID: &'static str = "CB";

    const ELEMENT_NAME: &'static str = "ComboBox";

    type ElementLSData = ComboBoxLSData;

    fn lsdata(&self) -> Option<&Self::ElementLSData> {
        self.lsdata.as_ref()
    }

    fn lsevents(&self) -> Option<&EventParameterMap> {
        self.lsevents.as_ref()
    }
}

impl<'a> ElementDef<'a, ComboBox<'a>> {
    pub fn elem(&self, body: &'_ Body) -> Result<ComboBox<'a>, BodyError> {
        ComboBox::from_body(self, body)
    }
}

impl<'a> ComboBox<'a> {
    
    pub const fn new(id: &'a str, lsdata: Option<ComboBoxLSData>, lsevents: Option<EventParameterMap>) -> Self {
        Self {
            id,
            lsdata,
            lsevents
        }
    }

    pub fn from_body(elem_def: &ElementDef<'a, Self>, body: &'_ Body) -> Result<Self, BodyError> {
        let selector = &elem_def.selector().or(Err(BodyError::InvalidSelector))?;
        let lsdata_obj = Self::lsdata_elem(selector, body.document())?;
        let lsdata = serde_json::from_value::<ComboBoxLSData>(lsdata_obj).or(Err(ElementError::InvalidLSData))?;
        let lsevents = Self::lsevents_elem(selector, body.document())?;
        Ok(Self::new(elem_def.id, Some(lsdata), Some(lsevents)))
    }

    pub fn select(&self, key: &str, by_enter: bool) -> Result<Event, ElementError> {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("Key".to_string(), key.to_string());
        parameters.insert("ByEnter".to_string(), by_enter.to_string());
        self.fire_event("Select", parameters)
    }
}