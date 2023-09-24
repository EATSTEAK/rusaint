use anyhow::Result;
use std::borrow::Cow;

use indexmap::IndexMap;
use serde::Deserialize;

use crate::webdynpro::{error::ElementError, event::Event};

use super::{Element, ElementDef, EventParameterMap};

#[derive(Debug)]
#[allow(unused)]
pub struct Form<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: Option<FormLSData>,
    lsevents: Option<EventParameterMap>,
    data: Option<FormData>,
}

impl<'a> Element<'a> for Form<'a> {
    const CONTROL_ID: &'static str = "FOR";

    const ELEMENT_NAME: &'static str = "Form";

    type ElementLSData = FormLSData;

    fn lsdata(&self) -> Option<&Self::ElementLSData> {
        self.lsdata.as_ref()
    }

    fn lsevents(&self) -> Option<&EventParameterMap> {
        self.lsevents.as_ref()
    }

    fn from_elem(elem_def: ElementDef<'a, Self>, element: scraper::ElementRef<'a>) -> Result<Self> {
        let lsdata_obj = Self::lsdata_elem(element)?;
        let lsdata = serde_json::from_value::<Self::ElementLSData>(lsdata_obj)
            .or(Err(ElementError::InvalidLSData))?;
        let lsevents = Self::lsevents_elem(element)?;
        let data = FormData {
            ..Default::default()
        };
        Ok(Self::new(
            elem_def.id.to_owned(),
            element,
            Some(lsdata),
            Some(lsevents),
            Some(data),
        ))
    }
}

#[derive(Debug, Default)]
#[allow(unused)]
pub struct FormData {
    name: Option<String>,
    method: Option<String>,
    action: Option<String>,
    title: Option<String>,
    accept: Option<String>,
    accept_charset: Option<String>,
    enctype: Option<String>,
    target: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct FormLSData {
    #[serde(rename = "0")]
    has_event_queue: Option<bool>,
    #[serde(rename = "1")]
    response_data: Option<String>,
    #[serde(rename = "2")]
    custom_data: Option<String>,
}

impl<'a> Form<'a> {
    pub fn new(
        id: Cow<'static, str>,
        element_ref: scraper::ElementRef<'a>,
        lsdata: Option<FormLSData>,
        lsevents: Option<EventParameterMap>,
        data: Option<FormData>,
    ) -> Self {
        Self {
            id,
            element_ref,
            lsdata,
            lsevents,
            data,
        }
    }

    pub fn wrap(self) -> super::Elements<'a> {
        super::Elements::Form(self)
    }

    pub fn request(
        &self,
        is_async: bool,
        focus_info: &str,
        hash: &str,
        dom_changed: bool,
        is_dirty: bool,
    ) -> Result<Event> {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("Async".to_string(), is_async.to_string());
        parameters.insert("FocusInfo".to_string(), focus_info.to_string());
        parameters.insert("Hash".to_string(), hash.to_string());
        parameters.insert("DomChanged".to_string(), dom_changed.to_string());
        parameters.insert("IsDirty".to_string(), is_dirty.to_string());
        self.fire_event("Request".to_string(), parameters)
    }
}
