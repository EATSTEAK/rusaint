use std::ops::Index;

use scraper::ElementRef;

use crate::webdynpro::{element::ElementDef, error::ElementError};

use super::{
    cell::SapTableCellWrapper,
    property::{SapTableRowType, SapTableSelectionState},
    SapTable,
};

/// [`SapTable`]의 행
#[derive(custom_debug_derive::Debug)]
#[allow(unused)]
pub struct SapTableRow<'a> {
    table_def: ElementDef<'a, SapTable<'a>>,
    #[debug(skip)]
    elem_ref: ElementRef<'a>,
    cells: Vec<SapTableCellWrapper<'a>>,
    row_index: Option<u32>,
    user_data: Option<String>,
    drag_data: Option<String>,
    drop_target_info: Option<String>,
    parent_drop_target_info: Option<String>,
    selection_state: SapTableSelectionState,
    row_type: SapTableRowType,
}

impl<'a> SapTableRow<'a> {
    pub(super) fn new(
        table_def: ElementDef<'a, SapTable<'a>>,
        row_ref: ElementRef<'a>,
    ) -> Result<SapTableRow<'a>, ElementError> {
        let row = row_ref.value();
        let subct_selector = scraper::Selector::parse("[subct]").unwrap();
        let subcts = row_ref.select(&subct_selector);
        let cells = subcts
            .filter_map(|subct_ref| SapTableCellWrapper::dyn_cell(table_def.clone(), subct_ref))
            .collect::<Vec<SapTableCellWrapper<'a>>>();
        Ok(SapTableRow {
            table_def,
            elem_ref: row_ref,
            cells,
            row_index: row.attr("rr").and_then(|s| s.parse::<u32>().ok()),
            user_data: row.attr("uDat").and_then(|s| Some(s.to_owned())),
            drag_data: row.attr("ddData").and_then(|s| Some(s.to_owned())),
            drop_target_info: row.attr("ddDti").and_then(|s| Some(s.to_owned())),
            parent_drop_target_info: row.attr("ddPDti").and_then(|s| Some(s.to_owned())),
            selection_state: row
                .attr("sst")
                .and_then(|s| Some(s.into()))
                .unwrap_or(SapTableSelectionState::default()),
            row_type: row
                .attr("rt")
                .and_then(|s| Some(s.into()))
                .unwrap_or(SapTableRowType::default()),
        })
    }

    /// 행 내부 셀의 개수를 반환합니다.
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// 행 내부 셀의 [`Iterator`]를 반환합니다.
    pub fn iter(&self) -> impl Iterator<Item = &SapTableCellWrapper<'a>> + ExactSizeIterator {
        self.cells.iter()
    }

    /// 원본 [`SapTable`]의 [`ElementDef`]를 반환합니다.
    pub fn table_def(&self) -> ElementDef<'a, SapTable<'a>> {
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
}

impl<'a> Index<usize> for SapTableRow<'a> {
    type Output = SapTableCellWrapper<'a>;

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
