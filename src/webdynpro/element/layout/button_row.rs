use scraper::Selector;
use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::{action::Button, define_element_base, ElementWrapper, property::Visibility};

define_element_base! {
    ButtonRow<"BR", "ButtonRow"> {
        buttons: OnceCell<Vec<Button<'a>>>
    },
    ButtonRowLSData {
        visibility: Visibility => "0",
        custom_data: String => "1"
    }
}

impl<'a> ButtonRow<'a> {
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            buttons: OnceCell::new(),
        }
    }

    pub fn buttons(&'a self) -> &'a Vec<Button<'a>> {
        self.buttons.get_or_init(|| {
            let button_selector = &Selector::parse(r#":root [ct="B"]"#).unwrap();
            self.element_ref
                .select(button_selector)
                .filter_map(|elem| {
                    let elem = ElementWrapper::dyn_elem(elem);
                    match elem {
                        Ok(ElementWrapper::Button(button)) => Some(button),
                        _ => None,
                    }
                })
                .collect::<Vec<Button<'a>>>()
        })
    }
}
