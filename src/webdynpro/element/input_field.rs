use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::{Element, ElementDef, ElementWrapper, EventParameterMap};
use anyhow::Result;
use serde::Deserialize;

// TODO: Implement additional events and data
#[derive(Debug)]
pub struct InputField<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<InputFieldLSData>>,
    lsevents: OnceCell<Option<EventParameterMap>>,
}
impl<'a> Element<'a> for InputField<'a> {
    const CONTROL_ID: &'static str = "I";

    const ELEMENT_NAME: &'static str = "InputField";

    type ElementLSData = InputFieldLSData;

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

    fn wrap(self) -> ElementWrapper<'a> {
        ElementWrapper::InputField(self)
    }
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct InputFieldLSData {
    #[serde(rename = "0")]
    value: Option<String>,
    #[serde(rename = "1")]
    show_help_button: Option<bool>,
    // This field originally named "type"
    #[serde(rename = "2")]
    input_field_type: Option<String>,
    #[serde(rename = "3")]
    visibility: Option<String>,
    #[serde(rename = "4")]
    label_text: Option<String>,
    #[serde(rename = "5")]
    hide_field_help: Option<bool>,
    #[serde(rename = "6")]
    container_width_set: Option<bool>,
    #[serde(rename = "7")]
    ime_mode: Option<String>,
    #[serde(rename = "8")]
    auto_complete: Option<bool>,
    #[serde(rename = "9")]
    format_string: Option<String>,
    #[serde(rename = "10")]
    show_help_button_always: Option<bool>,
    #[serde(rename = "11")]
    date_picker_start_ref_id: Option<String>,
    #[serde(rename = "12")]
    access_key: Option<String>,
    #[serde(rename = "13")]
    display_as_text: Option<bool>,
    #[serde(rename = "14")]
    text_style: Option<String>,
    #[serde(rename = "15")]
    spinner_increment: Option<i32>,
    #[serde(rename = "16")]
    spinner_bounds_check: Option<bool>,
    #[serde(rename = "17")]
    spinner_max: Option<i32>,
    #[serde(rename = "18")]
    spinner_min: Option<i32>,
    #[serde(rename = "19")]
    sap_table_field_design: Option<String>,
    #[serde(rename = "20")]
    validation_trigger: Option<String>,
    #[serde(rename = "21")]
    tab_behaviour: Option<String>,
    #[serde(rename = "22")]
    semantic_color: Option<String>,
    #[serde(rename = "23")]
    embedding_behaviour: Option<String>,
    #[serde(rename = "24")]
    field_help_floating: Option<bool>,
    #[serde(rename = "25")]
    first_day_of_week: Option<i32>,
    #[serde(rename = "26")]
    custom_data: Option<String>,
    #[serde(rename = "27")]
    custom_style: Option<String>,
    #[serde(rename = "28")]
    field_help_embedding: Option<bool>,
    #[serde(rename = "29")]
    labelled_by: Option<String>,
    #[serde(rename = "30")]
    described_by: Option<String>,
}

impl<'a> InputField<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }
}
