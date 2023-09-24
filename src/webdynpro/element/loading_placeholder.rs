use anyhow::Result;
use std::borrow::Cow;

use indexmap::IndexMap;
use serde::Deserialize;

use crate::webdynpro::{error::ElementError, event::Event};

use super::{Element, ElementDef, EventParameterMap};

#[derive(Debug)]
pub struct LoadingPlaceholder<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: Option<LoadingPlaceholderLSData>,
    lsevents: Option<EventParameterMap>,
}
impl<'a> Element<'a> for LoadingPlaceholder<'a> {
    const CONTROL_ID: &'static str = "LP";

    const ELEMENT_NAME: &'static str = "LoadingPlaceHolder";

    type ElementLSData = LoadingPlaceholderLSData;

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
        Ok(Self::new(
            elem_def.id.to_owned(),
            element,
            Some(lsdata),
            Some(lsevents),
        ))
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
    pub const fn new(
        id: Cow<'static, str>,
        element_ref: scraper::ElementRef<'a>,
        lsdata: Option<LoadingPlaceholderLSData>,
        lsevents: Option<EventParameterMap>,
    ) -> Self {
        Self {
            id,
            element_ref,
            lsdata,
            lsevents,
        }
    }

    pub fn wrap(self) -> super::Elements<'a> {
        super::Elements::LoadingPlaceholder(self)
    }

    pub fn load(&self) -> Result<Event> {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        self.fire_event("Load".to_string(), parameters)
    }
}
