use indexmap::IndexMap;

use crate::webdynpro::event::{WDEvent, WDEventBuilder, ucf_parameters::{UcfParametersBuilder, UcfResponseData, UcfAction}};

use super::Component;

pub struct TabStrip<'a> {
    id: &'a str
}

impl<'a> Component<'a> for TabStrip<'a> {}

impl<'a> TabStrip<'a> {
    
    pub const fn new(id: &'a str) -> Self {
        Self {
            id
        }
    }

    pub fn tab_select(&self, item_id: &str, item_index: u32, first_visible_item_index: u32) -> WDEvent {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        let ucf_params = UcfParametersBuilder::default()
            .response(Some(UcfResponseData::Delta))
            .action(Some(UcfAction::Submit))
            .build()
            .unwrap();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("ItemId".to_string(), item_id.to_string());
        parameters.insert("ItemIndex".to_string(), item_index.to_string());
        parameters.insert("FirstVisibleItemIndex".to_string(), first_visible_item_index.to_string());
        WDEventBuilder::default()
            .control("TabStrip".to_owned())
            .event("TabSelect".to_owned())
            .parameters(parameters)
            .ucf_parameters(ucf_params)
            .build()
            .unwrap()
    }
}