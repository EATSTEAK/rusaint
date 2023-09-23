use crate::webdynpro::element::Elements;

use self::{
    header_cell::SapTableHeaderCell, hierarchical_cell::SapTableHierarchicalCell,
    matrix_cell::SapTableMatrixCell, normal_cell::SapTableNormalCell,
    selection_cell::SapTableSelectionCell,
};

pub enum SapTableCells {
    Normal(SapTableNormalCell),
    Header(SapTableHeaderCell),
    Hierarchical(SapTableHierarchicalCell),
    Matrix(SapTableMatrixCell),
    Selection(SapTableSelectionCell),
}

pub trait SapTableCell {
    fn content(&self) -> Elements;
}

pub mod header_cell;
pub mod hierarchical_cell;
pub mod matrix_cell;
pub mod normal_cell;
pub mod selection_cell;
