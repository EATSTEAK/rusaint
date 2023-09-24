use anyhow::Result;
use std::borrow::Cow;

use serde_json::Value;

use super::{Element, ElementDef, EventParameterMap};

// Type for unimplemented elements
#[derive(Debug)]
pub struct Unknown<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: Option<Value>,
    lsevents: Option<EventParameterMap>,
}

impl<'a> Element<'a> for Unknown<'a> {
    const CONTROL_ID: &'static str = "UNKNOWN";

    const ELEMENT_NAME: &'static str = "Unknown";

    type ElementLSData = Value;

    fn from_elem(elem_def: ElementDef<'a, Self>, element: scraper::ElementRef<'a>) -> Result<Self> {
        let lsdata_obj = Self::lsdata_elem(element).ok();
        let lsevents = Self::lsevents_elem(element).ok();
        Ok(Self::new(
            elem_def.id.to_owned(),
            element,
            lsdata_obj,
            lsevents,
        ))
    }

    fn lsdata(&self) -> Option<&Self::ElementLSData> {
        self.lsdata.as_ref()
    }

    fn lsevents(&self) -> Option<&super::EventParameterMap> {
        self.lsevents.as_ref()
    }
}

impl<'a> Unknown<'a> {
    pub fn new(
        id: Cow<'static, str>,
        element_ref: scraper::ElementRef<'a>,
        lsdata: Option<Value>,
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
        super::Elements::Unknown(self)
    }
}
