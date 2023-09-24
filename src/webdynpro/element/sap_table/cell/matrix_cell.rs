use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;
use serde::Deserialize;

use crate::webdynpro::element::{dyn_elem, Element, Elements, SubElement, SubElementDef};

use super::{SapTableCell, SapTableCells};

#[derive(Debug)]
pub struct SapTableMatrixCell<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<SapTableMatrixCellLSData>>,
    contents: OnceCell<Option<Elements<'a>>>,
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

impl<'a> SapTableCell<'a> for SapTableMatrixCell<'a> {
    fn content(&self) -> Option<&Elements<'a>> {
        self.contents
            .get_or_init(|| {
                let content_selector = Selector::parse(":root [ct]").unwrap();
                dyn_elem(
                    self.element_ref
                        .select(&content_selector)
                        .next()?
                        .to_owned(),
                )
                .ok()
            })
            .as_ref()
    }
}

impl<'a> SubElement<'a> for SapTableMatrixCell<'a> {
    const SUBCONTROL_ID: &'static str = "MC";
    const ELEMENT_NAME: &'static str = "SapTableMatrixCell";

    type SubElementLSData = SapTableMatrixCellLSData;

    fn lsdata(&self) -> Option<&Self::SubElementLSData> {
        self.lsdata
            .get_or_init(|| {
                let lsdata_obj = Self::lsdata_elem(self.element_ref).ok()?;
                serde_json::from_value::<Self::SubElementLSData>(lsdata_obj).ok()
            })
            .as_ref()
    }

    fn from_elem<Parent: Element<'a>>(
        elem_def: SubElementDef<'a, Parent, Self>,
        element: scraper::ElementRef<'a>,
    ) -> Result<Self> {
        Ok(Self::new(elem_def.id.to_owned(), element))
    }
}

impl<'a> SapTableMatrixCell<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            contents: OnceCell::new(),
        }
    }

    pub fn wrap(self) -> SapTableCells<'a> {
        SapTableCells::Matrix(self)
    }
}
