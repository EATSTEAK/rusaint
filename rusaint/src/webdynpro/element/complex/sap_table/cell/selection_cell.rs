use std::{borrow::Cow, cell::OnceCell};

use super::{SapTableCell, SapTableCellWrapper};
use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::element::{
    complex::{
        sap_table::{property::SapTableCellType, SapTableDef},
        SapTable,
    },
    sub::macros::define_subelement,
    ElementDefWrapper,
};

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
    fn content(&self, parser: &'a ElementParser) -> Option<ElementDefWrapper<'a>> {
        self.content
            .get_or_init(|| {
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

impl<'a> SapTableSelectionCell<'a> {
    /// HTML 엘리먼트로부터 [`SapTableSelectionCell`]을 생성합니다.
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
        SapTableCellWrapper::Selection(self)
    }
}
