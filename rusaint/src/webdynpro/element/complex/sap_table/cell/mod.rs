use crate::webdynpro::element::{ElementDef, ElementWrapper, SubElement, SubElementDef};

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
        table_def: ElementDef<'a, SapTable<'a>>,
        elem_ref: scraper::ElementRef<'a>,
    ) -> Option<SapTableCellWrapper<'a>> {
        let subct_value = elem_ref.value();
        match subct_value.attr("subct") {
            Some(SapTableNormalCell::SUBCONTROL_ID) => Some(
                SubElementDef::<_, SapTableNormalCell>::new_dynamic(
                    table_def,
                    subct_value.id()?.to_owned(),
                )
                .from_elem(elem_ref)
                .ok()?
                .wrap(),
            ),
            Some(SapTableHeaderCell::SUBCONTROL_ID) => Some(
                SubElementDef::<_, SapTableHeaderCell>::new_dynamic(
                    table_def,
                    subct_value.id()?.to_owned(),
                )
                .from_elem(elem_ref)
                .ok()?
                .wrap(),
            ),
            Some(SapTableHierarchicalCell::SUBCONTROL_ID) => Some(
                SubElementDef::<_, SapTableHierarchicalCell>::new_dynamic(
                    table_def,
                    subct_value.id()?.to_owned(),
                )
                .from_elem(elem_ref)
                .ok()?
                .wrap(),
            ),
            Some(SapTableMatrixCell::SUBCONTROL_ID) => Some(
                SubElementDef::<_, SapTableMatrixCell>::new_dynamic(
                    table_def,
                    subct_value.id()?.to_owned(),
                )
                .from_elem(elem_ref)
                .ok()?
                .wrap(),
            ),
            Some(SapTableSelectionCell::SUBCONTROL_ID) => Some(
                SubElementDef::<_, SapTableSelectionCell>::new_dynamic(
                    table_def,
                    subct_value.id()?.to_owned(),
                )
                .from_elem(elem_ref)
                .ok()?
                .wrap(),
            ),
            _ => None,
        }
    }
}

/// [`SapTable`]의 공통된 셀 기능
pub trait SapTableCell<'a> {
    /// 셀 내부 컨텐츠 엘리먼트를 반환합니다.
    fn content(&self) -> Option<&ElementWrapper<'a>>;
}

impl<'a> SapTableCell<'a> for SapTableCellWrapper<'a> {
    fn content(&self) -> Option<&ElementWrapper<'a>> {
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

pub use self::header_cell::{SapTableHeaderCell, SapTableHeaderCellLSData};
pub use self::hierarchical_cell::{SapTableHierarchicalCell, SapTableHierarchicalCellLSData};
pub use self::matrix_cell::{SapTableMatrixCell, SapTableMatrixCellLSData};
pub use self::normal_cell::{SapTableNormalCell, SapTableNormalCellLSData};
pub use self::selection_cell::{SapTableSelectionCell, SapTableSelectionCellLSData};

use super::SapTable;
