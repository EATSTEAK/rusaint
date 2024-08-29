use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::{
    action::Button, definition::ElementDefinition, macros::define_element_base,
    parser::ElementParser, property::Visibility, Element,
};

define_element_base! {
    #[doc = "[`Button`]의 나열"]
    ButtonRow<"BR", "ButtonRow"> {
        buttons: OnceCell<Vec<<Button<'a> as Element<'a>>::Def>>
    },
    #[doc = "[`ButtonRow`]의 정의"]
    ButtonRowDef,
    #[doc = "[`ButtonRow`] 내부 데이터"]
    ButtonRowLSData {
        visibility: Visibility => "0",
        custom_data: String => "1"
    }
}

impl<'a> ButtonRow<'a> {
    /// HTML 엘리먼트로부터 새로운 [`ButtonRow`] 엘리먼트를 생성합니다.
    pub fn new(id: Cow<'static, str>, tag: tl::HTMLTag<'a>) -> Self {
        Self {
            id,
            tag,
            lsdata: OnceCell::new(),
            buttons: OnceCell::new(),
        }
    }

    /// 내부 [`Button`]을 반환합니다.
    pub fn buttons(
        &'a self,
        parser: &ElementParser,
    ) -> impl Iterator<Item = &<Button<'a> as Element<'a>>::Def> + ExactSizeIterator {
        self.buttons
            .get_or_init(|| {
                self.tag
                    .query_selector(parser.dom().parser(), r#":root [ct="B"]"#)
                    .into_iter()
                    .flatten()
                    .filter_map(|handle| {
                        let id = handle
                            .get(parser.dom().parser())?
                            .as_tag()?
                            .attributes()
                            .id()?
                            .as_utf8_str()
                            .to_string();
                        Some(<Button<'a> as Element<'a>>::Def::new_dynamic(id))
                    })
                    .collect::<Vec<<Button<'a> as Element<'a>>::Def>>()
            })
            .iter()
    }
}
