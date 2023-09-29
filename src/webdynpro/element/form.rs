use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell};

use indexmap::IndexMap;
use serde::Deserialize;

use crate::webdynpro::event::Event;

use super::{Element, ElementDef, EventParameterMap};

#[derive(Debug)]
#[allow(unused)]
pub struct Form<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<FormLSData>>,
    lsevents: OnceCell<Option<EventParameterMap>>,
    data: OnceCell<Option<FormData>>,
}

impl<'a> Element<'a> for Form<'a> {
    const CONTROL_ID: &'static str = "FOR";

    const ELEMENT_NAME: &'static str = "Form";

    type ElementLSData = FormLSData;

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

    fn wrap(self) -> super::ElementWrapper<'a> {
        super::ElementWrapper::Form(self)
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
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
            data: OnceCell::new(),
        }
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
