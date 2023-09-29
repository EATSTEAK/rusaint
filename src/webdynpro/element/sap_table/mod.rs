use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;
use serde::Deserialize;

use crate::webdynpro::{
    element::SubElement,
    error::{BodyError, ElementError},
};

use self::cell::{
    header_cell::SapTableHeaderCell, hierarchical_cell::SapTableHierarchicalCell,
    matrix_cell::SapTableMatrixCell, normal_cell::SapTableNormalCell,
    selection_cell::SapTableSelectionCell, SapTableCells,
};

use super::{Element, ElementDef, EventParameterMap, SubElementDef};

pub type SapTableBody<'a> = Vec<Vec<SapTableCells<'a>>>;

#[derive(Debug)]
pub struct SapTable<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<SapTableLSData>>,
    lsevents: OnceCell<Option<EventParameterMap>>,
    table: OnceCell<Option<SapTableBody<'a>>>,
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct SapTableLSData {
    #[serde(rename = "0")]
    title_text: Option<String>,
    #[serde(rename = "1")]
    accessibility_description: Option<String>,
    #[serde(rename = "2")]
    row_count: Option<u32>,
    #[serde(rename = "3")]
    col_count: Option<u32>,
}

impl<'a> Element<'a> for SapTable<'a> {
    const CONTROL_ID: &'static str = "ST";

    const ELEMENT_NAME: &'static str = "SapTable";

    type ElementLSData = SapTableLSData;

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

    fn wrap(self) -> super::Elements<'a> {
        super::Elements::SapTable(self)
    }
}

impl<'a> SapTable<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
            table: OnceCell::new(),
        }
    }

    pub fn table(&self) -> Option<&SapTableBody<'a>> {
        self.table.get_or_init(|| self.parse_table().ok()).as_ref()
    }

    fn parse_table(&self) -> Result<SapTableBody<'a>> {
        let def: ElementDef<'a, SapTable<'a>> = {
            if let Cow::Borrowed(id) = self.id {
                ElementDef::new(id)
            } else {
                ElementDef::new_dynamic((&self.id).to_string())
            }
        };
        let element = self.element_ref;
        let elem_value = element.value();
        let tbody_selector = Selector::parse(
            format!(
                r#"[id="{}-contentTBody"]"#,
                elem_value
                    .id()
                    .ok_or(ElementError::NoSuchAttribute("id".to_string()))?
            )
            .as_str(),
        )
        .or(Err(BodyError::InvalidSelector))?;
        let tbody = element
            .select(&tbody_selector)
            .next()
            .ok_or(ElementError::NoSuchElement)?;
        Ok(tbody
            .children()
            .filter_map(|node| scraper::ElementRef::wrap(node))
            .map(|row_ref| -> Vec<SapTableCells<'a>> {
                let subct_selector = Selector::parse("[subct]").unwrap();
                let subcts = row_ref.select(&subct_selector);
                subcts
                    .filter_map(|subct_ref| -> Option<SapTableCells<'a>> {
                        let subct_value = subct_ref.value();
                        match subct_value.attr("subct") {
                            Some(SapTableNormalCell::SUBCONTROL_ID) => Some(
                                SubElementDef::<_, SapTableNormalCell>::new_dynamic(
                                    def.clone(),
                                    subct_value.id()?.to_owned(),
                                )
                                .from_elem(subct_ref)
                                .ok()?
                                .wrap(),
                            ),
                            Some(SapTableHeaderCell::SUBCONTROL_ID) => Some(
                                SubElementDef::<_, SapTableHeaderCell>::new_dynamic(
                                    def.clone(),
                                    subct_value.id()?.to_owned(),
                                )
                                .from_elem(subct_ref)
                                .ok()?
                                .wrap(),
                            ),
                            Some(SapTableHierarchicalCell::SUBCONTROL_ID) => Some(
                                SubElementDef::<_, SapTableHierarchicalCell>::new_dynamic(
                                    def.clone(),
                                    subct_value.id()?.to_owned(),
                                )
                                .from_elem(subct_ref)
                                .ok()?
                                .wrap(),
                            ),
                            Some(SapTableMatrixCell::SUBCONTROL_ID) => Some(
                                SubElementDef::<_, SapTableMatrixCell>::new_dynamic(
                                    def.clone(),
                                    subct_value.id()?.to_owned(),
                                )
                                .from_elem(subct_ref)
                                .ok()?
                                .wrap(),
                            ),
                            Some(SapTableSelectionCell::SUBCONTROL_ID) => Some(
                                SubElementDef::<_, SapTableSelectionCell>::new_dynamic(
                                    def.clone(),
                                    subct_value.id()?.to_owned(),
                                )
                                .from_elem(subct_ref)
                                .ok()?
                                .wrap(),
                            ),
                            _ => None,
                        }
                    })
                    .collect::<Vec<SapTableCells<'a>>>()
            })
            .collect())
    }
}

pub mod cell;
