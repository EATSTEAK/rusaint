use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;

use crate::webdynpro::{
    element::{
        ElementDefWrapper,
        complex::{
            SapTable,
            sap_table::{
                SapTableDef,
                property::{
                    SapTableHeaderCellDesign, SapTableHeaderCellType,
                    SapTableRowSelectionMassState, SapTableSelectionColumnAction,
                },
            },
        },
        property::SortState,
        sub::macros::define_subelement,
    },
    error::BodyError,
};

use super::{SapTableCell, SapTableCellWrapper};

define_subelement! {
    #[doc = "[`SapTable`]의 헤더 셀"]
    SapTableHeaderCell<SapTable, SapTableDef, "HC", "SapTableHeaderCell"> {
        content: OnceCell<Option<ElementDefWrapper<'a>>>
    },
    #[doc = "[`SapTableHeaderCell`]의 정의"]
    SapTableHeaderCellDef,
    #[doc = "[`SapTableHeaderCell`] 내부 데이터"]
    SapTableHeaderCellLSData {
        sort_state: SortState => "0",
        header_cell_design: SapTableHeaderCellDesign => "1",
        header_cell_type: SapTableHeaderCellType => "2",
        selection_column_action: SapTableSelectionColumnAction => "3",
        selection_menu_id: String => "4",
        row_selection_mass_state: SapTableRowSelectionMassState => "5",
        required: bool => "6",
        tooltip: String => "7",
        column_selected: bool => "8",
        column_selectable: bool => "9",
        filtered: bool => "10",
        mark_totals: bool => "11",
        accessibility_description: String => "12",
        icon_tooltip: String => "13",
        icon_first: bool => "14",
        icon_enabled: bool => "15",
        custom_style: String => "16",
        custom_data: String => "17",
    }
}

impl<'a> SapTableCell<'a> for SapTableHeaderCell<'a> {
    fn content(&self) -> Option<ElementDefWrapper<'a>> {
        self.content
            .get_or_init(|| {
                let content_selector =
                    Selector::parse(format!(r#"[id="{}-CONTENT"] [ct]"#, &self.id).as_str())
                        .or(Err(BodyError::InvalidSelector))
                        .ok()?;
                ElementDefWrapper::from_ref(
                    self.element_ref
                        .select(&content_selector)
                        .next()?
                        .to_owned(),
                )
                .ok()
            })
            .to_owned()
    }
}

impl<'a> SapTableHeaderCell<'a> {
    /// HTML 엘리먼트로부터 [`SapTableHeaderCell`]을 생성합니다.
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
