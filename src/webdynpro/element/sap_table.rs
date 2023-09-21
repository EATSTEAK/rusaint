use indexmap::IndexMap;
use serde::Deserialize;

use crate::webdynpro::{event::Event, application::client::body::Body, error::{BodyError, ElementError}};

use super::{Element, EventParameterMap, ElementDef};

pub struct SapTable<'a> {
    id: &'a str,
    lsdata: Option<SapTableLSData>,
    lsevents: Option<EventParameterMap>
}

#[derive(Deserialize, Debug, Default)]
pub struct SapTableLSData {
    #[serde(rename = "0")]
    title_text: Option<String>,
    #[serde(rename = "1")]
    accessibility_description: Option<String>,
    #[serde(rename = "2")]
    row_count: Option<u32>,
    #[serde(rename = "3")]
    col_count: Option<u32>,
}

impl<'a> Element for SapTable<'a> {
    const CONTROL_ID: &'static str = "ST";

    const ELEMENT_NAME: &'static str = "SapTable";

    type ElementLSData = SapTableLSData;

    fn lsdata(&self) -> Option<&Self::ElementLSData> {
        self.lsdata.as_ref()
    }

    fn lsevents(&self) -> Option<&EventParameterMap> {
        self.lsevents.as_ref()
    }
}

impl<'a> ElementDef<'a, SapTable<'a>> {
    pub fn elem(&self, body: &'_ Body) -> Result<SapTable<'a>, BodyError> {
        SapTable::from_body(self, body)
    }
}

impl<'a> SapTable<'a> {
    
    pub const fn new(id: &'a str, lsdata: Option<SapTableLSData>, lsevents: Option<EventParameterMap>) -> Self {
        Self {
            id,
            lsdata,
            lsevents
        }
    }

    pub fn from_body(elem_def: &ElementDef<'a, Self>, body: &'_ Body) -> Result<Self, BodyError> {
        let selector = &elem_def.selector().or(Err(BodyError::InvalidSelector))?;
        let lsdata_obj = Self::lsdata_elem(selector, body.document())?;
        let lsdata = serde_json::from_value::<SapTableLSData>(lsdata_obj).or(Err(ElementError::InvalidLSData))?;
        let lsevents = Self::lsevents_elem(selector, body.document())?;
        Ok(Self::new(elem_def.id, Some(lsdata), Some(lsevents)))
    }

    pub fn press(&self) -> Result<Event, ElementError> {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        self.fire_event("Press", parameters)
    }
}