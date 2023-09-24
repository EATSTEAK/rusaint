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
pub struct SapTableHeaderCell<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: Option<SapTableHeaderCellLSData>,
    contents: Vec<Elements<'a>>,
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct SapTableHeaderCellLSData {
    #[serde(rename = "0")]
    sort_state: Option<String>,
    #[serde(rename = "1")]
    header_cell_design: Option<String>,
    #[serde(rename = "2")]
    header_cell_type: Option<String>,
    #[serde(rename = "3")]
    selection_column_action: Option<String>,
    #[serde(rename = "4")]
    selection_menu_id: Option<String>,
    #[serde(rename = "5")]
    row_selection_mass_state: Option<String>,
    #[serde(rename = "6")]
    required: Option<bool>,
    #[serde(rename = "7")]
    tooltip: Option<String>,
    #[serde(rename = "8")]
    column_selected: Option<bool>,
    #[serde(rename = "9")]
    column_selectable: Option<bool>,
    #[serde(rename = "10")]
    filtered: Option<bool>,
    #[serde(rename = "11")]
    mark_totals: Option<bool>,
    #[serde(rename = "12")]
    accessibility_description: Option<String>,
    #[serde(rename = "13")]
    icon_tooltip: Option<String>,
    #[serde(rename = "14")]
    icon_first: Option<bool>,
    #[serde(rename = "15")]
    icon_enabled: Option<bool>,
    #[serde(rename = "16")]
    custom_style: Option<String>,
    #[serde(rename = "17")]
    custom_data: Option<String>,
}

impl<'a> SubElement<'a> for SapTableHeaderCell<'a> {
    const SUBCONTROL_ID: &'static str = "HC";
    const ELEMENT_NAME: &'static str = "SapTableHeaderCell";

    type SubElementLSData = SapTableHeaderCellLSData;

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
        let content_selector =
            Selector::parse(format!(r#"[id="{}-CONTENT"] > [ct]"#, &elem_def.id).as_str())
                .or(Err(BodyError::InvalidSelector))?;
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

impl<'a> SapTableCell for SapTableHeaderCell<'a> {
    fn content(&self) -> &Vec<Elements<'a>> {
        &self.contents
    }
}

impl<'a> SapTableHeaderCell<'a> {
    pub const fn new(
        id: Cow<'static, str>,
        element_ref: scraper::ElementRef<'a>,
        lsdata: Option<SapTableHeaderCellLSData>,
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
        SapTableCells::Header(self)
    }
}
