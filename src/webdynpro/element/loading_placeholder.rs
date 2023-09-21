use indexmap::IndexMap;
use serde::Deserialize;

use crate::webdynpro::{event::Event, application::client::body::Body, error::{BodyError, ElementError}};

use super::{Element, EventParameterMap, ElementDef};

pub struct LoadingPlaceholder<'a> {
    id: &'a str,
    lsdata: Option<LoadingPlaceholderLSData>,
    lsevents: Option<EventParameterMap>
}

impl<'a> Element for LoadingPlaceholder<'a> {
    const CONTROL_ID: &'static str = "LP";

    const ELEMENT_NAME: &'static str = "LoadingPlaceHolder";

    type ElementLSData = LoadingPlaceholderLSData;

    fn lsdata(&self) -> Option<&Self::ElementLSData> {
        self.lsdata.as_ref()
    }

    fn lsevents(&self) -> Option<&EventParameterMap> {
        self.lsevents.as_ref()
    }
}

impl<'a> ElementDef<'a, LoadingPlaceholder<'a>> {
    pub fn elem(&self, body: &'_ Body) -> Result<LoadingPlaceholder<'a>, BodyError> {
        LoadingPlaceholder::from_body(self, body)
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

    pub const fn new(id: &'a str, lsdata: Option<LoadingPlaceholderLSData>, lsevents: Option<EventParameterMap>) -> Self {
        Self {
            id,
            lsdata,
            lsevents
        }
    }

    pub fn from_body(elem_def: &ElementDef<'a, Self>, body: &'_ Body) -> Result<Self, BodyError> {
        let selector = &elem_def.selector().or(Err(BodyError::InvalidSelector))?;
        let lsdata_obj = Self::lsdata_elem(selector, body.document())?;
        let lsdata = serde_json::from_value::<LoadingPlaceholderLSData>(lsdata_obj).or(Err(ElementError::InvalidLSData))?;
        let lsevents = Self::lsevents_elem(selector, body.document())?;
        Ok(Self::new(elem_def.id, Some(lsdata), Some(lsevents)))
    }

    pub fn load(&self) -> Result<Event, ElementError> {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        self.fire_event("Load", parameters)
    }
}