use getset::Getters;
use std::{borrow::Cow, cell::OnceCell, ops::Deref};

use scraper::Selector;
use serde::Deserialize;

use crate::webdynpro::{
    element::{
        complex::sap_table::property::{
            SapTableHeaderCellDesign, SapTableHeaderCellType, SapTableRowSelectionMassState,
            SapTableSelectionColumnAction,
        },
        property::SortState,
        Element, ElementWrapper, SubElement, SubElementDef,
    },
    error::{BodyError, WebDynproError},
};

use super::{SapTableCell, SapTableCellWrapper};

/// 테이블의 헤더 셀
#[derive(custom_debug_derive::Debug)]
pub struct SapTableHeaderCell<'a> {
    id: Cow<'static, str>,
    #[debug(skip)]
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<SapTableHeaderCellLSData>,
    content: OnceCell<Option<ElementWrapper<'a>>>,
}

#[derive(Getters, Deserialize, Debug, Default)]
#[allow(unused)]
#[get = "pub"]
pub struct SapTableHeaderCellLSData {
    #[serde(rename = "0")]
    sort_state: Option<SortState>,
    #[serde(rename = "1")]
    header_cell_design: Option<SapTableHeaderCellDesign>,
    #[serde(rename = "2")]
    header_cell_type: Option<SapTableHeaderCellType>,
    #[serde(rename = "3")]
    selection_column_action: Option<SapTableSelectionColumnAction>,
    #[serde(rename = "4")]
    selection_menu_id: Option<String>,
    #[serde(rename = "5")]
    row_selection_mass_state: Option<SapTableRowSelectionMassState>,
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

    fn lsdata(&self) -> &Self::SubElementLSData {
        self.lsdata
            .get_or_init(|| {
                let Ok(lsdata_obj) = Self::lsdata_elem(self.element_ref) else {
                    return Self::SubElementLSData::default();
                };
                serde_json::from_value::<Self::SubElementLSData>(lsdata_obj)
                    .unwrap_or(Self::SubElementLSData::default())
            })
    }

    fn from_elem<Parent: Element<'a>>(
        elem_def: SubElementDef<'a, Parent, Self>,
        element: scraper::ElementRef<'a>,
    ) -> Result<Self, WebDynproError> {
        Ok(Self::new(elem_def.id.to_owned(), element))
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn element_ref(&self) -> &scraper::ElementRef<'a> {
        &self.element_ref
    }
}

impl<'a> SapTableCell<'a> for SapTableHeaderCell<'a> {
    fn content(&self) -> Option<&ElementWrapper<'a>> {
        self.content
            .get_or_init(|| {
                let content_selector =
                    Selector::parse(format!(r#"[id="{}-CONTENT"] [ct]"#, &self.id).as_str())
                        .or(Err(BodyError::InvalidSelector))
                        .ok()?;
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

impl<'a> SapTableHeaderCell<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            content: OnceCell::new(),
        }
    }

    /// 셀을 [`SapTableCellWrapper`]로 감쌉니다.
    pub fn wrap(self) -> SapTableCellWrapper<'a> {
        SapTableCellWrapper::Header(self)
    }
}

impl<'a> Deref for SapTableHeaderCell<'a> {
    type Target = SapTableHeaderCellLSData;

    fn deref(&self) -> &Self::Target {
        self.lsdata()
    }
}