use std::borrow::Cow;

use scraper::Selector;
use serde::Deserialize;

use crate::webdynpro::{
    element::{dyn_elem, Element, Elements, SubElement, SubElementDef},
    error::{BodyError, ElementError},
};

use super::{SapTableCell, SapTableCells};

pub struct SapTableNormalCell {
    id: Cow<'static, str>,
    lsdata: Option<SapTableNormalCellLSData>,
    contents: Vec<Elements>,
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

impl SapTableCell for SapTableNormalCell {
    fn content(&self) -> &Vec<Elements> {
        &self.contents
    }
}

impl SubElement for SapTableNormalCell {
    const SUBCONTROL_ID: &'static str = "STC";
    const ELEMENT_NAME: &'static str = "SapTableNormalCell";

    type SubElementLSData = SapTableNormalCellLSData;

    fn lsdata(&self) -> Option<&Self::SubElementLSData> {
        self.lsdata.as_ref()
    }

    fn from_elem<Parent: Element>(
        elem_def: SubElementDef<Parent, Self>,
        element: scraper::ElementRef,
    ) -> Result<Self, BodyError> {
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

impl SapTableNormalCell {
    pub const fn new(
        id: Cow<'static, str>,
        lsdata: Option<SapTableNormalCellLSData>,
        contents: Vec<Elements>,
    ) -> Self {
        Self {
            id,
            lsdata,
            contents,
        }
    }

    pub fn wrap(self) -> SapTableCells {
        SapTableCells::Normal(self)
    }
}
