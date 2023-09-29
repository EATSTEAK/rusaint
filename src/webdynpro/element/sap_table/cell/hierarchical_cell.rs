use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;
use serde::Deserialize;

use crate::webdynpro::element::{Element, ElementWrapper, SubElement, SubElementDef};

use super::{SapTableCell, SapTableCells};

#[derive(Debug)]
pub struct SapTableHierarchicalCell<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<SapTableHierarchicalCellLSData>>,
    contents: OnceCell<Option<ElementWrapper<'a>>>,
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct SapTableHierarchicalCellLSData {
    #[serde(rename = "0")]
    is_selected: Option<bool>,
    #[serde(rename = "1")]
    is_secondary_selected: Option<bool>,
    #[serde(rename = "2")]
    cell_design: Option<String>,
    #[serde(rename = "3")]
    header_cell_ids: Option<String>,
    #[serde(rename = "4")]
    level: Option<i32>,
    #[serde(rename = "5")]
    status: Option<String>,
    #[serde(rename = "6")]
    status_enabled: Option<bool>,
    #[serde(rename = "7")]
    content_type_tooltip: Option<String>,
    #[serde(rename = "8")]
    custom_style: Option<String>,
    #[serde(rename = "9")]
    custom_data: Option<String>,
}

impl<'a> SapTableCell<'a> for SapTableHierarchicalCell<'a> {
    fn content(&self) -> Option<&ElementWrapper<'a>> {
        self.contents
            .get_or_init(|| {
                let content_selector = Selector::parse(":root [ct]").unwrap();
                ElementWrapper::dyn_elem(
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

impl<'a> SubElement<'a> for SapTableHierarchicalCell<'a> {
    const SUBCONTROL_ID: &'static str = "HIC";
    const ELEMENT_NAME: &'static str = "SapTableHierarchicalCell";

    type SubElementLSData = SapTableHierarchicalCellLSData;

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

impl<'a> SapTableHierarchicalCell<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            contents: OnceCell::new(),
        }
    }

    pub fn wrap(self) -> SapTableCells<'a> {
        SapTableCells::Hierarchical(self)
    }
}
