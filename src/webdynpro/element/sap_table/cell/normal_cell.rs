use anyhow::Result;
use std::borrow::Cow;

use scraper::Selector;
use serde::Deserialize;

use crate::webdynpro::{
    element::{dyn_elem, Element, Elements, SubElement, SubElementDef},
    error::ElementError,
};

use super::{SapTableCell, SapTableCells};

#[derive(Debug)]
pub struct SapTableNormalCell<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: Option<SapTableNormalCellLSData>,
    contents: Vec<Elements<'a>>,
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct SapTableNormalCellLSData {
    #[serde(rename = "0")]
    is_selected: Option<bool>,
    #[serde(rename = "1")]
    is_secondary_selected: Option<bool>,
    #[serde(rename = "2")]
    cell_type: Option<String>,
    #[serde(rename = "3")]
    cell_design: Option<String>,
    #[serde(rename = "4")]
    header_cell_ids: Option<String>,
    #[serde(rename = "5")]
    row_header_cell_ids: Option<String>,
    #[serde(rename = "6")]
    custom_style: Option<String>,
    #[serde(rename = "7")]
    custom_data: Option<String>,
}

impl<'a> SapTableCell<'a> for SapTableNormalCell<'a> {
    fn content(&self) -> &Vec<Elements<'a>> {
        &self.contents
    }
}

impl<'a> SubElement<'a> for SapTableNormalCell<'a> {
    const SUBCONTROL_ID: &'static str = "STC";
    const ELEMENT_NAME: &'static str = "SapTableNormalCell";

    type SubElementLSData = SapTableNormalCellLSData;

    fn lsdata(&self) -> Option<&Self::SubElementLSData> {
        self.lsdata.as_ref()
    }

    fn from_elem<Parent: Element<'a>>(
        elem_def: SubElementDef<'a, Parent, Self>,
        element: scraper::ElementRef<'a>,
    ) -> Result<Self> {
        let lsdata_obj = Self::lsdata_elem(element)?;
        let lsdata = serde_json::from_value::<Self::SubElementLSData>(lsdata_obj)
            .or(Err(ElementError::InvalidLSData))?;
        let content_selector = Selector::parse(":root [ct]").unwrap();
        let contents: Vec<Elements> = element
            .select(&content_selector)
            .filter_map(|node| dyn_elem(node).ok())
            .collect();
        Ok(Self::new(
            elem_def.id.to_owned(),
            element,
            Some(lsdata),
            contents,
        ))
    }
}

impl<'a> SapTableNormalCell<'a> {
    pub const fn new(
        id: Cow<'static, str>,
        element_ref: scraper::ElementRef<'a>,
        lsdata: Option<SapTableNormalCellLSData>,
        contents: Vec<Elements<'a>>,
    ) -> Self {
        Self {
            id,
            element_ref,
            lsdata,
            contents,
        }
    }

    pub fn wrap(self) -> SapTableCells<'a> {
        SapTableCells::Normal(self)
    }
}
