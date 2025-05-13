use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;

use crate::webdynpro::element::{
    ElementDefWrapper,
    complex::{
        SapTable,
        sap_table::{
            SapTableDef,
            property::{SapTableCellDesign, SapTableHierarchicalCellStatus},
        },
    },
    sub::macros::define_subelement,
};

use super::{SapTableCell, SapTableCellWrapper};

define_subelement! {
    #[doc = "계층적 [`SapTable`]의 셀"]
    SapTableHierarchicalCell<SapTable, SapTableDef, "HIC", "SapTableHierarchicalCell"> {
        content: OnceCell<Option<ElementDefWrapper<'a>>>
    },
    #[doc = "[`SapTableHierarchicalCell`]의 정의"]
    SapTableHierarchicalCellDef,
    #[doc = "[`SapTableHierarchicalCell`] 내부 데이터"]
    SapTableHierarchicalCellLSData {
        is_selected: bool => "0",
        is_secondary_selected: bool => "1",
        cell_design: SapTableCellDesign => "2",
        header_cell_ids: String => "3",
        level: i32 => "4",
        status: SapTableHierarchicalCellStatus => "5",
        status_enabled: bool => "6",
        content_type_tooltip: String => "7",
        custom_style: String => "8",
        custom_data: String => "9",
    }
}

impl<'a> SapTableCell<'a> for SapTableHierarchicalCell<'a> {
    fn content(&self) -> Option<ElementDefWrapper<'a>> {
        self.content
            .get_or_init(|| {
                let content_selector = Selector::parse(":root [ct]").unwrap();
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

impl<'a> SapTableHierarchicalCell<'a> {
    /// HTML 엘리먼트로부터 [`SapTableHierarchicalCell`]을 생성합니다.
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
        SapTableCellWrapper::Hierarchical(self)
    }
}
