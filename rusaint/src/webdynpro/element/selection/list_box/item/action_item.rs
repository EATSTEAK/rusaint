use crate::webdynpro::element::{macros::define_element_base, parser::ElementParser, Element};
use std::{borrow::Cow, cell::OnceCell};
use tl::Bytes;

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
    pub fn new(id: Cow<'static, str>, tag: tl::HTMLTag<'a>) -> Self {
        Self {
            id,
            tag,
            lsdata: OnceCell::new(),
            title: OnceCell::new(),
            text: OnceCell::new(),
        }
    }

    /// 제목을 반환합니다.
    pub fn title(&self) -> &str {
        self.title.get_or_init(|| {
            self.tag
                .attributes()
                .get("title")
                .flatten()
                .and_then(Bytes::try_as_utf8_str)
                .unwrap_or("")
                .to_owned()
        })
    }

    /// 내부 텍스트를 반환합니다.
    pub fn text(&self, parser: &ElementParser) -> &str {
        self.text
            .get_or_init(|| self.tag().inner_text(parser.dom().parser()).to_string())
    }
}
