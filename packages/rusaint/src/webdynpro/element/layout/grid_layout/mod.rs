use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::{macros::define_element_interactable, property::Visibility};

// TODO: Implement additional events and data
define_element_interactable! {
    #[doc = "HTML `grid` 레이아웃"]
    GridLayout<"GL", "GridLayout"> {},
    #[doc = "[`GridLayout`]의 정의"]
    GridLayoutDef,
    #[doc = "[`GridLayout`] 내부 데이터"]
    GridLayoutLSData {
        height: String => "0",
        visibility: Visibility => "1",
        drag_source_info: String => "2",
        drop_target_info: String => "3",
        drop_decorator_type: String => "4",
        custom_style: String => "5",
        custom_data: String => "6",
    }
}

impl<'a> GridLayout<'a> {
    /// HTML 엘리먼트로부터 새로운 [`GridLayout`] 엘리먼트를 생성합니다.
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }
}

/// [`GridLayout`] 내부 셀
pub mod cell;
