use anyhow::Result;
use std::borrow::Cow;

use scraper::Selector;
use serde::Deserialize;

use crate::webdynpro::{
    element::{dyn_elem, Element, Elements, SubElement, SubElementDef},
    error::{BodyError, ElementError},
};

use super::{SapTableCell, SapTableCells};

#[derive(Debug)]
pub struct SapTableMatrixCell {
    id: Cow<'static, str>,
    lsdata: Option<SapTableMatrixCellLSData>,
    contents: Vec<Elements>,
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

impl SapTableCell for SapTableMatrixCell {
    fn content(&self) -> &Vec<Elements> {
        &self.contents
    }
}

impl SubElement for SapTableMatrixCell {
    const SUBCONTROL_ID: &'static str = "MC";
    const ELEMENT_NAME: &'static str = "SapTableMatrixCell";

    type SubElementLSData = SapTableMatrixCellLSData;

    fn lsdata(&self) -> Option<&Self::SubElementLSData> {
        self.lsdata.as_ref()
    }

    fn from_elem<Parent: Element>(
        elem_def: SubElementDef<Parent, Self>,
        element: scraper::ElementRef,
    ) -> Result<Self> {
        let lsdata_obj = Self::lsdata_elem(element)?;
        let lsdata = serde_json::from_value::<Self::SubElementLSData>(lsdata_obj)
            .or(Err(ElementError::InvalidLSData))?;
        let content_selector = Selector::parse(":root > [ct]").unwrap();
        let contents: Vec<Elements> = element
            .select(&content_selector)
            .filter_map(|node| dyn_elem(node).ok())
            .collect();
        Ok(Self::new(elem_def.id.to_owned(), Some(lsdata), contents))
    }
}

impl SapTableMatrixCell {
    pub const fn new(
        id: Cow<'static, str>,
        lsdata: Option<SapTableMatrixCellLSData>,
        contents: Vec<Elements>,
    ) -> Self {
        Self {
            id,
            lsdata,
            contents,
        }
    }

    pub fn wrap(self) -> SapTableCells {
        SapTableCells::Matrix(self)
    }
}
