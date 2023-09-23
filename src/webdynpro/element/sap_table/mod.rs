use anyhow::Result;
use std::borrow::Cow;

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

pub type SapTableBody = Vec<Vec<SapTableCells>>;

#[derive(Debug)]
pub struct SapTable {
    id: Cow<'static, str>,
    lsdata: Option<SapTableLSData>,
    lsevents: Option<EventParameterMap>,
    table: Option<SapTableBody>,
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

impl ElementDef<SapTable> {
    pub fn wrap(self) -> super::Elements {
        super::Elements::SapTable(self)
    }
}

impl Element for SapTable {
    const CONTROL_ID: &'static str = "ST";

    const ELEMENT_NAME: &'static str = "SapTable";

    type ElementLSData = SapTableLSData;

    fn lsdata(&self) -> Option<&Self::ElementLSData> {
        self.lsdata.as_ref()
    }

    fn lsevents(&self) -> Option<&EventParameterMap> {
        self.lsevents.as_ref()
    }

    fn from_elem(elem_def: ElementDef<Self>, element: scraper::ElementRef) -> Result<Self> {
        let lsdata_obj = Self::lsdata_elem(element)?;
        let lsdata = serde_json::from_value::<Self::ElementLSData>(lsdata_obj)
            .or(Err(ElementError::InvalidLSData))?;
        let lsevents = Self::lsevents_elem(element)?;
        let table = Self::parse_table(elem_def.clone(), element)?;
        Ok(Self::new(
            elem_def.id.to_owned(),
            Some(lsdata),
            Some(lsevents),
            Some(table),
        ))
    }
}

impl SapTable {
    pub const fn new(
        id: Cow<'static, str>,
        lsdata: Option<SapTableLSData>,
        lsevents: Option<EventParameterMap>,
        table: Option<SapTableBody>,
    ) -> Self {
        Self {
            id,
            lsdata,
            lsevents,
            table,
        }
    }

    pub fn table(&self) -> Option<&SapTableBody> {
        self.table.as_ref()
    }

    fn parse_table(
        def: ElementDef<SapTable>,
        element: scraper::ElementRef,
    ) -> Result<SapTableBody> {
        let elem_value = element.value();
        dbg!("reading tbody");
        let tbody_selector = Selector::parse(
            format!(
                r#"[id="{}-contentTBody"]"#,
                elem_value.id().ok_or(ElementError::InvalidId)?
            )
            .as_str(),
        )
        .or(Err(BodyError::InvalidSelector))?;
        let tbody = element
            .select(&tbody_selector)
            .next()
            .ok_or(ElementError::InvalidId)?;
        dbg!("tbody readed");
        Ok(tbody
            .children()
            .filter_map(|node| scraper::ElementRef::wrap(node))
            .map(|row_ref| -> Vec<SapTableCells> {
                let subct_selector = Selector::parse("[subct]").unwrap();
                let subcts = row_ref.select(&subct_selector);
                subcts
                    .filter_map(|subct_ref| -> Option<SapTableCells> {
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
                    .collect::<Vec<SapTableCells>>()
            })
            .collect())
    }
}

pub mod cell;
