use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::{macros::define_element_interactable, property::Visibility};

// TODO: Implement additional events and data
define_element_interactable! {
    #[doc = "버튼 등의 엘리먼트를 부연하는 라벨"]
    Label<"L", "Label"> {},
    #[doc = "[`Label`]의 정의"]
    LabelDef,
    #[doc = "[`Label`] 내부 데이터"]
    LabelLSData {
        tooltip: String => "0",
        label_for: String => "1",
        wrapping: bool => "2",
        text: String => "3",
        required: bool => "4",
        enabled: bool => "5",
        design_bar: String => "6",
        width: String => "7",
        has_icon: bool => "8",
        image_first: bool => "9",
        visibility: Visibility => "10",
        show_help: bool => "11",
        access_key: String => "12",
        align: String => "13",
        text_overflow: bool => "14",
        required_indicator_at_front: bool => "15",
        interaction_behavior: String => "16",
        is_link: bool => "17",
        editable: bool => "18",
        custom_data: String => "19",
        custom_style: String => "20",
        height: String => "21",
        labelled_by: String => "22",
    }
}

impl<'a> Label<'a> {
    /// HTML 엘리먼트로부터 새로운 [`Label`] 엘리먼트를 반환합니다.
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }
}
