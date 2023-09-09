use indexmap::IndexMap;

use crate::webdynpro::event::{
    ucf_parameters::{UcfAction, UcfParametersBuilder, UcfResponseData},
    WDEvent, WDEventBuilder,
};

use super::Element;

pub struct CustomClientInfo {
    pub window_opener_exists: bool,
    pub client_url: String,
    pub client_width: u32,
    pub client_height: u32,
    pub document_domain: String,
    pub is_top_window: bool,
    pub parent_accessible: bool,
}

impl Default for CustomClientInfo {
    fn default() -> Self {
        Self { window_opener_exists: true, client_url: Default::default(), client_width: 1920, client_height: 1000, document_domain: Default::default(), is_top_window: true, parent_accessible: true }
    }
}

impl CustomClientInfo {
    pub fn new(window_opener_exists: bool, client_url: &str, client_width: u32, client_height: u32, document_domain: &str, is_top_window: bool, parent_accessible: bool) -> CustomClientInfo {
        CustomClientInfo {
            window_opener_exists,
            client_url: client_url.to_owned(),
            client_width,
            client_height,
            document_domain: document_domain.to_string(),
            is_top_window,
            parent_accessible
        }
    }
}

pub struct Custom<'a> {
    id: &'a str,
}

impl<'a> Element<'a> for Custom<'a> {
    // Note: This element is not rendered to client itself. This control id is a dummy.
    const CONTROL_ID: &'static str = "CUSTOM";
}

impl<'a> Custom<'a> {
    pub const fn new(id: &'a str) -> Self {
        Self { id }
    }

    pub fn client_infos(&self, infos: CustomClientInfo) -> WDEvent {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        let ucf_params = UcfParametersBuilder::default()
            .action(Some(UcfAction::Enqueue))
            .response(Some(UcfResponseData::Delta))
            .build()
            .unwrap();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert(
            "WindowOpenerExists".to_string(),
            infos.window_opener_exists.to_string(),
        );
        parameters.insert("ClientURL".to_string(), infos.client_url);
        parameters.insert("ClientWidth".to_string(), infos.client_width.to_string());
        parameters.insert("ClientHeight".to_string(), infos.client_height.to_string());
        parameters.insert("DocumentDomain".to_string(), infos.document_domain);
        parameters.insert("IsTopWindow".to_string(), infos.is_top_window.to_string());
        parameters.insert(
            "ParentAccessible".to_string(),
            infos.parent_accessible.to_string(),
        );
        WDEventBuilder::default()
            .control("Custom".to_owned())
            .event("ClientInfos".to_owned())
            .parameters(parameters)
            .ucf_parameters(ucf_params)
            .build()
            .unwrap()
    }
}