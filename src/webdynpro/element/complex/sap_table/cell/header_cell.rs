use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;

use crate::webdynpro::{
    element::{
        complex::sap_table::property::{
            SapTableHeaderCellDesign, SapTableHeaderCellType, SapTableRowSelectionMassState,
            SapTableSelectionColumnAction,
        }, define_lsdata, property::SortState, Element, ElementDefWrapper, SubElement, SubElementDef
    },
    error::{BodyError, WebDynproError},
};

use super::{SapTableCell, SapTableCellWrapper};

/// 테이블의 헤더 셀
#[derive(custom_debug_derive::Debug)]
pub struct SapTableHeaderCell<'a> {
    id: Cow<'static, str>,
    #[debug(skip)]
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<SapTableHeaderCellLSData>,
    content: OnceCell<Option<ElementDefWrapper<'a>>>,
}

define_lsdata! {
    #[doc = "[`SapTableHeaderCell`] 내부 데이터"]
    SapTableHeaderCellLSData {
        sort_state: SortState => "0",
        header_cell_design: SapTableHeaderCellDesign => "1",
        header_cell_type: SapTableHeaderCellType => "2",
        selection_column_action: SapTableSelectionColumnAction => "3",
        selection_menu_id: String => "4",
        row_selection_mass_state: SapTableRowSelectionMassState => "5",
        required: bool => "6",
        tooltip: String => "7",
        column_selected: bool => "8",
        column_selectable: bool => "9",
        filtered: bool => "10",
        mark_totals: bool => "11",
        accessibility_description: String => "12",
        icon_tooltip: String => "13",
        icon_first: bool => "14",
        icon_enabled: bool => "15",
        custom_style: String => "16",
        custom_data: String => "17",
    }
}

impl<'a> SubElement<'a> for SapTableHeaderCell<'a> {
    const SUBCONTROL_ID: &'static str = "HC";
    const ELEMENT_NAME: &'static str = "SapTableHeaderCell";

    type SubElementLSData = SapTableHeaderCellLSData;

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

impl<'a> SapTableCell<'a> for SapTableHeaderCell<'a> {
    fn content(&self) -> Option<ElementDefWrapper<'a>> {
        self.content
            .get_or_init(|| {
                let content_selector =
                    Selector::parse(format!(r#"[id="{}-CONTENT"] [ct]"#, &self.id).as_str())
                        .or(Err(BodyError::InvalidSelector))
                        .ok()?;
                ElementDefWrapper::dyn_elem_def(
                    self.element_ref
                        .select(&content_selector)
                        .next()?
                        .to_owned(),
                )
                .ok()
            }).to_owned()
    }
}

impl<'a> SapTableHeaderCell<'a> {
    /// HTML 엘리먼트로부터 [`SapTableHeaderCell`]을 생성합니다.
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
        SapTableCellWrapper::Header(self)
    }
}
