use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell};

use indexmap::IndexMap;
use serde::Deserialize;

use crate::webdynpro::event::Event;

use super::{Element, ElementDef, EventParameterMap};

#[derive(Debug)]
pub struct Link<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<LinkLSData>>,
    lsevents: OnceCell<Option<EventParameterMap>>,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct LinkLSData {
    #[serde(rename = "0")]
    tooltip: Option<String>,
    #[serde(rename = "1")]
    text: Option<String>,
    #[serde(rename = "2")]
    has_reference: Option<bool>,
    #[serde(rename = "3")]
    enabled: Option<bool>,
    #[serde(rename = "4")]
    has_link_caption: Option<bool>,
    #[serde(rename = "5")]
    visibility: Option<String>,
    #[serde(rename = "6")]
    label_text: Option<String>,
    #[serde(rename = "7")]
    emphasized: Option<bool>,
    #[serde(rename = "8")]
    access_key: Option<String>,
    #[serde(rename = "9")]
    hotkey: Option<String>,
    #[serde(rename = "10")]
    custom_data: Option<String>,
    #[serde(rename = "11")]
    custom_style: Option<String>,
    #[serde(rename = "12")]
    labelled_by: Option<String>,
}

impl<'a> Element<'a> for Link<'a> {
    const CONTROL_ID: &'static str = "B";

    const ELEMENT_NAME: &'static str = "Link";

    type ElementLSData = LinkLSData;

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
        super::Elements::Link(self)
    }
}

impl<'a> Link<'a> {
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }

    pub fn activate(&self, ctrl: bool, shift: bool) -> Result<Event> {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("Ctrl".to_string(), ctrl.to_string());
        parameters.insert("Shift".to_string(), shift.to_string());
        self.fire_event("Activate".to_string(), parameters)
    }

    pub fn double_click(&self) -> Result<Event> {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        self.fire_event("DoubleClick".to_string(), parameters)
    }
}
