use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::define_element_interactable;

// TODO: Implement additional events and data
define_element_interactable! {
    GridLayoutCell<"GLC", "GridLayoutCell"> {},
    GridLayoutCellLSData {
        drag_data: String => "0",
        semantic_color: String => "1",
        custom_data: String => "2",
        layout_cell_position: String => "3",
        custom_style: String => "4",
    }
}

impl<'a> GridLayoutCell<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }
}
