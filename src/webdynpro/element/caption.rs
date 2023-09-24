use anyhow::Result;
use getset::Getters;
use std::{borrow::Cow, cell::OnceCell};

use serde::Deserialize;

use super::{Element, ElementDef, EventParameterMap};

#[derive(Debug)]
pub struct Caption<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<CaptionLSData>>,
    lsevents: OnceCell<Option<EventParameterMap>>,
    text: OnceCell<String>,
}

#[allow(unused)]
#[derive(Getters, Debug, Deserialize, Default)]
#[get = "pub"]
pub struct CaptionLSData {
    #[serde(rename = "0")]
    tooltip: Option<String>,
    #[serde(rename = "1")]
    text: Option<String>,
    #[serde(rename = "2")]
    image_src: Option<String>,
    #[serde(rename = "3")]
    image_first: Option<bool>,
    #[serde(rename = "4")]
    image_width: Option<String>,
    #[serde(rename = "5")]
    image_height: Option<String>,
    #[serde(rename = "6")]
    is_nested: Option<bool>,
    #[serde(rename = "7")]
    visibility: Option<String>,
    #[serde(rename = "8")]
    is_drag_handle: Option<bool>,
    #[serde(rename = "9")]
    hover_image_src: Option<String>,
    #[serde(rename = "10")]
    drag_source_info: Option<String>,
    #[serde(rename = "11")]
    editable: Option<bool>,
    #[serde(rename = "12")]
    semantic_color: Option<String>,
    #[serde(rename = "13")]
    design: Option<String>,
    #[serde(rename = "14")]
    custom_data: Option<String>,
    #[serde(rename = "15")]
    labelled_by: Option<String>,
}
impl<'a> Element<'a> for Caption<'a> {
    const CONTROL_ID: &'static str = "CP";

    const ELEMENT_NAME: &'static str = "Caption";

    type ElementLSData = CaptionLSData;

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
}

impl<'a> Caption<'a> {
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
            text: OnceCell::new(),
        }
    }

    pub fn text(&self) -> &str {
        self.text.get_or_init(|| {
            if let Some(lsdata) = self.lsdata() {
                lsdata.text().as_ref().unwrap_or(&"".to_string()).to_owned()
            } else {
                "".to_string()
            }
        })
    }

    pub fn wrap(self) -> super::Elements<'a> {
        super::Elements::Caption(self)
    }
}
