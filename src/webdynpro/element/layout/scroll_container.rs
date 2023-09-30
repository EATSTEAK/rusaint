use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::define_element_interactable;

// TODO: Implement additional events and data
define_element_interactable! {
    ScrollContainer<"SC", "ScrollContainer"> {},
    ScrollContainerLSData {
        is_selected: bool => "0",
        is_secondary_selected: bool => "1",
        enabled: bool => "2",
        cell_type: String => "3",
        row_description: String => "4",
        is_deselectable: bool => "5",
        tooltip: String => "6",
        custom_data: String => "7",
    }
}

impl<'a> ScrollContainer<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }
}
