use std::{borrow::Cow, cell::OnceCell};

use super::{SapTableCell, SapTableCellWrapper};
use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::element::{
    complex::{
        sap_table::{
            property::{
                SapTableHeaderCellDesign, SapTableHeaderCellType, SapTableRowSelectionMassState,
                SapTableSelectionColumnAction,
            },
            SapTableDef,
        },
        SapTable,
    },
    property::SortState,
    sub::macros::define_subelement,
    ElementDefWrapper,
};

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
    fn content(&self, parser: &'a ElementParser) -> Option<ElementDefWrapper<'a>> {
        self.content
            .get_or_init(|| {
                // TODO: Reduce codes for get children tags
                let content_tag = self
                    .tag
                    .query_selector(
                        parser.dom().parser(),
                        format!(r#"[id="{}-CONTENT"] [ct]"#, &self.id).as_str(),
                    )
                    .into_iter()
                    .flatten()
                    .next()?
                    .get(parser.dom().parser())?
                    .as_tag()?
                    .to_owned();

                ElementDefWrapper::from_tag(content_tag).ok()
            })
            .to_owned()
    }
}

impl<'a> SapTableHeaderCell<'a> {
    /// HTML 엘리먼트로부터 [`SapTableHeaderCell`]을 생성합니다.
    pub const fn new(id: Cow<'static, str>, tag: tl::HTMLTag<'a>) -> Self {
        Self {
            id,
            tag,
            lsdata: OnceCell::new(),
            content: OnceCell::new(),
        }
    }

    /// 셀을 [`SapTableCellWrapper`]로 감쌉니다.
    pub fn wrap(self) -> SapTableCellWrapper<'a> {
        SapTableCellWrapper::Header(self)
    }
}
