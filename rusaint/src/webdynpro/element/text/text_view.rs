use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::{
    macros::define_element_interactable, parser::ElementParser, property::Visibility, Element
};

define_element_interactable! {
    #[doc = "텍스트 표시 뷰"]
    TextView<"TV", "TextView"> {
        text: OnceCell<String>
    },
    #[doc = "[`TextView`]의 정의"]
    TextViewDef,
    #[doc = "[`TextView`] 내부 데이터"]
    TextViewLSData {
        tooltip: String => "0",
        required: bool => "1",
        enabled: bool => "2",
        design: String => "3",
        layout: String => "4",
        semantic_color: String => "5",
        semantic_bg_color: String => "6",
        is_nested: bool => "7",
        visibility: Visibility => "8",
        text_overflow: bool => "9",
    }
}

impl<'a> TextView<'a> {
    /// HTML 엘리먼트로부터 새로운 [`TextView`] 엘리먼트를 반환합니다.
    pub fn new(id: Cow<'static, str>, tag: tl::HTMLTag<'a>) -> Self {
        Self {
            id,
            tag,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
            text: OnceCell::new(),
        }
    }

    /// 내부 텍스트를 반환합니다.
    pub fn text(&self, parser: &'a ElementParser) -> &str {
        self.text.get_or_init(|| {
            self.tag().inner_text(parser.dom().parser()).to_string()
        })
    }
}
