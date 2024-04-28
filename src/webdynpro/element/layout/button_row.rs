use scraper::Selector;
use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::{
    action::Button, define_element_base, definition::ElementDef, property::Visibility
};

define_element_base! {
    #[doc = "[`Button`]의 나열"]
    ButtonRow<"BR", "ButtonRow"> {
        buttons: OnceCell<Vec<ElementDef<'a, Button<'a>>>>
    },
    #[doc = "[`ButtonRow`] 내부 데이터"]
    ButtonRowLSData {
        visibility: Visibility => "0",
        custom_data: String => "1"
    }
}

impl<'a> ButtonRow<'a> {
    /// HTML 엘리먼트로부터 새로운 [`ButtonRow`] 엘리먼트를 생성합니다.
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            buttons: OnceCell::new(),
        }
    }

    /// 내부 [`Button`]을 반환합니다.
    pub fn buttons(&'a self) -> impl Iterator<Item = &ElementDef<'a, Button<'a>>> + ExactSizeIterator {
        self.buttons
            .get_or_init(|| {
                let button_selector = &Selector::parse(r#":root [ct="B"]"#).unwrap();
                self.element_ref
                    .select(button_selector)
                    .filter_map(|elem| {
                        let def = ElementDef::from_element_ref(elem);
                        match def {
                            Ok(button_def) => Some(button_def),
                            _ => None
                        }
                    })
                    .collect::<Vec<ElementDef<'a, Button<'a>>>>()
            })
            .iter()
    }
}
