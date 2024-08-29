use std::{borrow::Cow, cell::OnceCell};

use super::{SapTableCell, SapTableCellWrapper};
use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::element::{
    complex::{
        sap_table::{property::SapTableCellDesign, SapTableDef},
        SapTable,
    },
    sub::macros::define_subelement,
    ElementDefWrapper,
};

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

impl<'a> SapTableMatrixCell<'a> {
    /// HTML 엘리먼트로부터 [`SapTableMatrixCell`]을 생성합니다.
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
        SapTableCellWrapper::Matrix(self)
    }
}
