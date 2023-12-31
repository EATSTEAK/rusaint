use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::{define_element_interactable, Element};

define_element_interactable! {
    #[doc = "텍스트 표시 뷰"]
    TextView<"TV", "TextView"> {
        text: OnceCell<String>
    },
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
        visibility: String => "8",
        text_overflow: bool => "9",
    }
}

impl<'a> TextView<'a> {
    /// HTML 엘리먼트로부터 새로운 [`TextView`] 엘리먼트를 반환합니다.
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
        self.text
            .get_or_init(|| self.element_ref().text().collect::<String>())
    }
}
