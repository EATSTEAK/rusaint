use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell};

use serde::Deserialize;

use crate::webdynpro::element::{Element, ElementDef, EventParameterMap};

#[derive(Debug)]
pub struct TabStripItem<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<TabStripItemLSData>>,
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct TabStripItemLSData {
    #[serde(rename = "0")]
    id: Option<String>,
    #[serde(rename = "1")]
    index: Option<i32>,
    #[serde(rename = "2")]
    caption: Option<String>,
    #[serde(rename = "3")]
    has_title_caption: Option<bool>,
    #[serde(rename = "4")]
    tooltip: Option<String>,
    #[serde(rename = "5")]
    enabled: Option<bool>,
    #[serde(rename = "6")]
    scrolling_mode: Option<String>,
    #[serde(rename = "7")]
    has_toolbar: Option<bool>,
    #[serde(rename = "8")]
    default_button_id: Option<String>,
    #[serde(rename = "9")]
    is_closable: Option<bool>,
    #[serde(rename = "10")]
    scroll_top: Option<i32>,
    #[serde(rename = "11")]
    scroll_left: Option<i32>,
    #[serde(rename = "12")]
    client_tab_select: Option<bool>,
    #[serde(rename = "13")]
    hotkeys_id: Option<String>,
    #[serde(rename = "14")]
    access_key: Option<String>,
    #[serde(rename = "15")]
    has_editable_title: Option<bool>,
    #[serde(rename = "16")]
    area_design: Option<String>,
    #[serde(rename = "17")]
    custom_data: Option<String>,
    #[serde(rename = "18")]
    custom_style: Option<String>,
    #[serde(rename = "19")]
    visibility: Option<String>,
}

impl<'a> Element<'a> for TabStripItem<'a> {
    // Note: This element renders as "TSITM_ie6" if >= IE6
    const CONTROL_ID: &'static str = "TSITM_standards";

    // Unused
    const ELEMENT_NAME: &'static str = "TabStripTab";

    type ElementLSData = TabStripItemLSData;

    fn lsdata(&self) -> Option<&Self::ElementLSData> {
        self.lsdata
            .get_or_init(|| {
                let lsdata_obj = Self::lsdata_elem(self.element_ref).ok()?;
                serde_json::from_value::<Self::ElementLSData>(lsdata_obj).ok()
            })
            .as_ref()
    }

    fn lsevents(&self) -> Option<&EventParameterMap> {
        None
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

    fn wrap(self) -> super::super::Elements<'a> {
        super::super::Elements::TabStripItem(self)
    }
}

impl<'a> TabStripItem<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
        }
    }
}
