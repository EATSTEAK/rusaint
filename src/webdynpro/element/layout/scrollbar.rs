use std::{borrow::Cow, cell::OnceCell};


use crate::webdynpro::element::define_element_interactable;

// TODO: Implement additional events and data
define_element_interactable! {
    Scrollbar<"SCB", "Scrollbar"> {},
    ScrollbarLSData {
        value: i32 => "0",
        maximum: i32 => "1",
        minimum: i32 => "2",
        large_change: i32 => "3",
        small_change: i32 => "4",
        scroll_direction: String => "5",
        scrolled_element_id: String => "6",
        show_scroll_tip: bool => "7",
        scroll_tip_value_description: String => "8",
        enabled: bool => "9",
        item_count: i32 => "10",
        custom_data: String => "11",
        visibility: String => "12",
    }
}

impl<'a> Scrollbar<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }
}
