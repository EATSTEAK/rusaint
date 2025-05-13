use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;

use crate::webdynpro::element::{
    ElementDefWrapper,
    complex::{
        SapTable,
        sap_table::{SapTableDef, property::SapTableCellType},
    },
    sub::macros::define_subelement,
};

use super::{SapTableCell, SapTableCellWrapper};

define_subelement! {
    #[doc = "선택 가능한 [`SapTable`]의 셀"]
    SapTableSelectionCell<SapTable, SapTableDef, "SC", "SapTableSelectionCell"> {
        content: OnceCell<Option<ElementDefWrapper<'a>>>,
    },
    #[doc = "[`SapTableSelectionCell`]의 정의"]
    SapTableSelectionCellDef,
    #[doc = "[`SapTableSelectionCell`] 내부 데이터"]
    SapTableSelectionCellLSData {
        is_selected: bool => "0",
        is_secondary_selected: bool => "1",
        enabled: bool => "2",
        cell_type: SapTableCellType => "3",
        row_description: String => "4",
        is_deselectable: bool => "5",
        tooltip: String => "6",
        custom_data: String => "7",
    }
}

impl<'a> SapTableCell<'a> for SapTableSelectionCell<'a> {
    fn content(&self) -> Option<ElementDefWrapper<'a>> {
        self.content
            .get_or_init(|| {
                let content_selector = Selector::parse(":root > div > div [ct]").unwrap();
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

impl<'a> SapTableSelectionCell<'a> {
    /// HTML 엘리먼트로부터 [`SapTableSelectionCell`]을 생성합니다.
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
        SapTableCellWrapper::Selection(self)
    }
}
