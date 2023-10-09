use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::define_element_interactable;

define_element_interactable! {
    Caption<"CP", "Caption"> {
    },
    CaptionLSData {
        tooltip: String=> "0",
        text: String => "1",
        image_src: String => "2",
        image_first: bool => "3",
        image_width: String => "4",
        image_height: String => "5",
        is_nested: bool => "6",
        visibility: String => "7",
        is_drag_handle: bool => "8",
        hover_image_src: String => "9",
        drag_source_info: String => "10",
        editable: bool => "11",
        semantic_color: String => "12",
        design: String => "13",
        custom_data: String => "14",
        labelled_by: String => "15",
    }
}

impl<'a> Caption<'a> {
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }
}
