use std::borrow::Cow;

use serde::Deserialize;

use crate::webdynpro::{
    element::{Element, SubElement, SubElementDef},
    error::{BodyError, ElementError},
};

use super::{SapTableCell, SapTableCells};

pub struct SapTableMatrixCell {
    id: Cow<'static, str>,
    lsdata: Option<SapTableMatrixCellLSData>,
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
    ) -> Result<Self, BodyError> {
        let lsdata_obj = Self::lsdata_elem(element)?;
        let lsdata = serde_json::from_value::<Self::SubElementLSData>(lsdata_obj)
            .or(Err(ElementError::InvalidLSData))?;
        Ok(Self::new(elem_def.id.to_owned(), Some(lsdata)))
    }
}

impl SapTableMatrixCell {
    pub const fn new(id: Cow<'static, str>, lsdata: Option<SapTableMatrixCellLSData>) -> Self {
        Self { id, lsdata }
    }

    pub fn wrap(self) -> SapTableCells {
        SapTableCells::Matrix(self)
    }
}
