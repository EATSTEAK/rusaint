use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::{macros::define_element_interactable, property::Visibility};

define_element_interactable! {
    #[doc = "체크박스"]
    CheckBox<"C_standards", "CheckBox"> {},
    #[doc = "[`CheckBox`]의 정의"]
    CheckBoxDef,
    #[doc = "[`CheckBox`]의 내부 데이터"]
    CheckBoxLSData {
        name: String => "0",
        checked: bool => "1",
        enabled: bool => "2",
        readonly: bool => "3",
        text: String => "4",
        tooltip: String => "5",
        invalid: bool => "6",
        visibility: Visibility => "7",
        show_help: bool => "8",
        input_state: String => "9",
        access_key: String => "10",
        arrangement: String => "11",
        associated_edit_context: String => "12",
        text_design: String => "13",
        used_in_sap_table: bool => "14",
        custom_data: String => "15",
        custom_style: String => "16",
        text_overflow: bool => "17",
        height: String => "18",
        is_text_label: bool => "19",
        labelled_by: String => "20"
    }
}

impl<'a> CheckBox<'a> {
    /// HTML 엘리먼트로부터 새로운 [`CheckBox`] 엘리먼트를 생성합니다.
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }

    /// 이 [`CheckBox`]가 체크되었는지 여부를 반환합니다.
    pub fn checked(&self) -> bool {
        self.element_ref
            .attr("aria-checked")
            .is_some_and(|str| str == "true")
    }

    /// 이 [`CheckBox`]가 읽기 전용인지 여부를 반환합니다.
    pub fn readonly(&self) -> bool {
        self.element_ref
            .attr("aria-readonly")
            .is_some_and(|str| str == "true")
    }

    /// 이 [`CheckBox`]가 비활성화 상태인지 여부를 반환합니다.
    pub fn disabled(&self) -> bool {
        self.element_ref
            .attr("aria-disabled")
            .is_some_and(|str| str == "true")
    }

    /// 이 [`CheckBox`]가 올바르지 않은 상태인지 여부를 반환합니다.
    pub fn invalid(&self) -> bool {
        self.element_ref
            .attr("aria-invalid")
            .is_some_and(|str| str == "true")
    }
}
