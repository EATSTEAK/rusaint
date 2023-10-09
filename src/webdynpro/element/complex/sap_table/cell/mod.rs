use crate::webdynpro::element::ElementWrapper;

#[derive(Debug)]
pub enum SapTableCellWrapper<'a> {
    Normal(SapTableNormalCell<'a>),
    Header(SapTableHeaderCell<'a>),
    Hierarchical(SapTableHierarchicalCell<'a>),
    Matrix(SapTableMatrixCell<'a>),
    Selection(SapTableSelectionCell<'a>),
}

impl<'a> SapTableCellWrapper<'a> {
    pub fn content(&self) -> Option<&ElementWrapper<'a>> {
        match self {
            SapTableCellWrapper::Normal(elem) => elem.content(),
            SapTableCellWrapper::Header(elem) => elem.content(),
            SapTableCellWrapper::Hierarchical(elem) => elem.content(),
            SapTableCellWrapper::Matrix(elem) => elem.content(),
            SapTableCellWrapper::Selection(elem) => elem.content(),
        }
    }
}

pub trait SapTableCell<'a> {
    /// 셀 내부 컨텐츠 엘리먼트를 반환합니다.
    fn content(&self) -> Option<&ElementWrapper<'a>>;
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
