use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::define_element_interactable;

// TODO: Implement additional events and data
define_element_interactable! {
    #[doc = "이미지"]
    Image<"IMG", "Image"> {},
    ImageLSData {
        tooltip: String => "0",
        width: String => "1",
        height: String => "2",
        src: String => "3",
        is_interactive: bool => "4",
        has_image_map: bool => "5",
        visibility: String => "6",
        is_nested: bool => "7",
        label_text: String => "8",
        adjust_image_size: bool => "9",
        drag_source_info: String => "10",
        is_drag_handle: bool => "11",
        enabled: bool => "12",
        error_image_src: String => "13",
        custom_data: String => "14",
        its_mode: bool => "15",
        its_display_mode: String => "16",
        custom_style: String => "17",
        drop_target_info: String => "18",
        vertical_text_align: String => "19",
        horizontal_text_align: String => "20",
        used_in_sap_table: bool => "21",
        labelled_by: String => "22",
    }
}

impl<'a> Image<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }
}
