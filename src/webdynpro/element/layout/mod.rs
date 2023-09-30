use std::{borrow::Cow, cell::OnceCell};

use super::define_element_base;

define_element_base! {
    FlowLayout<"FL", "FlowLayout"> {},
    FlowLayoutLSData {
        visibility: String => "0",
        custom_data: String => "1"
    }
}

impl<'a> FlowLayout<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
        }
    }
}

define_element_base! {
    Container<"CO", "Container"> {},
    ContainerLSData {
        locked: bool => "0",
        printable: bool => "1",
        print_area: bool => "2",
        locked_design: String => "3",
        locked_message: String => "4",
        custom_data: String => "5",
        custom_style: String => "6"
    }
}

impl<'a> Container<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
        }
    }
}

pub mod grid_layout;
pub mod scroll_container;
