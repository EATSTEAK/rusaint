use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell};

use serde::Deserialize;

use crate::webdynpro::element::Elements;

use super::{Element, ElementDef, EventParameterMap};


pub struct ListBoxItem<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<ListBoxItemLSData>>,
    index: OnceCell<Option<&'a str>>,
    key: OnceCell<Option<&'a str>>,
    tooltip: OnceCell<Option<&'a str>>,
    value1: OnceCell<Option<&'a str>>,
    value2: OnceCell<Option<&'a str>>,
    selected: OnceCell<Option<bool>>,
    icon_tooltip: OnceCell<Option<&'a str>>,
    enabled: OnceCell<Option<bool>>,
    group_title: OnceCell<Option<&'a str>>,
    title: OnceCell<&'a str>,
}

impl<'a> std::fmt::Debug for ListBoxItem<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListBoxItem")
            .field("id", &self.id())
            .field("lsdata", &self.lsdata())
            .field("index", &self.index())
            .field("key", &self.key())
            .field("tooltip", &self.tooltip())
            .field("value1", &self.value1())
            .field("value2", &self.value2())
            .field("selected", &self.selected())
            .field("icon_tooltip", &self.icon_tooltip())
            .field("enabled", &self.enabled())
            .field("group_title", &self.group_title())
            .field("title", &self.title())
            .finish()
    }
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct ListBoxItemLSData {
    #[serde(rename = "0")]
    icon_src: Option<String>,
    #[serde(rename = "1")]
    disabled_icon_src: Option<String>,
    #[serde(rename = "2")]
    semantic_text_color: Option<String>,
    #[serde(rename = "3")]
    is_deletable: Option<bool>,
    #[serde(rename = "4")]
    custom_data: Option<String>,
}

impl<'a> Element<'a> for ListBoxItem<'a> {
    const CONTROL_ID: &'static str = "LIB_I";
    const ELEMENT_NAME: &'static str = "ListBoxItem";

    type ElementLSData = ListBoxItemLSData;

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

    fn wrap(self) -> Elements<'a> {
        Elements::ListBoxItem(self)
    }
}

impl<'a> ListBoxItem<'a> {
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            title: OnceCell::new(),
            index: OnceCell::new(),
            key: OnceCell::new(),
            tooltip: OnceCell::new(),
            value1: OnceCell::new(),
            value2: OnceCell::new(),
            selected: OnceCell::new(),
            icon_tooltip: OnceCell::new(),
            enabled: OnceCell::new(),
            group_title: OnceCell::new(),
        }
    }

    pub fn index(&self) -> Option<&str> {
        self.index
            .get_or_init(|| self.element_ref.value().attr("data-itemindex"))
            .to_owned()
    }

    pub fn key(&self) -> Option<&str> {
        self.key
            .get_or_init(|| self.element_ref.value().attr("data-itemkey"))
            .to_owned()
    }

    pub fn tooltip(&self) -> Option<&str> {
        self.tooltip
            .get_or_init(|| self.element_ref.value().attr("data-itemtooltip"))
            .to_owned()
    }

    pub fn value1(&self) -> Option<&str> {
        self.value1
            .get_or_init(|| self.element_ref.value().attr("data-itemvalue1"))
            .to_owned()
    }

    pub fn value2(&self) -> Option<&str> {
        self.value2
            .get_or_init(|| self.element_ref.value().attr("data-itemvalue2"))
            .to_owned()
    }

    pub fn selected(&self) -> Option<bool> {
        self.selected
            .get_or_init(|| {
                self.element_ref
                    .value()
                    .attr("aria-selected")
                    .and_then(|str| str.parse::<bool>().ok())
            })
            .to_owned()
    }

    pub fn icon_tooltip(&self) -> Option<&str> {
        self.icon_tooltip
            .get_or_init(|| self.element_ref.value().attr("data-itemicontooltip"))
            .to_owned()
    }

    pub fn enabled(&self) -> Option<bool> {
        self.enabled
            .get_or_init(|| {
                self.element_ref
                    .value()
                    .attr("data-itemdisabled")
                    .and_then(|str| str.parse::<bool>().ok().and_then(|b| Some(!b)))
            })
            .to_owned()
    }

    pub fn group_title(&self) -> Option<&str> {
        self.group_title
            .get_or_init(|| self.element_ref.value().attr("data-itemgrouptitle"))
            .to_owned()
    }

    pub fn title(&self) -> &str {
        self.title
            .get_or_init(|| self.element_ref.value().attr("title").unwrap_or(""))
    }
}
