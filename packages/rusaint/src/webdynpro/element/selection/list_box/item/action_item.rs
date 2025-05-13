use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::{Element, macros::define_element_base};

define_element_base! {
    #[doc = "실행할 수 있는 액션이 포함된 [`ListBox`](crate::webdynpro::element::selection::list_box::ListBox)의 아이템"]
    ListBoxActionItem<"LIB_AI", "ListBoxActionItem"> {
        title: OnceCell<String>,
        text: OnceCell<String>,
    },
    #[doc = "[`ListBoxActionItem`]의 정의"]
    ListBoxActionItemDef,
    #[doc = "[`ListBoxActionItem`]의 내부 데이터"]
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
    /// HTML 엘리먼트로부터 새로운 [`ListBoxActionItem`]을 만듭니다.
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            title: OnceCell::new(),
            text: OnceCell::new(),
        }
    }

    /// 제목을 반환합니다.
    pub fn title(&self) -> &str {
        self.title.get_or_init(|| {
            self.element_ref
                .value()
                .attr("title")
                .unwrap_or("")
                .to_owned()
        })
    }

    /// 내부 텍스트를 반환합니다.
    pub fn text(&self) -> &str {
        self.text
            .get_or_init(|| self.element_ref().text().collect::<String>())
    }
}
