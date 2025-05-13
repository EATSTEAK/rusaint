use std::{borrow::Cow, cell::OnceCell, collections::HashMap};

use crate::webdynpro::{
    element::{
        Interactable,
        macros::define_element_interactable,
        property::{HotkeyValue, Visibility},
    },
    error::WebDynproError,
    event::Event,
};

define_element_interactable! {
    #[doc = "액션을 수행하거나 링크로 이동하는 하이퍼링크"]
    Link<"LN", "Link"> {},
    #[doc = "[`Link`]의 정의"]
    LinkDef,
    #[doc ="[`Link`] 내부 데이터"]
    LinkLSData {
        tooltip: String => "0",
        text: String => "1",
        has_reference: bool => "2",
        enabled: bool => "3",
        has_link_caption: bool => "4",
        visibility: Visibility => "5",
        label_text: String => "6",
        emphasized: bool => "7",
        access_key: String => "8",
        hotkey: HotkeyValue => "9",
        custom_data: String => "10",
        custom_style: String => "11",
        labelled_by: String => "12",
    }
}

impl<'a> Link<'a> {
    /// HTML 엘리먼트로부터 [`Link`] 엘리먼트를 생성합니다.
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }

    /// 링크 활성화 이벤트를 반환합니다. `ctrl` 이나 `shift` 가 참일 경우 각 버튼을 누른 채로 클릭한 것으로 간주합니다.
    pub fn activate(&self, ctrl: bool, shift: bool) -> Result<Event, WebDynproError> {
        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("Ctrl".to_string(), ctrl.to_string());
        parameters.insert("Shift".to_string(), shift.to_string());
        self.fire_event("Activate".to_string(), parameters)
    }

    /// 더블 클릭 이벤트를 반환합니다.
    pub fn double_click(&self) -> Result<Event, WebDynproError> {
        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        self.fire_event("DoubleClick".to_string(), parameters)
    }
}
