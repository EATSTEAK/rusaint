use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::{Element, ElementDef, ElementWrapper, EventParameterMap};
use anyhow::Result;
use serde::Deserialize;

// TODO: Implement additional events and data
#[derive(Debug)]
pub struct GridLayoutCell<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<GridLayoutCellLSData>>,
    lsevents: OnceCell<Option<EventParameterMap>>,
}
impl<'a> Element<'a> for GridLayoutCell<'a> {
    const CONTROL_ID: &'static str = "GLC";

    const ELEMENT_NAME: &'static str = "GridLayoutCell";

    type ElementLSData = GridLayoutCellLSData;

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

    fn wrap(self) -> ElementWrapper<'a> {
        ElementWrapper::GridLayoutCell(self)
    }
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct GridLayoutCellLSData {
    #[serde(rename = "0")]
    drag_data: Option<String>,
    #[serde(rename = "1")]
    semantic_color: Option<String>,
    #[serde(rename = "2")]
    custom_data: Option<String>,
    #[serde(rename = "3")]
    layout_cell_position: Option<String>,
    #[serde(rename = "4")]
    custom_style: Option<String>,
}

impl<'a> GridLayoutCell<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }
}
