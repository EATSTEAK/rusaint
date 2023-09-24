use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell};

use indexmap::IndexMap;
use serde::Deserialize;

use crate::webdynpro::event::Event;

use super::{Element, ElementDef, EventParameterMap};

#[derive(Debug)]
pub struct Button<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<ButtonLSData>>,
    lsevents: OnceCell<Option<EventParameterMap>>,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct ButtonLSData {
    #[serde(rename = "0")]
    text: Option<String>,
    #[serde(rename = "1")]
    text_design: Option<String>,
    #[serde(rename = "2")]
    design: Option<String>,
    #[serde(rename = "3")]
    width: Option<String>,
    #[serde(rename = "4")]
    tooltip: Option<String>,
    #[serde(rename = "5")]
    enabled: Option<bool>,
    #[serde(rename = "6")]
    has_button_caption: Option<bool>,
    #[serde(rename = "7")]
    visibility: Option<String>,
    #[serde(rename = "8")]
    show_help: Option<bool>,
    #[serde(rename = "9")]
    down: Option<bool>,
    #[serde(rename = "10")]
    has_icon: Option<bool>,
    #[serde(rename = "11")]
    disabled_icon_src: Option<String>,
    #[serde(rename = "12")]
    up_icon_src: Option<String>,
    #[serde(rename = "13")]
    down_icon_src: Option<String>,
    #[serde(rename = "14")]
    has_popup_menu: Option<bool>,
    #[serde(rename = "15")]
    popup_menu_id: Option<String>,
    #[serde(rename = "16")]
    has_popup_menu_section: Option<bool>,
    #[serde(rename = "17")]
    image_first: Option<bool>,
    #[serde(rename = "18")]
    access_key: Option<String>,
    #[serde(rename = "19")]
    hotkey: Option<String>,
    #[serde(rename = "20")]
    up: Option<bool>,
    #[serde(rename = "21")]
    text_overflow: Option<bool>,
    #[serde(rename = "22")]
    fixed_height: Option<bool>,
    // Note: this field originally named as "type"
    #[serde(rename = "23")]
    element_type: Option<String>,
    #[serde(rename = "24")]
    drag_source_info: Option<String>,
    #[serde(rename = "25")]
    semantic_color: Option<String>,
    #[serde(rename = "26")]
    interaction_behaviour: Option<String>,
    #[serde(rename = "27")]
    custom_style: Option<String>,
    #[serde(rename = "28")]
    custom_data: Option<String>,
    #[serde(rename = "29")]
    wrapping: Option<bool>,
    #[serde(rename = "30")]
    height: Option<String>,
    #[serde(rename = "31")]
    content_visibility: Option<String>,
}

impl<'a> Element<'a> for Button<'a> {
    const CONTROL_ID: &'static str = "B";

    const ELEMENT_NAME: &'static str = "Button";

    type ElementLSData = ButtonLSData;

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
}

impl<'a> Button<'a> {
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }

    pub fn wrap(self) -> super::Elements<'a> {
        super::Elements::Button(self)
    }

    pub fn press(&self) -> Result<Event> {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        self.fire_event("Press".to_string(), parameters)
    }
}
