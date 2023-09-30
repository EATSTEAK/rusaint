use std::{borrow::Cow, cell::OnceCell};

use super::{define_element_interactable, Element};

define_element_interactable! {
    TextView<"TV", "TextView"> {
        text: OnceCell<String>
    },
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
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
            text: OnceCell::new(),
        }
    }

    pub fn text(&self) -> &str {
        self.text
            .get_or_init(|| self.element_ref().text().collect::<String>())
    }
}
