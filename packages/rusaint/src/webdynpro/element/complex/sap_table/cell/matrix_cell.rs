use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;

use crate::webdynpro::element::{
    ElementDefWrapper,
    complex::{
        SapTable,
        sap_table::{SapTableDef, property::SapTableCellDesign},
    },
    sub::macros::define_subelement,
};

use super::{SapTableCell, SapTableCellWrapper};

define_subelement! {
    #[doc = "매트릭스 형태의 [`SapTable`] 셀"]
    SapTableMatrixCell<SapTable, SapTableDef, "MC", "SapTableMatrixCell"> {
        content: OnceCell<Option<ElementDefWrapper<'a>>>,
    },
    #[doc = "[`SapTableMatrixCell`]의 정의"]
    SapTableMatrixCellDef,
    #[doc = "[`SapTableMatrixCell`] 내부 데이터"]
    SapTableMatrixCellLSData {
        cell_background_design: SapTableCellDesign => "0",
        header_cell_ids: String => "1",
        row_header_cell_ids: String => "2",
        custom_data: String => "3",
    }
}

impl<'a> SapTableCell<'a> for SapTableMatrixCell<'a> {
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

impl<'a> SapTableMatrixCell<'a> {
    /// HTML 엘리먼트로부터 [`SapTableMatrixCell`]을 생성합니다.
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
        SapTableCellWrapper::Matrix(self)
    }
}
