use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::{Element, ElementDef, Elements, EventParameterMap};
use anyhow::Result;
use serde::Deserialize;

// TODO: Implement additional events and data
#[derive(Debug)]
pub struct Scrollbar<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<ScrollbarLSData>>,
    lsevents: OnceCell<Option<EventParameterMap>>,
}
impl<'a> Element<'a> for Scrollbar<'a> {
    const CONTROL_ID: &'static str = "SCB";

    const ELEMENT_NAME: &'static str = "Scrollbar";

    type ElementLSData = ScrollbarLSData;

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
        Elements::Scrollbar(self)
    }
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct ScrollbarLSData {
    #[serde(rename = "0")]
    value: Option<i32>,
    #[serde(rename = "1")]
    maximum: Option<i32>,
    #[serde(rename = "2")]
    minimum: Option<i32>,
    #[serde(rename = "3")]
    large_change: Option<i32>,
    #[serde(rename = "4")]
    small_change: Option<i32>,
    #[serde(rename = "5")]
    scroll_direction: Option<String>,
    #[serde(rename = "6")]
    scrolled_element_id: Option<String>,
    #[serde(rename = "7")]
    show_scroll_tip: Option<bool>,
    #[serde(rename = "8")]
    scroll_tip_value_description: Option<String>,
    #[serde(rename = "9")]
    enabled: Option<bool>,
    #[serde(rename = "10")]
    item_count: Option<i32>,
    #[serde(rename = "11")]
    custom_data: Option<String>,
    #[serde(rename = "12")]
    visibility: Option<String>,
}

impl<'a> Scrollbar<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }
}
