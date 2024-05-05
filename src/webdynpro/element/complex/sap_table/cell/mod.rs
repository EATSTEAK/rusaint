use crate::webdynpro::client::body::Body;
use crate::webdynpro::element::definition::sub::SubElementDefinition;
use crate::webdynpro::element::sub::SubElement;
use crate::webdynpro::element::{Element, ElementDefWrapper};
use crate::webdynpro::error::WebDynproError;

/// [`SapTable`] 셀의 Wrapper
#[derive(Debug)]
pub enum SapTableCellWrapper<'a> {
    /// 일반적인 셀
    Normal(SapTableNormalCell<'a>),
    /// 헤더 셀
    Header(SapTableHeaderCell<'a>),
    /// 순차형 셀
    Hierarchical(SapTableHierarchicalCell<'a>),
    /// Matrix 레이아웃용 셀
    Matrix(SapTableMatrixCell<'a>),
    /// 선택을 위한 셀
    Selection(SapTableSelectionCell<'a>),
}

impl<'a> SapTableCellWrapper<'a> {
    /// 셀을 표현하는 HTML 엘리먼트로부터 [`SapTableCellWrapper`]를 생성합니다.
    pub fn dyn_cell(
        table_def: <SapTable<'a> as Element<'a>>::Def,
        elem_ref: scraper::ElementRef<'a>,
    ) -> Option<SapTableCellWrapper<'a>> {
        let subct_value = elem_ref.value();
        match subct_value.attr("subct") {
            Some(SapTableNormalCell::SUBCONTROL_ID) => Some(
                <SapTableNormalCell<'_> as SubElement>::Def::new_dynamic(
                    table_def,
                    subct_value.id()?.to_owned(),
                )
                .from_element(elem_ref)
                .ok()?
                .wrap(),
            ),
            Some(SapTableHeaderCell::SUBCONTROL_ID) => Some(
                <SapTableHeaderCell<'_> as SubElement>::Def::new_dynamic(
                    table_def,
                    subct_value.id()?.to_owned(),
                )
                .from_element(elem_ref)
                .ok()?
                .wrap(),
            ),
            Some(SapTableHierarchicalCell::SUBCONTROL_ID) => Some(
                <SapTableHierarchicalCell<'_> as SubElement>::Def::new_dynamic(
                    table_def,
                    subct_value.id()?.to_owned(),
                )
                .from_element(elem_ref)
                .ok()?
                .wrap(),
            ),
            Some(SapTableMatrixCell::SUBCONTROL_ID) => Some(
                <SapTableMatrixCell<'_> as SubElement>::Def::new_dynamic(
                    table_def,
                    subct_value.id()?.to_owned(),
                )
                .from_element(elem_ref)
                .ok()?
                .wrap(),
            ),
            Some(SapTableSelectionCell::SUBCONTROL_ID) => Some(
                <SapTableSelectionCell<'_> as SubElement>::Def::new_dynamic(
                    table_def,
                    subct_value.id()?.to_owned(),
                )
                .from_element(elem_ref)
                .ok()?
                .wrap(),
            ),
            _ => None,
        }
    }
}

/// [`SapTable`] 셀에 대한 [`SubElementDef`] Wrapper
#[derive(Clone, Debug)]
pub enum SapTableCellDefWrapper {
    /// 일반 셀 정의
    Normal(SapTableNormalCellDef),
    /// 헤더 셀 정의
    Header(SapTableHeaderCellDef),
    /// 순차형 셀 정의
    Hierarchical(SapTableHierarchicalCellDef),
    /// Matrix 레이아웃용 셀 정의
    Matrix(SapTableMatrixCellDef),
    /// 선택을 위한 셀 정의
    Selection(SapTableSelectionCellDef),
}

