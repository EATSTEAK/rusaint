use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::define_element_base;

define_element_base! {
    // Note: This element renders as "TSITM_ie6" if >= IE6
    TabStripItem<"TSITM_standards", "TabStripTab"> {},
    TabStripItemLSData {
        id: String => "0",
        index: i32 => "1",
        caption: String => "2",
        has_title_caption: bool => "3",
        tooltip: String => "4",
        enabled: bool => "5",
        scrolling_mode: String => "6",
        has_toolbar: bool => "7",
        default_button_id: String => "8",
        is_closable: bool => "9",
        scroll_top: i32 => "10",
        scroll_left: i32 => "11",
        client_tab_select: bool => "12",
        hotkeys_id: String => "13",
        access_key: String => "14",
        has_editable_title: bool => "15",
        area_design: String => "16",
        custom_data: String => "17",
        custom_style: String => "18",
        visibility: String => "19",
    }
}

impl<'a> TabStripItem<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
        }
    }
}
