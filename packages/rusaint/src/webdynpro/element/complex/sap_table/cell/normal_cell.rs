use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;

use crate::webdynpro::element::{
    ElementDefWrapper,
    complex::{
        SapTable,
        sap_table::{
            SapTableDef,
            property::{SapTableCellDesign, SapTableCellType},
        },
    },
    sub::macros::define_subelement,
};

use super::{SapTableCell, SapTableCellWrapper};

define_subelement! {
    #[doc = "일반 [`SapTable`] 셀"]
    SapTableNormalCell<SapTable, SapTableDef, "STC", "SapTableNormalCell"> {
        content: OnceCell<Option<ElementDefWrapper<'a>>>,
    },
    #[doc = "[`SapTableNormalCell`]의 정의"]
    SapTableNormalCellDef,
    #[doc = "[`SapTableNormalCell`] 내부 데이터"]
    SapTableNormalCellLSData {
        is_selected: bool => "0",
        is_secondary_selected: bool => "1",
        cell_type: SapTableCellType => "2",
        cell_design: SapTableCellDesign => "3",
        header_cell_ids: String => "4",
        row_header_cell_ids: String => "5",
        custom_style: String => "6",
        custom_data: String => "7",
    }
}

impl<'a> SapTableCell<'a> for SapTableNormalCell<'a> {
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

impl<'a> SapTableNormalCell<'a> {
    /// HTML 엘리먼트로부터 [`SapTableNormalCell`]을 생성합니다.
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
        SapTableCellWrapper::Normal(self)
    }
}
