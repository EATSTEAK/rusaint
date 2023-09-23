use std::borrow::Cow;

use indexmap::IndexMap;
use scraper::Selector;
use serde::Deserialize;

use crate::webdynpro::{
    error::{BodyError, ElementError},
    event::Event,
};

use self::item::TabStripItem;
use super::{Element, ElementDef, EventParameterMap};

type TabItems = Vec<ElementDef<TabStripItem>>;

pub struct TabStrip {
    id: Cow<'static, str>,
    lsdata: Option<TabStripLSData>,
    lsevents: Option<EventParameterMap>,
    tab_items: Option<TabItems>,
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct TabStripLSData {
    #[serde(rename = "0")]
    current_index: Option<i32>,
    #[serde(rename = "1")]
    height: Option<String>,
    #[serde(rename = "2")]
    width: Option<String>,
    #[serde(rename = "3")]
    accessibility_description: Option<String>,
    #[serde(rename = "4")]
    visibility: Option<String>,
    #[serde(rename = "5")]
    first_visible_item_idx: Option<i32>,
    #[serde(rename = "6")]
    scrollable: Option<bool>,
    #[serde(rename = "7")]
    exact_tab_alignment: Option<bool>,
    #[serde(rename = "8")]
    client_tab_select: Option<bool>,
    #[serde(rename = "9")]
    drag_source_info: Option<String>,
    #[serde(rename = "10")]
    drop_target_info: Option<String>,
    #[serde(rename = "11")]
    tab_items_position: Option<String>,
    #[serde(rename = "12")]
    custom_data: Option<String>,
    #[serde(rename = "13")]
    custom_style: Option<String>,
    #[serde(rename = "14")]
    tab_items_design: Option<String>,
    #[serde(rename = "15")]
    heading_level: Option<i32>,
}

impl ElementDef<TabStrip> {
    pub fn wrap(self) -> super::Elements {
        super::Elements::TabStrip(self)
    }
}

impl Element for TabStrip {
    // Note: This element renders as "TS_ie6" if >= IE6
    const CONTROL_ID: &'static str = "TS_standards";

    const ELEMENT_NAME: &'static str = "TabStrip";

    type ElementLSData = TabStripLSData;

    fn lsdata(&self) -> Option<&Self::ElementLSData> {
        self.lsdata.as_ref()
    }

    fn lsevents(&self) -> Option<&EventParameterMap> {
        self.lsevents.as_ref()
    }

    fn from_elem(
        elem_def: ElementDef<Self>,
        element: scraper::ElementRef,
    ) -> Result<Self, BodyError> {
        let lsdata_obj = Self::lsdata_elem(element)?;
        let lsdata = serde_json::from_value::<Self::ElementLSData>(lsdata_obj)
            .or(Err(ElementError::InvalidLSData))?;
        let lsevents = Self::lsevents_elem(element)?;
        let items_selector =
            Selector::parse(format!(r#"[ct="{}"]"#, TabStripItem::CONTROL_ID).as_str())
                .or(Err(BodyError::InvalidSelector))?;
        let tab_items: TabItems = element
            .select(&items_selector)
            .filter_map(|eref| {
                let id = eref.value().id()?;
                Some(ElementDef::<TabStripItem>::new_dynamic(id.to_owned()))
            })
            .collect();
        Ok(Self::new(
            elem_def.id.to_owned(),
            Some(lsdata),
            Some(lsevents),
            Some(tab_items),
        ))
    }
}

impl TabStrip {
    pub const fn new(
        id: Cow<'static, str>,
        lsdata: Option<TabStripLSData>,
        lsevents: Option<EventParameterMap>,
        tab_items: Option<TabItems>,
    ) -> Self {
        Self {
            id,
            lsdata,
            lsevents,
            tab_items,
        }
    }

    pub fn tab_select(
        &self,
        item_id: &str,
        item_index: u32,
        first_visible_item_index: u32,
    ) -> Result<Event, ElementError> {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("ItemId".to_string(), item_id.to_string());
        parameters.insert("ItemIndex".to_string(), item_index.to_string());
        parameters.insert(
            "FirstVisibleItemIndex".to_string(),
            first_visible_item_index.to_string(),
        );
        self.fire_event("TabSelect", parameters)
    }
}

pub mod item;
