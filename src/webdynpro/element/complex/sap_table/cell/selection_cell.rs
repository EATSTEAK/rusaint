use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;

use crate::webdynpro::{
    element::{
        complex::sap_table::property::SapTableCellType, define_lsdata, Element, ElementDefWrapper,
        ElementWrapper, SubElement, SubElementDef,
    },
    error::WebDynproError,
};

use super::{SapTableCell, SapTableCellWrapper};

/// 선택 가능한 테이블 셀
#[derive(custom_debug_derive::Debug)]
pub struct SapTableSelectionCell<'a> {
    id: Cow<'static, str>,
    #[debug(skip)]
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<SapTableSelectionCellLSData>,
    content: OnceCell<Option<ElementDefWrapper<'a>>>,
}

define_lsdata! {
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

impl<'a> SubElement<'a> for SapTableSelectionCell<'a> {
    const SUBCONTROL_ID: &'static str = "SC";
    const ELEMENT_NAME: &'static str = "SapTableSelectionCell";

    type SubElementLSData = SapTableSelectionCellLSData;

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
