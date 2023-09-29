use anyhow::Result;
use scraper::Selector;
use std::{borrow::Cow, cell::OnceCell};

use serde::Deserialize;

use super::{button::Button, Element, ElementDef, ElementWrapper, EventParameterMap};

#[derive(Debug)]
pub struct ButtonRow<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<ButtonRowLSData>>,
    buttons: OnceCell<Vec<Button<'a>>>,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct ButtonRowLSData {
    #[serde(rename = "0")]
    visibility: Option<String>,
    #[serde(rename = "1")]
    custom_data: Option<String>,
}

impl<'a> Element<'a> for ButtonRow<'a> {
    const CONTROL_ID: &'static str = "BR";

    const ELEMENT_NAME: &'static str = "ButtonRow";

    type ElementLSData = ButtonRowLSData;

    fn lsdata(&self) -> Option<&Self::ElementLSData> {
        self.lsdata
            .get_or_init(|| {
                let lsdata_obj = Self::lsdata_elem(self.element_ref).ok()?;
                serde_json::from_value::<Self::ElementLSData>(lsdata_obj).ok()
            })
            .as_ref()
    }

    fn lsevents(&self) -> Option<&EventParameterMap> {
        None
    }

    fn from_elem(elem_def: ElementDef<'a, Self>, element: scraper::ElementRef<'a>) -> Result<Self> {
        Ok(Self::new(elem_def.id.to_owned(), element))
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn element_ref(&self) -> &scraper::ElementRef<'a> {
        &self.element_ref
    }

    fn wrap(self) -> super::ElementWrapper<'a> {
        super::ElementWrapper::ButtonRow(self)
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
