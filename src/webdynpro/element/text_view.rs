use anyhow::Result;
use std::{borrow::Cow, collections::HashMap};

use indexmap::IndexMap;
use serde::Deserialize;

use crate::webdynpro::{
    error::ElementError,
    event::{ucf_parameters::UcfParameters, Event},
};

use super::{Element, ElementDef, EventParameterMap};

#[derive(Debug)]
pub struct TextView<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: Option<TextViewLSData>,
    lsevents: Option<HashMap<String, (UcfParameters, IndexMap<String, String>)>>,
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

    fn lsdata(&self) -> Option<&TextViewLSData> {
        self.lsdata.as_ref()
    }

    fn lsevents(&self) -> Option<&EventParameterMap> {
        self.lsevents.as_ref()
    }

    fn from_elem(elem_def: ElementDef<'a, Self>, element: scraper::ElementRef<'a>) -> Result<Self> {
        let lsdata_obj = Self::lsdata_elem(element)?;
        let lsdata = serde_json::from_value::<Self::ElementLSData>(lsdata_obj)
            .or(Err(ElementError::InvalidLSData))?;
        let lsevents = Self::lsevents_elem(element).ok();
        Ok(Self::new(elem_def.id.to_owned(), Some(lsdata), lsevents))
    }
}

impl<'a> TextView<'a> {
    pub fn new(
        id: Cow<'static, str>,
        element_ref: scraper::ElementRef<'a>,
        lsdata: Option<TextViewLSData>,
        lsevents: Option<EventParameterMap>,
    ) -> Self {
        Self {
            id,
            element_ref,
            lsdata,
            lsevents,
        }
    }

    pub fn wrap(self) -> super::Elements {
        super::Elements::TextView(self)
    }
}
