use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::{
    macros::define_element_interactable,
    property::{
        EmbeddingBehaviour, IMEMode, InputFieldTextStyle, InputFieldType, SemanticColor,
        TabBehaviour, TableFieldDesign, Visibility,
    },
};

// TODO: Implement additional events and data
define_element_interactable! {
    #[doc = "입력 필드"]
    InputField<"I", "InputField"> {},
    #[doc = "[`InputField`]의 정의"]
    InputFieldDef,
    #[doc = "[`InputField`] 내부 데이터"]
    InputFieldLSData {
        value: String => "0",
        show_help_button: bool => "1",
        // This field originally named "type"
        input_field_type: InputFieldType => "2",
        width: String => "3",
        visibility: Visibility => "4",
        label_text: String => "5",
        hide_field_help: bool => "6",
        container_width_set: bool => "7",
        ime_mode: IMEMode => "8",
        auto_complete: bool => "9",
        format_string: String => "10",
        show_help_button_always: bool => "11",
        date_picker_start_ref_id: String => "12",
        access_key: String => "13",
        display_as_text: bool => "14",
        text_style: InputFieldTextStyle => "15",
        spinner_increment: i32 => "16",
        spinner_bounds_check: bool => "17",
        spinner_max: i32 => "18",
        spinner_min: i32 => "19",
        sap_table_field_design: TableFieldDesign => "20",
        validation_trigger: String => "21",
        tab_behaviour: TabBehaviour => "22",
        semantic_color: SemanticColor => "23",
        embedding_behaviour: EmbeddingBehaviour => "24",
        field_help_floating: bool => "25",
        first_day_of_week: i32 => "26",
        custom_data: String => "27",
        custom_style: String => "28",
        field_help_embedding: bool => "29",
        height: String => "30",
        labelled_by: String => "31",
        described_by: String => "32",
    }
}

impl<'a> InputField<'a> {
    /// HTML 엘리먼트로부터 새로운 [`InputField`]를 생성합니다.
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }

    /// 이 [`InputField`]의 값을 가져옵니다.
    pub fn value(&self) -> Option<&str> {
        self.element_ref.attr("value")
    }
}
