use std::ops::Index;

use scraper::ElementRef;

use super::{
    SapTableDef,
    cell::{SapTableCell, SapTableCellDefWrapper, SapTableCellWrapper},
    property::{SapTableRowType, SapTableSelectionState},
};
use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::{
    element::{Element, ElementDefWrapper, definition::ElementDefinition},
    error::{ElementError, WebDynproError},
};

/// [`SapTable`](super::SapTable)의 행
#[derive(Clone, Debug)]
#[allow(unused)]
pub struct SapTableHeader {
    table_def: SapTableDef,
    cells: Vec<SapTableCellDefWrapper>,
    row_index: Option<u32>,
    user_data: Option<String>,
    drag_data: Option<String>,
    drop_target_info: Option<String>,
    parent_drop_target_info: Option<String>,
    selection_state: SapTableSelectionState,
}

impl<'a> SapTableHeader {
    pub(super) fn new(
        table_def: SapTableDef,
        header_ref: ElementRef<'a>,
    ) -> Result<SapTableHeader, ElementError> {
        let row = header_ref.value();
        let subct_selector = scraper::Selector::parse("[subct]").unwrap();
        let subcts = header_ref.select(&subct_selector);
        let cells = subcts
            .filter_map(|subct_ref| SapTableCellDefWrapper::from_ref(table_def.clone(), subct_ref))
            .collect::<Vec<SapTableCellDefWrapper>>();
        let row_type = row.attr("rt").map(|s| s.into()).unwrap_or_default();
        if !matches!(row_type, SapTableRowType::Header) {
            return Err(ElementError::InvalidContent {
                element: table_def.id().to_string(),
                content: "Header of table is invalid".to_string(),
            });
        }
        Ok(SapTableHeader {
            table_def,
            cells,
            row_index: row.attr("rr").and_then(|s| s.parse::<u32>().ok()),
            user_data: row.attr("uDat").map(|s| s.to_owned()),
            drag_data: row.attr("ddData").map(|s| s.to_owned()),
            drop_target_info: row.attr("ddDti").map(|s| s.to_owned()),
            parent_drop_target_info: row.attr("ddPDti").map(|s| s.to_owned()),
            selection_state: row.attr("sst").map(|s| s.into()).unwrap_or_default(),
        })
    }

    /// 행 내부 셀의 개수를 반환합니다.
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// 행 내부 셀의 존재 여부를 반환합니다.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 행 내부 셀 정의의 [`Iterator`]를 반환합니다.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &SapTableCellDefWrapper> {
        self.cells.iter()
    }

    /// 행 내부 셀 엘리먼트의 [`Iterator`]를 반환합니다.
    pub fn iter_value(
        &'a self,
        parser: &'a ElementParser,
    ) -> impl ExactSizeIterator<Item = Result<SapTableCellWrapper<'a>, WebDynproError>> {
        self.cells
            .iter()
            .map(|def| SapTableCellWrapper::from_def(def, parser))
    }

    /// 헤더 행 제목들의 [`Vec`]를 반환합니다.
    pub fn titles(&'a self, parser: &'a ElementParser) -> Result<Vec<String>, WebDynproError> {
        self.iter()
            .map(|def| -> Result<String, WebDynproError> {
                let cell_wrapper = SapTableCellWrapper::from_def(def, parser)?;
                if let SapTableCellWrapper::Header(header_cell) = cell_wrapper {
                    if let Some(def_wrapper) = header_cell.content() {
                        if let ElementDefWrapper::Caption(caption_def) = def_wrapper {
                            Ok(parser
                                .element_from_def(&caption_def)?
                                .lsdata()
                                .text()
                                .unwrap_or(&String::default())
                                .to_string())
                        } else {
                            Err(ElementError::InvalidContent {
                                element: self.table_def().id().to_string(),
                                content: "Caption inside table header cell".to_string(),
                            })?
                        }
                    } else {
                        Err(ElementError::NoSuchContent {
                            element: self.table_def().id().to_string(),
                            content: "Caption inside table header cell".to_string(),
                        })?
                    }
                } else {
                    Err(ElementError::InvalidContent {
                        element: self.table_def().id().to_string(),
                        content: "Table header cell".to_string(),
                    })?
                }
            })
            .collect::<Result<Vec<String>, WebDynproError>>()
    }

    /// 원본 [`SapTable`](super::SapTable)의 [`ElementDefinition`]를 반환합니다.
    pub fn table_def(&self) -> SapTableDef {
        self.table_def.clone()
    }

    /// 인덱스를 반환합니다.
    pub fn row_index(&self) -> Option<u32> {
        self.row_index
    }

    /// 유저 데이터를 반환합니다.
    pub fn user_data(&self) -> Option<&str> {
        self.user_data.as_deref()
    }

    /// 드레그 데이터를 반환합니다.
    pub fn drag_data(&self) -> Option<&str> {
        self.drag_data.as_deref()
    }

    /// 드롭 타겟 정보를 반환합니다.
    pub fn drop_target_info(&self) -> Option<&str> {
        self.drop_target_info.as_deref()
    }

    /// 부모의 드롭 타겟 정보를 반환합니다.
    pub fn parent_drop_target_info(&self) -> Option<&str> {
        self.parent_drop_target_info.as_deref()
    }

    /// 선택 상태를 반환합니다.
    pub fn selection_state(&self) -> SapTableSelectionState {
        self.selection_state
    }
}

impl Index<usize> for SapTableHeader {
    type Output = SapTableCellDefWrapper;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}
