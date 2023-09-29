use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell};

use serde::Deserialize;

use crate::webdynpro::element::ElementWrapper;

use super::{Element, ElementDef, EventParameterMap};

pub struct ListBoxActionItem<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<ListBoxActionItemLSData>>,
    title: OnceCell<String>,
    text: OnceCell<String>,
}

impl<'a> std::fmt::Debug for ListBoxActionItem<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListBoxActionItem")
            .field("id", &self.id())
            .field("lsdata", &self.lsdata())
            .field("text", &self.text())
            .field("title", &self.title())
            .finish()
    }
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct ListBoxActionItemLSData {
    #[serde(rename = "0")]
    custom_data: Option<String>,
}

impl<'a> Element<'a> for ListBoxActionItem<'a> {
    const CONTROL_ID: &'static str = "LIB_AI";
    const ELEMENT_NAME: &'static str = "ListBoxActionItem";

    type ElementLSData = ListBoxActionItemLSData;

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

    fn wrap(self) -> ElementWrapper<'a> {
        ElementWrapper::ListBoxActionItem(self)
    }
}

impl<'a> ListBoxActionItem<'a> {
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            title: OnceCell::new(),
            text: OnceCell::new(),
        }
    }

    pub fn title(&self) -> &str {
        self.title.get_or_init(|| {
            self.element_ref
                .value()
                .attr("title")
                .unwrap_or("")
                .to_owned()
        })
    }

    pub fn text(&self) -> &str {
        self.text
            .get_or_init(|| self.element_ref().text().collect::<String>())
    }
}
