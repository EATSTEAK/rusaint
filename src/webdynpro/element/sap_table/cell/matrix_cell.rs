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
pub struct SapTableMatrixCell<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: Option<SapTableMatrixCellLSData>,
    contents: Vec<Elements<'a>>,
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct SapTableMatrixCellLSData {
    #[serde(rename = "0")]
    cell_background_design: Option<String>,
    #[serde(rename = "1")]
    header_cell_ids: Option<String>,
    #[serde(rename = "2")]
    row_header_cell_ids: Option<String>,
    #[serde(rename = "3")]
    custom_data: Option<String>,
}

impl<'a> SapTableCell for SapTableMatrixCell<'a> {
    fn content(&self) -> &Vec<Elements> {
        &self.contents
    }
}

impl<'a> SubElement<'a> for SapTableMatrixCell<'a> {
    const SUBCONTROL_ID: &'static str = "MC";
    const ELEMENT_NAME: &'static str = "SapTableMatrixCell";

    type SubElementLSData = SapTableMatrixCellLSData;

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

impl<'a> SapTableMatrixCell<'a> {
    pub const fn new(
        id: Cow<'static, str>,
        element_ref: scraper::ElementRef<'a>,
        lsdata: Option<SapTableMatrixCellLSData>,
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
        SapTableCells::Matrix(self)
    }
}
