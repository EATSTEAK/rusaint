use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell};

use serde::Deserialize;

use crate::webdynpro::element::Elements;

use super::{Element, ElementDef, EventParameterMap};

#[derive(Debug)]
pub struct ListBoxItem<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<ListBoxItemLSData>>,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct ListBoxItemLSData {
    #[serde(rename = "0")]
    visibility: Option<String>,
    #[serde(rename = "1")]
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
        }
    }
}
