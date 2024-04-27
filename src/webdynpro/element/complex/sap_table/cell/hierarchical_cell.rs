use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;

use crate::webdynpro::{
    element::{
        complex::sap_table::property::{SapTableCellDesign, SapTableHierarchicalCellStatus},
        define_lsdata, Element, ElementWrapper, SubElement, SubElementDef,
    },
    error::WebDynproError,
};

use super::{SapTableCell, SapTableCellWrapper};

/// 계층적 테이블의 셀
#[derive(custom_debug_derive::Debug)]
pub struct SapTableHierarchicalCell<'a> {
    id: Cow<'static, str>,
    #[debug(skip)]
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<SapTableHierarchicalCellLSData>,
    content: OnceCell<Option<ElementWrapper<'a>>>,
}
define_lsdata! {
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

impl<'a> SubElement<'a> for SapTableHierarchicalCell<'a> {
    const SUBCONTROL_ID: &'static str = "HIC";
    const ELEMENT_NAME: &'static str = "SapTableHierarchicalCell";

    type SubElementLSData = SapTableHierarchicalCellLSData;

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
        Ok(Self::new(elem_def.id().into(), element))
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn element_ref(&self) -> &scraper::ElementRef<'a> {
        &self.element_ref
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
