use std::{borrow::Cow, cell::OnceCell};

use super::macros::define_element_base;
use super::property::{LockedDesign, Visibility};

define_element_base! {
    #[doc = "HTML `flow` 레이아웃"]
    FlowLayout<"FL", "FlowLayout"> {},
    #[doc = "[`FlowLayout`]의 정의"]
    FlowLayoutDef,
    #[doc = "[`FlowLayout`] 내부 데이터"]
    FlowLayoutLSData {
        visibility: Visibility => "0",
        custom_data: String => "1"
    }
}

impl<'a> FlowLayout<'a> {
    /// HTML 엘리먼트로부터 새로운 [`FlowLayout`] 엘리먼트를 생성합니다.
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
        }
    }
}

define_element_base! {
    #[doc = "가상 컨테이너"]
    Container<"CO", "Container"> {},
    #[doc = "[`Container`]의 정의"]
    ContainerDef,
    #[doc = "[`Container`] 내부 데이터"]
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
    /// HTML 엘리먼트로부터 새로운 [`Container`] 엘리먼트를 생성합니다.
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
        }
    }
}

/// [`GridLayout`] 엘리먼트 모듈
pub mod grid_layout;
/// [`TabStrip`] 엘리먼트 모듈
pub mod tab_strip;

#[doc(inline)]
pub use self::grid_layout::{GridLayout, GridLayoutDef, GridLayoutLSData};
#[doc(inline)]
pub use self::tab_strip::{TabStrip, TabStripDef, TabStripLSData};

mod button_row;
mod form;
mod popup_window;
mod scroll_container;
mod scrollbar;
mod tray;

pub use self::button_row::{ButtonRow, ButtonRowDef, ButtonRowLSData};
pub use self::form::{Form, FormData, FormDef, FormLSData};
pub use self::popup_window::{PopupWindow, PopupWindowDef, PopupWindowLSData};
pub use self::scroll_container::{ScrollContainer, ScrollContainerDef, ScrollContainerLSData};
pub use self::scrollbar::{Scrollbar, ScrollbarDef, ScrollbarLSData};
pub use self::tray::{Tray, TrayDef, TrayLSData};
