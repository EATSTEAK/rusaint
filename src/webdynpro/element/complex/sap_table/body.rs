use scraper::ElementRef;

use crate::webdynpro::{element::ElementDef, error::ElementError};

use super::{
    cell::SapTableCellWrapper,
    property::{SapTableRowType, SapTableSelectionState},
    SapTable,
};

pub struct SapTableBody<'a> {
    table_def: ElementDef<'a, SapTable<'a>>,
    elem_ref: ElementRef<'a>,
    header: SapTableRow<'a>,
    rows: Vec<SapTableRow<'a>>,
}

impl<'a> SapTableBody<'a> {
    pub(super) fn new(
        table_def: ElementDef<'a, SapTable<'a>>,
        elem_ref: ElementRef<'a>,
    ) -> Result<SapTableBody<'a>, ElementError> {
        let rows_iter = elem_ref
            .children()
            .filter_map(|node| scraper::ElementRef::wrap(node))
            .filter_map(|row_ref| SapTableRow::new(table_def, row_ref).ok());
        let mut header_iter = rows_iter
            .clone()
            .filter(|row| matches!(row.row_type, SapTableRowType::Header));
        let Some(header) = header_iter.next() else {
            return Err(ElementError::NoSuchContent { element: table_def.id().to_owned(), content: "Header of table".to_owned() });
        };
        if header_iter.next().is_some() {
            return Err(ElementError::InvalidContent {
                element: table_def.id().to_owned(),
                content: "Multiple header in table".to_owned(),
            });
        }
        let rows = rows_iter.skip(1).collect::<Vec<SapTableRow<'a>>>();
        Ok(SapTableBody {
            table_def,
            elem_ref,
            header,
            rows,
        })
    }
}

pub struct SapTableRow<'a> {
    table_def: ElementDef<'a, SapTable<'a>>,
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
            .filter_map(|subct_ref| SapTableCellWrapper::dyn_cell(table_def, subct_ref))
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
