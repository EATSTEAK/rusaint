use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::{Element, ElementDef, Elements, EventParameterMap};
use anyhow::Result;
use serde::Deserialize;

// TODO: Implement additional events and data
#[derive(Debug)]
pub struct Tray<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<TrayLSData>>,
    lsevents: OnceCell<Option<EventParameterMap>>,
}
impl<'a> Element<'a> for Tray<'a> {
    const CONTROL_ID: &'static str = "TY";

    const ELEMENT_NAME: &'static str = "Tray";

    type ElementLSData = TrayLSData;

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
        Elements::Tray(self)
    }
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct TrayLSData {
    #[serde(rename = "0")]
    title: Option<String>,
    #[serde(rename = "1")]
    design: Option<String>,
    #[serde(rename = "2")]
    collapsed: Option<bool>,
    #[serde(rename = "3")]
    enabled: Option<bool>,
    #[serde(rename = "4")]
    tooltip: Option<String>,
    #[serde(rename = "5")]
    height: Option<String>,
    #[serde(rename = "6")]
    content_height: Option<String>,
    #[serde(rename = "7")]
    has_option_menu: Option<bool>,
    #[serde(rename = "8")]
    option_menu_id: Option<String>,
    #[serde(rename = "9")]
    has_close_button: Option<bool>,
    #[serde(rename = "10")]
    scrolling_mode: Option<String>,
    #[serde(rename = "11")]
    has_toolbar: Option<bool>,
    #[serde(rename = "12")]
    is_collapsible: Option<bool>,
    #[serde(rename = "13")]
    accessibility_description: Option<String>,
    #[serde(rename = "14")]
    visibility: Option<String>,
    #[serde(rename = "15")]
    default_button_id: Option<String>,
    #[serde(rename = "16")]
    scroll_top: Option<i32>,
    #[serde(rename = "17")]
    scroll_left: Option<i32>,
    #[serde(rename = "18")]
    access_key: Option<String>,
    #[serde(rename = "19")]
    hotkeys_id: Option<String>,
    #[serde(rename = "20")]
    is_drag_handle: Option<bool>,
    #[serde(rename = "21")]
    client_select: Option<bool>,
    #[serde(rename = "22")]
    heading_level: Option<i32>,
    #[serde(rename = "23")]
    custom_data: Option<String>,
    #[serde(rename = "24")]
    custom_style: Option<String>,
}

impl<'a> Tray<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }
}
