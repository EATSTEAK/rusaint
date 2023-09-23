use anyhow::Result;
use std::borrow::Cow;

use indexmap::IndexMap;
use serde::Deserialize;

use crate::webdynpro::{error::ElementError, event::Event};

use super::{Element, ElementDef, EventParameterMap};

#[derive(Debug)]
pub struct LoadingPlaceholder {
    id: Cow<'static, str>,
    lsdata: Option<LoadingPlaceholderLSData>,
    lsevents: Option<EventParameterMap>,
}

impl ElementDef<LoadingPlaceholder> {
    pub fn wrap(self) -> super::Elements {
        super::Elements::LoadingPlaceholder(self)
    }
}

impl Element for LoadingPlaceholder {
    const CONTROL_ID: &'static str = "LP";

    const ELEMENT_NAME: &'static str = "LoadingPlaceHolder";

    type ElementLSData = LoadingPlaceholderLSData;

    fn lsdata(&self) -> Option<&Self::ElementLSData> {
        self.lsdata.as_ref()
    }

    fn lsevents(&self) -> Option<&EventParameterMap> {
        self.lsevents.as_ref()
    }

    fn from_elem(elem_def: ElementDef<Self>, element: scraper::ElementRef) -> Result<Self> {
        let lsdata_obj = Self::lsdata_elem(element)?;
        let lsdata = serde_json::from_value::<Self::ElementLSData>(lsdata_obj)
            .or(Err(ElementError::InvalidLSData))?;
        let lsevents = Self::lsevents_elem(element)?;
        Ok(Self::new(
            elem_def.id.to_owned(),
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

impl LoadingPlaceholder {
    pub const fn new(
        id: Cow<'static, str>,
        lsdata: Option<LoadingPlaceholderLSData>,
        lsevents: Option<EventParameterMap>,
    ) -> Self {
        Self {
            id,
            lsdata,
            lsevents,
        }
    }

    pub fn load(&self) -> Result<Event> {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        self.fire_event("Load", parameters)
    }
}
