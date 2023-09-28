use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell};

use serde::Deserialize;

use crate::webdynpro::element::Elements;

use super::{Element, ElementDef, EventParameterMap};

#[derive(Debug)]
pub struct ListBoxActionItem<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<ListBoxActionItemLSData>>,
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

    fn wrap(self) -> Elements<'a> {
        Elements::ListBoxActionItem(self)
    }
}

impl<'a> ListBoxActionItem<'a> {
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
        }
    }
}
