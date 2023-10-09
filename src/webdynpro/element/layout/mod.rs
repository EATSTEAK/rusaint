use std::{borrow::Cow, cell::OnceCell};

use super::define_element_base;
use super::property::{Visibility, LockedDesign};

define_element_base! {
    FlowLayout<"FL", "FlowLayout"> {},
    FlowLayoutLSData {
        visibility: Visibility => "0",
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
        locked_design: LockedDesign => "3",
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
pub mod tab_strip;

#[doc(inline)]
pub use self::grid_layout::{GridLayout, GridLayoutLSData};
#[doc(inline)]
pub use self::tab_strip::{TabStrip, TabStripLSData};

mod button_row;
mod form;
mod popup_window;
mod scroll_container;
mod scrollbar;
mod tray;

pub use self::button_row::{ButtonRow, ButtonRowLSData};
pub use self::form::{Form, FormData, FormLSData};
pub use self::popup_window::{PopupWindow, PopupWindowLSData};
pub use self::scroll_container::{ScrollContainer, ScrollContainerLSData};
pub use self::scrollbar::{Scrollbar, ScrollbarLSData};
pub use self::tray::{Tray, TrayLSData};
