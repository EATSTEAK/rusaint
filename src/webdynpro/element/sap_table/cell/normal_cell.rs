use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;
use serde::Deserialize;

use crate::webdynpro::element::{dyn_elem, Element, Elements, SubElement, SubElementDef};

use super::{SapTableCell, SapTableCells};

#[derive(Debug)]
pub struct SapTableNormalCell<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<SapTableNormalCellLSData>>,
    contents: OnceCell<Option<Elements<'a>>>,
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

impl<'a> SubElement<'a> for SapTableNormalCell<'a> {
    const SUBCONTROL_ID: &'static str = "STC";
    const ELEMENT_NAME: &'static str = "SapTableNormalCell";

    type SubElementLSData = SapTableNormalCellLSData;

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

    fn id(&self) -> &str {
        &self.id
    }

    fn element_ref(&self) -> &scraper::ElementRef<'a> {
        &self.element_ref
    }
}

impl<'a> SapTableNormalCell<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            contents: OnceCell::new(),
        }
    }

    pub fn wrap(self) -> SapTableCells<'a> {
        SapTableCells::Normal(self)
    }
}
