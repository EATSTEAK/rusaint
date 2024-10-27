use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::{
    macros::define_element_interactable,
    property::{ScrollingMode, Visibility},
};

use self::property::TrayDesign;

pub mod property {
    use serde::Deserialize;
    #[derive(Clone, Deserialize, Debug)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum TrayDesign {
        Transparent,
        Plain,
        Fill,
    }
}

// TODO: Implement additional events and data
define_element_interactable! {
    #[doc = "열고 닫을 수 있는 트레이"]
    Tray<"TY", "Tray"> {},
    #[doc = "[`Tray`]의 정의"]
    TrayDef,
    #[doc = "[`Tray`] 내부 데이터"]
    TrayLSData {
        title: String => "0",
        design: TrayDesign => "1",
        collapsed: bool => "2",
        enabled: bool => "3",
        tooltip: String => "4",
        height: String => "5",
        content_height: String => "6",
        has_option_menu: bool => "7",
        option_menu_id: String => "8",
        has_close_button: bool => "9",
        scrolling_mode: ScrollingMode => "10",
        has_toolbar: bool => "11",
        is_collapsible: bool => "12",
        accessibility_description: String => "13",
        visibility: Visibility => "14",
        default_button_id: String => "15",
        scroll_top: i32 => "16",
        scroll_left: i32 => "17",
        access_key: String => "18",
        hotkeys_id: String => "19",
        is_drag_handle: bool => "20",
        client_select: bool => "21",
        heading_level: i32 => "22",
        custom_data: String => "23",
        custom_style: String => "24",
    }
}

impl<'a> Tray<'a> {
    /// HTML 엘리먼트로부터 새로운 [`Tray`] 엘리먼트를 생성합니다.
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }
}
