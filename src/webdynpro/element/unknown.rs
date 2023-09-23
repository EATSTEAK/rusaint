use std::borrow::Cow;

use serde_json::Value;

use crate::webdynpro::error::BodyError;

use super::{Element, ElementDef, EventParameterMap};

// Type for unimplemented elements
#[derive(Debug)]
pub struct Unknown {
    id: Cow<'static, str>,
    lsdata: Option<Value>,
    lsevents: Option<EventParameterMap>,
}

impl ElementDef<Unknown> {
    pub fn wrap(self) -> super::Elements {
        super::Elements::Unknown(self)
    }
}

impl Element for Unknown {
    const CONTROL_ID: &'static str = "UNKNOWN";

    const ELEMENT_NAME: &'static str = "Unknown";

    type ElementLSData = Value;

    fn from_elem(
        elem_def: ElementDef<Self>,
        element: scraper::ElementRef,
    ) -> Result<Self, BodyError> {
        let lsdata_obj = Self::lsdata_elem(element).ok();
        let lsevents = Self::lsevents_elem(element).ok();
        Ok(Self::new(elem_def.id.to_owned(), lsdata_obj, lsevents))
    }

    fn lsdata(&self) -> Option<&Self::ElementLSData> {
        self.lsdata.as_ref()
    }

    fn lsevents(&self) -> Option<&super::EventParameterMap> {
        self.lsevents.as_ref()
    }
}

impl Unknown {
    pub fn new(
        id: Cow<'static, str>,
        lsdata: Option<Value>,
        lsevents: Option<EventParameterMap>,
    ) -> Self {
        Self {
            id,
            lsdata,
            lsevents,
        }
    }
}
