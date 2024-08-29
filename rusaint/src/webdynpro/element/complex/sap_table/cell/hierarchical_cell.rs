use std::{borrow::Cow, cell::OnceCell};

use super::{SapTableCell, SapTableCellWrapper};
use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::element::{
    complex::{
        sap_table::{
            property::{SapTableCellDesign, SapTableHierarchicalCellStatus},
            SapTableDef,
        },
        SapTable,
    },
    sub::macros::define_subelement,
    ElementDefWrapper,
};

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

impl<'a> SapTableHierarchicalCell<'a> {
    /// HTML 엘리먼트로부터 [`SapTableHierarchicalCell`]을 생성합니다.
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
        SapTableCellWrapper::Hierarchical(self)
    }
}
