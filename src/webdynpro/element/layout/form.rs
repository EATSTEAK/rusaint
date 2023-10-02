use anyhow::Result;
use std::{borrow::Cow, cell::OnceCell, collections::HashMap};

use crate::webdynpro::event::Event;

use crate::webdynpro::element::{define_element_interactable, Interactable};

define_element_interactable! {
    Form<"FOR", "Form"> {
        data: OnceCell<FormData>
    },
    FormLSData {
        has_event_queue: bool => "0",
        response_data: String => "1",
        custom_data: String => "2",
    }
}

#[derive(Debug, Default)]
#[allow(unused)]
pub struct FormData {
    name: Option<String>,
    method: Option<String>,
    action: Option<String>,
    title: Option<String>,
    accept: Option<String>,
    accept_charset: Option<String>,
    enctype: Option<String>,
    target: Option<String>,
}

impl<'a> Form<'a> {
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
            data: OnceCell::new(),
        }
    }

    pub fn request(
        &self,
        is_async: bool,
        focus_info: &str,
        hash: &str,
        dom_changed: bool,
        is_dirty: bool,
    ) -> Result<Event> {
        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("Async".to_string(), is_async.to_string());
        parameters.insert("FocusInfo".to_string(), focus_info.to_string());
        parameters.insert("Hash".to_string(), hash.to_string());
        parameters.insert("DomChanged".to_string(), dom_changed.to_string());
        parameters.insert("IsDirty".to_string(), is_dirty.to_string());
        self.fire_event("Request".to_string(), parameters)
    }
}
