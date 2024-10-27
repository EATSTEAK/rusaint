use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::{
    macros::define_element_interactable,
    property::{ScrollingMode, Visibility},
};

// TODO: Implement additional events and data
define_element_interactable! {
    #[doc = "스크롤을 처리하는 컨테이너"]
    ScrollContainer<"SC", "ScrollContainer"> {},
    #[doc = "[`ScrollContainer`]의 정의"]
    ScrollContainerDef,
    #[doc = "[`ScrollContainer`] 내부 데이터"]
    ScrollContainerLSData {
        scrolling_mode: ScrollingMode => "0",
        visibility: Visibility => "1",
        accessibility_description: String => "2",
        is_layout: bool => "3",
        default_button_id: String => "4",
        tooltip: String => "5",
        scroll_top: i32 => "6",
        scroll_left: i32 => "7",
        hotkeys_id: String => "8",
        custom_data: String => "9",
        custom_style: String => "10",
        labelled_by: String => "11",
    }
}

impl<'a> ScrollContainer<'a> {
    /// HTML 엘리먼트로부터 새로운 [`ScrollContainer`] 엘리먼트를 생성합니다.
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }
}
