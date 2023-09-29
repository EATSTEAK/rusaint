use std::{borrow::Cow, cell::OnceCell};

use anyhow::Result;
use serde::Deserialize;
use crate::webdynpro::element::{EventParameterMap, Element, ElementDef, Elements};

// TODO: Implement additional events and data
#[derive(Debug)]
pub struct ScrollContainer<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<ScrollContainerLSData>>,
    lsevents: OnceCell<Option<EventParameterMap>>,
}
impl<'a> Element<'a> for ScrollContainer<'a> {
    const CONTROL_ID: &'static str = "SC";

    const ELEMENT_NAME: &'static str = "ScrollContainer";

    type ElementLSData = ScrollContainerLSData;

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

    fn wrap(self) -> Elements<'a> {
        Elements::ScrollContainer(self)
    }
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct ScrollContainerLSData {
    #[serde(rename = "0")]
    is_selected: Option<bool>,
    #[serde(rename = "1")]
    is_secondary_selected: Option<bool>,
    #[serde(rename = "2")]
    enabled: Option<bool>,
    #[serde(rename = "3")]
    cell_type: Option<String>,
    #[serde(rename = "4")]
    row_description: Option<String>,
    #[serde(rename = "5")]
    is_deselectable: Option<bool>,
    #[serde(rename = "6")]
    tooltip: Option<String>,
    #[serde(rename = "7")]
    custom_data: Option<String>,
}

impl<'a> ScrollContainer<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }
}