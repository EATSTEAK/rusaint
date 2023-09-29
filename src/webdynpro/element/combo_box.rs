use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell};

use indexmap::IndexMap;
use serde::Deserialize;

use crate::webdynpro::{event::Event, error::ElementError, application::client::body::Body};

use super::{Element, ElementDef, EventParameterMap, Elements, list_box::ListBoxes};

#[derive(Debug)]
pub struct ComboBox<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<ComboBoxLSData>>,
    lsevents: OnceCell<Option<EventParameterMap>>,
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
    described_by: Option<String>,
}

impl<'a> Element<'a> for ComboBox<'a> {
    const CONTROL_ID: &'static str = "CB";

    const ELEMENT_NAME: &'static str = "ComboBox";

    type ElementLSData = ComboBoxLSData;

    fn lsdata(&self) -> Option<&Self::ElementLSData> {
        self.lsdata
            .get_or_init(|| {
                let lsdata_obj = Self::lsdata_elem(self.element_ref).ok()?;
                serde_json::from_value::<Self::ElementLSData>(lsdata_obj).ok()
            })
            .as_ref()
    }

    fn lsevents(&self) -> Option<&EventParameterMap> {
        self.lsevents
            .get_or_init(|| Self::lsevents_elem(self.element_ref).ok())
            .as_ref()
    }

    fn from_elem(elem_def: ElementDef<'a, Self>, element: scraper::ElementRef<'a>) -> Result<Self> {
        Ok(Self::new(elem_def.id.to_owned(), element))
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn element_ref(&self) -> &scraper::ElementRef<'a> {
        &self.element_ref
    }

    fn wrap(self) -> super::Elements<'a> {
        super::Elements::ComboBox(self)
    }
}

impl<'a> ComboBox<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }

    pub fn item_list_box(&self, body: &'a Body) -> Result<ListBoxes<'a>> {
        let listbox_id = self.lsdata().and_then(|lsdata| {lsdata.item_list_box_id.as_ref()}).ok_or(ElementError::InvalidLSData)?;
        let selector = scraper::Selector::parse(format!(r#"[id="{}"]"#, listbox_id).as_str()).or(Err(ElementError::InvalidId(listbox_id.to_owned())))?;
        let elem = body.document().select(&selector).next().ok_or(ElementError::NoSuchElement)?;
        Ok(ListBoxes::from_elements(Elements::dyn_elem(elem)?).ok_or(ElementError::NoSuchElement)?)
    }

    pub fn select(&self, key: &str, by_enter: bool) -> Result<Event> {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("Key".to_string(), key.to_string());
        parameters.insert("ByEnter".to_string(), by_enter.to_string());
        self.fire_event("Select".to_string(), parameters)
    }
}
