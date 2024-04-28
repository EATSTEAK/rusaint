use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;

use crate::webdynpro::{
    element::{
        complex::sap_table::property::SapTableCellDesign, define_lsdata, Element,
        ElementDefWrapper, SubElement, SubElementDef,
    },
    error::WebDynproError,
};

use super::{SapTableCell, SapTableCellWrapper};

/// 매트릭스 테이블의 셀
#[derive(custom_debug_derive::Debug)]
pub struct SapTableMatrixCell<'a> {
    id: Cow<'static, str>,
    #[debug(skip)]
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<SapTableMatrixCellLSData>,
    content: OnceCell<Option<ElementDefWrapper<'a>>>,
}

define_lsdata! {
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
                ElementDefWrapper::dyn_elem_def(
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

impl<'a> SubElement<'a> for SapTableMatrixCell<'a> {
    const SUBCONTROL_ID: &'static str = "MC";
    const ELEMENT_NAME: &'static str = "SapTableMatrixCell";

    type SubElementLSData = SapTableMatrixCellLSData;

    fn lsdata(&self) -> &Self::SubElementLSData {
        self.lsdata.get_or_init(|| {
            let Ok(lsdata_obj) = Self::lsdata_elem(self.element_ref) else {
                return Self::SubElementLSData::default();
            };
            serde_json::from_value::<Self::SubElementLSData>(lsdata_obj)
                .unwrap_or(Self::SubElementLSData::default())
        })
    }

    fn from_elem<Parent: Element<'a>>(
        elem_def: SubElementDef<'a, Parent, Self>,
        element: scraper::ElementRef<'a>,
    ) -> Result<Self, WebDynproError> {
        Ok(Self::new(elem_def.id_cow(), element))
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn element_ref(&self) -> &scraper::ElementRef<'a> {
        &self.element_ref
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
