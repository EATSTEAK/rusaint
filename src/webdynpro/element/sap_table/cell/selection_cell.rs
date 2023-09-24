use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;
use serde::Deserialize;

use crate::webdynpro::element::{Element, Elements, SubElement, SubElementDef};

use super::{SapTableCell, SapTableCells};

#[derive(Debug)]
pub struct SapTableSelectionCell<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<SapTableSelectionCellLSData>>,
    contents: OnceCell<Option<Elements<'a>>>,
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct SapTableSelectionCellLSData {
    #[serde(rename = "0")]
    is_selected: Option<bool>,
    #[serde(rename = "1")]
    is_secondary_selected: Option<bool>,
    #[serde(rename = "2")]
    enabled: Option<bool>,
    #[serde(rename = "3")]
    cell_type: Option<String>,
    #[serde(rename = "4")]
    row_description: Option<String>,
    #[serde(rename = "5")]
    is_deselectable: Option<bool>,
    #[serde(rename = "6")]
    tooltip: Option<String>,
    #[serde(rename = "7")]
    custom_data: Option<String>,
}

impl<'a> SapTableCell<'a> for SapTableSelectionCell<'a> {
    fn content(&self) -> Option<&Elements<'a>> {
        self.contents
            .get_or_init(|| {
                let content_selector = Selector::parse(":root > div > div [ct]").unwrap();
                Elements::dyn_elem(
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

impl<'a> SubElement<'a> for SapTableSelectionCell<'a> {
    const SUBCONTROL_ID: &'static str = "SC";
    const ELEMENT_NAME: &'static str = "SapTableSelectionCell";

    type SubElementLSData = SapTableSelectionCellLSData;

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

impl<'a> SapTableSelectionCell<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            contents: OnceCell::new(),
        }
    }

    pub fn wrap(self) -> SapTableCells<'a> {
        SapTableCells::Selection(self)
    }
}