impl SapTableCellDefWrapper {
    // TODO: include node id in def to improve performance
    /// 셀을 표현하는 HTML 엘리먼트로부터 [`SapTableCellDefWrapper`]를 생성합니다.
    pub fn dyn_cell_def(
        table_def: SapTableDef,
        elem_ref: scraper::ElementRef<'_>,
    ) -> Option<SapTableCellDefWrapper> {
        let subct_value = elem_ref.value();
        match subct_value.attr("subct") {
            Some(SapTableNormalCell::SUBCONTROL_ID) => Some(SapTableCellDefWrapper::Normal(
                SapTableNormalCellDef::new_dynamic(table_def, subct_value.id()?.to_owned()),
            )),
            Some(SapTableHeaderCell::SUBCONTROL_ID) => Some(SapTableCellDefWrapper::Header(
                SapTableHeaderCellDef::new_dynamic(table_def, subct_value.id()?.to_owned()),
            )),
            Some(SapTableHierarchicalCell::SUBCONTROL_ID) => Some(
                SapTableCellDefWrapper::Hierarchical(SapTableHierarchicalCellDef::new_dynamic(
                    table_def,
                    subct_value.id()?.to_owned(),
                )),
            ),
            Some(SapTableMatrixCell::SUBCONTROL_ID) => Some(SapTableCellDefWrapper::Matrix(
                SapTableMatrixCellDef::new_dynamic(table_def, subct_value.id()?.to_owned()),
            )),
            Some(SapTableSelectionCell::SUBCONTROL_ID) => Some(SapTableCellDefWrapper::Selection(
                SapTableSelectionCellDef::new_dynamic(table_def, subct_value.id()?.to_owned()),
            )),
            _ => None,
        }
    }

    /// [`Body`]에서 서브 엘리먼트를 가져옵니다.
    pub fn from_body(self, body: &Body) -> Result<SapTableCellWrapper, WebDynproError> {
        match self {
            Self::Normal(def) => Ok(def.from_body(body)?.wrap()),
            Self::Header(def) => Ok(def.from_body(body)?.wrap()),
            Self::Hierarchical(def) => Ok(def.from_body(body)?.wrap()),
            Self::Matrix(def) => Ok(def.from_body(body)?.wrap()),
            Self::Selection(def) => Ok(def.from_body(body)?.wrap()),
        }
    }

    /// [`scraper::ElementRef`]에서 서브 엘리먼트를 가져옵니다.
    pub fn from_element(
        self,
        element: scraper::ElementRef<'_>,
    ) -> Result<SapTableCellWrapper, WebDynproError> {
        match self {
            Self::Normal(def) => Ok(def.from_element(element)?.wrap()),
            Self::Header(def) => Ok(def.from_element(element)?.wrap()),
            Self::Hierarchical(def) => Ok(def.from_element(element)?.wrap()),
            Self::Matrix(def) => Ok(def.from_element(element)?.wrap()),
            Self::Selection(def) => Ok(def.from_element(element)?.wrap()),
        }
    }
}

/// [`SapTable`]의 공통된 셀 기능
pub trait SapTableCell<'a> {
    /// 셀 내부 컨텐츠 엘리먼트를 반환합니다.
    fn content(&self) -> Option<ElementDefWrapper<'a>>;
}

impl<'a> SapTableCell<'a> for SapTableCellWrapper<'a> {
    fn content(&self) -> Option<ElementDefWrapper<'a>> {
        match self {
            SapTableCellWrapper::Normal(elem) => elem.content(),
            SapTableCellWrapper::Header(elem) => elem.content(),
            SapTableCellWrapper::Hierarchical(elem) => elem.content(),
            SapTableCellWrapper::Matrix(elem) => elem.content(),
            SapTableCellWrapper::Selection(elem) => elem.content(),
        }
    }
}

mod header_cell;
mod hierarchical_cell;
mod matrix_cell;
mod normal_cell;
mod selection_cell;

pub use self::header_cell::{SapTableHeaderCell, SapTableHeaderCellDef, SapTableHeaderCellLSData};
pub use self::hierarchical_cell::{
    SapTableHierarchicalCell, SapTableHierarchicalCellDef, SapTableHierarchicalCellLSData,
};
pub use self::matrix_cell::{SapTableMatrixCell, SapTableMatrixCellDef, SapTableMatrixCellLSData};
pub use self::normal_cell::{SapTableNormalCell, SapTableNormalCellDef, SapTableNormalCellLSData};
pub use self::selection_cell::{
    SapTableSelectionCell, SapTableSelectionCellDef, SapTableSelectionCellLSData,
};

use super::{SapTable, SapTableDef};
