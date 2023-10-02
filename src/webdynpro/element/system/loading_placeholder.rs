use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell, collections::HashMap};

use crate::webdynpro::event::Event;

use crate::webdynpro::element::{define_element_interactable, Interactable};

define_element_interactable! {
    LoadingPlaceholder<"LP", "LoadingPlaceHolder"> {},
    LoadingPlaceholderLSData {
        id: String => "0",
        custom_data: String => "1",
    }
}

impl<'a> LoadingPlaceholder<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }

    pub fn load(&self) -> Result<Event> {
        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        self.fire_event("Load".to_string(), parameters)
    }
}
