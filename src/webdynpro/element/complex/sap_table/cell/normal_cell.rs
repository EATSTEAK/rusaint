use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;

use crate::webdynpro::{
    element::{
        complex::sap_table::property::{SapTableCellDesign, SapTableCellType},
        define_lsdata, Element, ElementWrapper, SubElement, SubElementDef,
    },
    error::WebDynproError,
};

use super::{SapTableCell, SapTableCellWrapper};

/// 일반 테이블 셀
#[derive(custom_debug_derive::Debug)]
pub struct SapTableNormalCell<'a> {
    id: Cow<'static, str>,
    #[debug(skip)]
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<SapTableNormalCellLSData>,
    content: OnceCell<Option<ElementWrapper<'a>>>,
}

define_lsdata! {
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
    fn content(&self) -> Option<&ElementWrapper<'a>> {
        self.content
            .get_or_init(|| {
                let content_selector = Selector::parse(":root [ct]").unwrap();
                ElementWrapper::dyn_elem(
                    self.element_ref
                        .select(&content_selector)
                        .next()?
                        .to_owned(),
                )
                .ok()
            })
            .as_ref()
    }
}

impl<'a> SubElement<'a> for SapTableNormalCell<'a> {
    const SUBCONTROL_ID: &'static str = "STC";
    const ELEMENT_NAME: &'static str = "SapTableNormalCell";

    type SubElementLSData = SapTableNormalCellLSData;

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
