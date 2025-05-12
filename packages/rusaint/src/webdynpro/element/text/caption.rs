use std::{borrow::Cow, cell::OnceCell};

use scraper::Node;

use crate::webdynpro::element::{
    Element, macros::define_element_interactable, property::Visibility,
};

define_element_interactable! {
    #[doc = "엘리먼트 제목 부분 등에서 사용되는 캡션"]
    #[doc = ""]
    #[doc = "이 엘리먼트는 단독 엘리먼트로 존재하지 않고, [`SapTableHeaderCell`]이나 [`Tray`]같은 엘리먼트의 제목 부분에 활용됩니다."]
    #[doc = ""]
    #[doc = "[`SapTableHeaderCell`]: crate::webdynpro::element::complex::sap_table::cell::SapTableHeaderCell"]
    #[doc = "[`Tray`]: crate::webdynpro::element::layout::Tray"]
    Caption<"CP", "Caption"> {
        text: OnceCell<String>,
    },
    #[doc = "[`Caption`]의 정의"]
    CaptionDef,
    #[doc = "[`Caption`] 내부 데이터"]
    CaptionLSData {
        tooltip: String=> "0",
        text: String => "1",
        image_src: String => "2",
        image_first: bool => "3",
        image_width: String => "4",
        image_height: String => "5",
        is_nested: bool => "6",
        visibility: Visibility => "7",
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
    /// HTML 엘리먼트로부터 [`Caption`] 엘리먼트를 생성합니다.
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
            text: OnceCell::new(),
        }
    }

    /// 내부 텍스트를 반환합니다.
    pub fn text(&self) -> &str {
        self.text.get_or_init(|| {
            self.element_ref()
                .children()
                .filter_map(|node| match node.value() {
                    Node::Text(text) => Some(text.to_string()),
                    Node::Element(elem) => {
                        if elem.name() == "br" {
                            Some("\n".to_string())
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .collect::<String>()
        })
    }
}
