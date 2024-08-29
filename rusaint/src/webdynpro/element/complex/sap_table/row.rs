use std::ops::Index;

use super::{
    cell::{SapTableCellDefWrapper, SapTableCellWrapper},
    property::{SapTableRowType, SapTableSelectionState},
    FromSapTable, SapTableDef, SapTableHeader,
};
use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::error::{ElementError, WebDynproError};
use tl::Bytes;

/// [`SapTable`](super::SapTable)의 행
#[derive(Clone, Debug)]
#[allow(unused)]
pub struct SapTableRow {
    table_def: SapTableDef,
    cells: Vec<SapTableCellDefWrapper>,
    row_index: Option<u32>,
    user_data: Option<String>,
    drag_data: Option<String>,
    drop_target_info: Option<String>,
    parent_drop_target_info: Option<String>,
    selection_state: SapTableSelectionState,
    row_type: SapTableRowType,
}

impl<'a> SapTableRow {
    pub(super) fn new(
        table_def: SapTableDef,
        row_tag: tl::HTMLTag<'a>,
        cells: Vec<SapTableCellDefWrapper>,
    ) -> Result<SapTableRow, ElementError> {
        Ok(SapTableRow {
            table_def,
            cells,
            row_index: row_tag
                .attributes()
                .get("rr")
                .flatten()
                .and_then(Bytes::try_as_utf8_str)
                .and_then(|s| s.parse::<u32>().ok()),
            user_data: row_tag
                .attributes()
                .get("uDat")
                .flatten()
                .and_then(Bytes::try_as_utf8_str)
                .and_then(|s| Some(s.to_owned())),
            drag_data: row_tag
                .attributes()
                .get("ddData")
                .flatten()
                .and_then(Bytes::try_as_utf8_str)
                .and_then(|s| Some(s.to_owned())),
            drop_target_info: row_tag
                .attributes()
                .get("ddDti")
                .flatten()
                .and_then(Bytes::try_as_utf8_str)
                .and_then(|s| Some(s.to_owned())),
            parent_drop_target_info: row_tag
                .attributes()
                .get("ddPDti")
                .flatten()
                .and_then(Bytes::try_as_utf8_str)
                .and_then(|s| Some(s.to_owned())),
            selection_state: row_tag
                .attributes()
                .get("sst")
                .flatten()
                .and_then(Bytes::try_as_utf8_str)
                .and_then(|s| Some(s.into()))
                .unwrap_or(SapTableSelectionState::default()),
            row_type: row_tag
                .attributes()
                .get("rt")
                .flatten()
                .and_then(Bytes::try_as_utf8_str)
                .and_then(|s| Some(s.into()))
                .unwrap_or(SapTableRowType::default()),
        })
    }

    /// 행 내부 셀의 개수를 반환합니다.
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// 행 내부 셀 정의의 [`Iterator`]를 반환합니다.
    pub fn iter(&self) -> impl Iterator<Item = &SapTableCellDefWrapper> + ExactSizeIterator {
        self.cells.iter()
    }

    /// 행 내부 셀 엘리먼트의 [`Iterator`]를 반환합니다.
    pub fn iter_value(
        &'a self,
        parser: &'a ElementParser,
    ) -> impl Iterator<Item = Result<SapTableCellWrapper<'a>, WebDynproError>> + ExactSizeIterator
    {
        self.cells
            .iter()
            .map(|def| SapTableCellWrapper::from_def(def, parser))
    }

    /// 원본 [`SapTable`](super::SapTable)의 [`SapTableDef`]를 반환합니다.
    pub fn table_def(&self) -> SapTableDef {
        self.table_def.clone()
    }

    /// 인덱스를 반환합니다.
    pub fn row_index(&self) -> Option<u32> {
        self.row_index
    }

    /// 유저 데이터를 반환합니다.
    pub fn user_data(&self) -> Option<&str> {
        self.user_data.as_ref().map(|x| x.as_str())
    }

    /// 드레그 데이터를 반환합니다.
    pub fn drag_data(&self) -> Option<&str> {
        self.drag_data.as_ref().map(|x| x.as_str())
    }

    /// 드롭 타겟 정보를 반환합니다.
    pub fn drop_target_info(&self) -> Option<&str> {
        self.drop_target_info.as_ref().map(|x| x.as_str())
    }

    /// 부모의 드롭 타겟 정보를 반환합니다.
    pub fn parent_drop_target_info(&self) -> Option<&str> {
        self.parent_drop_target_info.as_ref().map(|x| x.as_str())
    }

    /// 선택 상태를 반환합니다.
    pub fn selection_state(&self) -> SapTableSelectionState {
        self.selection_state
    }

    /// 행 종류를 반환합니다.
    pub fn row_type(&self) -> SapTableRowType {
        self.row_type
    }

    /// 행을 [`FromSapTable`]을 구현하는 형으로 변환합니다.
    pub fn try_row_into<T: FromSapTable<'a>>(
        &'a self,
        header: &'a SapTableHeader,
        parser: &'a ElementParser,
    ) -> Result<T, WebDynproError> {
        T::from_table(header, self, parser)
    }
}

impl<'a> Index<usize> for SapTableRow {
    type Output = SapTableCellDefWrapper;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl From<&str> for SapTableSelectionState {
    fn from(s: &str) -> Self {
        match s {
            "4" => Self::NotSelectable,
            "0" => Self::NotSelected,
            "2" => Self::Selected,
            "1" => Self::PrimarySelected,
            _ => Self::None,
        }
    }
}

impl From<&str> for SapTableRowType {
    fn from(s: &str) -> Self {
        match s {
            "1" => Self::Standard,
            "2" => Self::Header,
            "3" => Self::Filter,
            "4" => Self::TopFixed,
            "5" => Self::BottomFixed,
            "6" => Self::Pivot,
            _ => Self::Unspecified,
        }
    }
}
