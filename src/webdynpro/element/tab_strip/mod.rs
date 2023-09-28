use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell};

use indexmap::IndexMap;
use scraper::Selector;
use serde::Deserialize;

use crate::webdynpro::{error::BodyError, event::Event};

use self::item::TabStripItem;
use super::{Element, ElementDef, EventParameterMap};

type TabItems<'a> = Vec<ElementDef<'a, TabStripItem<'a>>>;

#[derive(Debug)]
pub struct TabStrip<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<TabStripLSData>>,
    lsevents: OnceCell<Option<EventParameterMap>>,
    tab_items: OnceCell<Option<TabItems<'a>>>,
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

impl<'a> Element<'a> for TabStrip<'a> {
    // Note: This element renders as "TS_ie6" if >= IE6
    const CONTROL_ID: &'static str = "TS_standards";

    const ELEMENT_NAME: &'static str = "TabStrip";

    type ElementLSData = TabStripLSData;

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

    fn wrap(self) -> super::Elements<'a> {
        super::Elements::TabStrip(self)
    }
}

impl<'a> TabStrip<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
            tab_items: OnceCell::new(),
        }
    }

    pub fn tab_items(&self) -> Option<&TabItems<'a>> {
        self.tab_items
            .get_or_init(|| {
                let items_selector =
                    Selector::parse(format!(r#"[ct="{}"]"#, TabStripItem::CONTROL_ID).as_str())
                        .or(Err(BodyError::InvalidSelector))
                        .ok()?;
                Some(
                    self.element_ref
                        .select(&items_selector)
                        .filter_map(|eref| {
                            let id = eref.value().id()?;
                            Some(ElementDef::<TabStripItem>::new_dynamic(id.to_owned()))
                        })
                        .collect(),
                )
            })
            .as_ref()
    }

    pub fn tab_select(
        &self,
        item_id: &str,
        item_index: u32,
        first_visible_item_index: u32,
    ) -> Result<Event> {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("ItemId".to_string(), item_id.to_string());
        parameters.insert("ItemIndex".to_string(), item_index.to_string());
        parameters.insert(
            "FirstVisibleItemIndex".to_string(),
            first_visible_item_index.to_string(),
        );
        self.fire_event("TabSelect".to_string(), parameters)
    }
}

pub mod item;
