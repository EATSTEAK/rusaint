use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell};

use indexmap::IndexMap;
use serde::Deserialize;

use crate::webdynpro::event::Event;

use super::{Element, ElementDef, EventParameterMap};

#[derive(Debug)]
pub struct LoadingPlaceholder<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<LoadingPlaceholderLSData>>,
    lsevents: OnceCell<Option<EventParameterMap>>,
}
impl<'a> Element<'a> for LoadingPlaceholder<'a> {
    const CONTROL_ID: &'static str = "LP";

    const ELEMENT_NAME: &'static str = "LoadingPlaceHolder";

    type ElementLSData = LoadingPlaceholderLSData;

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
        super::Elements::LoadingPlaceholder(self)
    }
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct LoadingPlaceholderLSData {
    #[serde(rename = "0")]
    id: Option<String>,
    #[serde(rename = "1")]
    custom_data: Option<String>,
}

impl<'a> LoadingPlaceholder<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }

    pub fn load(&self) -> Result<Event> {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        self.fire_event("Load".to_string(), parameters)
    }
}
