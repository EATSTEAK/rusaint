use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell};

use serde::Deserialize;

use super::{Element, ElementDef, EventParameterMap};

#[derive(Debug)]
pub struct TextView<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<TextViewLSData>>,
    lsevents: OnceCell<Option<EventParameterMap>>,
    text: OnceCell<String>,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct TextViewLSData {
    #[serde(rename = "0")]
    tooltip: Option<String>,
    #[serde(rename = "1")]
    required: Option<bool>,
    #[serde(rename = "2")]
    enabled: Option<bool>,
    #[serde(rename = "3")]
    design: Option<String>,
    #[serde(rename = "4")]
    layout: Option<String>,
    #[serde(rename = "5")]
    semantic_color: Option<String>,
    #[serde(rename = "6")]
    semantic_bg_color: Option<String>,
    #[serde(rename = "7")]
    is_nested: Option<bool>,
    #[serde(rename = "8")]
    visibility: Option<String>,
    #[serde(rename = "9")]
    text_overflow: Option<bool>,
}
impl<'a> Element<'a> for TextView<'a> {
    const CONTROL_ID: &'static str = "TV";

    const ELEMENT_NAME: &'static str = "TextView";

    type ElementLSData = TextViewLSData;

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
        super::Elements::TextView(self)
    }
}

impl<'a> TextView<'a> {
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
            text: OnceCell::new(),
        }
    }

    pub fn text(&self) -> String {
        self.text
            .get_or_init(|| self.element_ref().text().collect::<String>())
            .to_owned()
    }
}
