use std::{borrow::Cow, cell::OnceCell, collections::HashMap};

use crate::webdynpro::element::property::{Mode, QuickViewDesign};
use crate::webdynpro::error::WebDynproError;
use crate::webdynpro::event::Event;

use crate::webdynpro::element::{Interactable, macros::define_element_interactable};

// TODO: Implement additional events and data
define_element_interactable! {
    #[doc = "브라우저 창 내부에 모달 등의 팝업으로 표시되는 창"]
    PopupWindow<"PW", "PopupWindow"> {},
    #[doc = "[`PopupWindow`]의 정의"]
    PopupWindowDef,
    #[doc = "[`PopupWindow`] 내부 데이터"]
    PopupWindowLSData {
        is_resizable: bool => "0",
        has_close_button: bool => "1",
        x: String => "2",
        y: String => "3",
        width: String => "4",
        height: String => "5",
        window_size: bool => "6",
        default_button_id: String => "7",
        hotkeys_id: String => "8",
        override_minimum_size: bool => "9",
        is_maximized: bool => "10",
        has_help_button: bool => "11",
        mode: Mode => "12",
        custom_data: String => "13",
        custom_style: String => "14",
        no_content_scrolling: bool => "15",
        context_menu_event: bool => "16",
        design: QuickViewDesign => "17"
    }
}

impl<'a> PopupWindow<'a> {
    /// HTML 엘리먼트로부터 새로운 [`PopupWindow`] 엘리먼트를 생성합니다.
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }

    /// 창을 닫는 이벤트를 반환합니다.
    pub fn close(&self) -> Result<Event, WebDynproError> {
        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        self.fire_event("Close".to_string(), parameters)
    }

    /// 도움 버튼을 누르는 이벤트를 반환합니다.
    pub fn help(&self) -> Result<Event, WebDynproError> {
        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        self.fire_event("Help".to_string(), parameters)
    }
}
