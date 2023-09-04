use indexmap::IndexMap;

use crate::webdynpro::event::{WDEvent, WDEventBuilder, ucf_parameters::{UcfParametersBuilder, UcfResponseData}};

use super::Element;

pub struct Form<'a> {
    id: &'a str
}

impl<'a> Element<'a> for Form<'a> {}

impl<'a> Form<'a> {
    
    pub const fn new(id: &'a str) -> Self {
        Self {
            id
        }
    }

    pub fn request(&self, is_async: bool, focus_info: &str, hash: &str, dom_changed: bool, is_dirty: bool) -> WDEvent {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        let ucf_params = UcfParametersBuilder::default()
            .response(Some(UcfResponseData::Delta))
            .build()
            .unwrap();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("Async".to_string(), is_async.to_string());
        parameters.insert("FocusInfo".to_string(), focus_info.to_string());
        parameters.insert("Hash".to_string(), hash.to_string());
        parameters.insert("DomChanged".to_string(), dom_changed.to_string());
        parameters.insert("IsDirty".to_string(), is_dirty.to_string());
        WDEventBuilder::default()
            .control("Form".to_owned())
            .event("Request".to_owned())
            .parameters(parameters)
            .ucf_parameters(ucf_params)
            .build()
            .unwrap()
    }
}