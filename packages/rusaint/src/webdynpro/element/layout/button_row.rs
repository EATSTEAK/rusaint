use scraper::Selector;
use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::{
    Element, action::Button, definition::ElementDefinition, macros::define_element_base,
    property::Visibility,
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
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            buttons: OnceCell::new(),
        }
    }

    /// 내부 [`Button`]을 반환합니다.
    pub fn buttons(
        &'a self,
    ) -> impl ExactSizeIterator<Item = &'a <Button<'a> as Element<'a>>::Def> {
        self.buttons
            .get_or_init(|| {
                let button_selector = &Selector::parse(r#":root [ct="B"]"#).unwrap();
                self.element_ref
                    .select(button_selector)
                    .filter_map(|elem| <Button<'a> as Element<'a>>::Def::from_ref(elem).ok())
                    .collect::<Vec<<Button<'a> as Element<'a>>::Def>>()
            })
            .iter()
    }
}
