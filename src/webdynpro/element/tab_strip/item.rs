use std::borrow::Cow;

use serde::Deserialize;

use crate::webdynpro::{
    element::{Element, ElementDef, EventParameterMap},
    error::{BodyError, ElementError},
};

pub struct TabStripItem {
    id: Cow<'static, str>,
    lsdata: Option<TabStripItemLSData>,
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

impl ElementDef<TabStripItem> {
    pub fn wrap(self) -> super::super::Elements {
        super::super::Elements::TabStripItem(self)
    }
}

impl Element for TabStripItem {
    // Note: This element renders as "TSITM_ie6" if >= IE6
    const CONTROL_ID: &'static str = "TSITM_standards";

    // Unused
    const ELEMENT_NAME: &'static str = "TabStripTab";

    type ElementLSData = TabStripItemLSData;

    fn lsdata(&self) -> Option<&Self::ElementLSData> {
        self.lsdata.as_ref()
    }

    fn lsevents(&self) -> Option<&EventParameterMap> {
        None
    }

    fn from_elem(
        elem_def: ElementDef<Self>,
        element: scraper::ElementRef,
    ) -> Result<Self, BodyError> {
        let lsdata_obj = Self::lsdata_elem(element)?;
        let lsdata = serde_json::from_value::<Self::ElementLSData>(lsdata_obj)
            .or(Err(ElementError::InvalidLSData))?;
        Ok(Self::new(elem_def.id.to_owned(), Some(lsdata)))
    }
}

impl TabStripItem {
    pub const fn new(id: Cow<'static, str>, lsdata: Option<TabStripItemLSData>) -> Self {
        Self { id, lsdata }
    }
}
