use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::define_element_base;

use super::Element;

define_element_base! {
    ListBoxActionItem<"LIB_AI", "ListBoxActionItem"> {
        title: OnceCell<String>,
        text: OnceCell<String>,
    },
    ListBoxActionItemLSData {
        custom_data: String => "0",
    }
}

/* impl<'a> std::fmt::Debug for ListBoxActionItem<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListBoxActionItem")
            .field("id", &self.id())
            .field("lsdata", &self.lsdata())
            .field("text", &self.text())
            .field("title", &self.title())
            .finish()
    }
} */

impl<'a> ListBoxActionItem<'a> {
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            title: OnceCell::new(),
            text: OnceCell::new(),
        }
    }

    pub fn title(&self) -> &str {
        self.title.get_or_init(|| {
            self.element_ref
                .value()
                .attr("title")
                .unwrap_or("")
                .to_owned()
        })
    }

    pub fn text(&self) -> &str {
        self.text
            .get_or_init(|| self.element_ref().text().collect::<String>())
    }
}
