use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::macros::define_element_interactable;

// TODO: Implement additional events and data
define_element_interactable! {
    #[doc = "[`GridLayout`](crate::webdynpro::element::layout::GridLayout) 내부 셀"]
    GridLayoutCell<"GLC", "GridLayoutCell"> {},
    #[doc = "[`GridLayoutCell`]의 정의"]
    GridLayoutCellDef,
    #[doc = "[`GridLayoutCell`] 내부 데이터"]
    GridLayoutCellLSData {
        drag_data: String => "0",
        semantic_color: String => "1",
        custom_data: String => "2",
        layout_cell_position: String => "3",
        custom_style: String => "4",
    }
}

impl<'a> GridLayoutCell<'a> {
    /// HTML 엘리먼트로부터 새로운 [`GridLayoutCell`] 엘리먼트를 생성합니다.
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }
}
