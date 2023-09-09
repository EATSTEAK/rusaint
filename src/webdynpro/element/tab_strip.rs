use indexmap::IndexMap;

use crate::webdynpro::{event::{WDEvent, WDEventBuilder, ucf_parameters::{UcfParametersBuilder, UcfResponseData, UcfAction}}, application::client::body::WDBody};

use super::{Element, Parseable, ElementParser};

pub struct TabStrip<'a> {
    id: &'a str
}

pub struct TabStripData {
    current_index: Option<i32>, // 0
    accessibility_description: Option<String>, // 3
    first_visible_item_idx: Option<i32>, // 5
    client_tab_select: Option<bool>, // 8
    tab_items_position: Option<String>, // 11
    heading_level: Option<i32> // 15
}

impl<'a> Element<'a> for TabStrip<'a> {
    // Note: This element renders as "TS_ie6" if >= IE6
    const CONTROL_ID: &'static str = "TS_standards";
}

impl<'a> Parseable<'a> for TabStrip<'a> {

    fn parser(&'a self, body: &'a WDBody) -> ElementParser<'a, TabStrip<'a>> {
        ElementParser {
            component: &self,
            id: &self.id,
            body
        }
    }
}

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

mod item {

    use crate::webdynpro::{application::client::body::WDBody, event::{ucf_parameters::{UcfParametersBuilder, UcfResponseData, UcfAction}, WDEventBuilder}, element::{Parseable, Element, ElementParser}};

    
    pub struct TabStripItem<'a> {
        id: &'a str
    }

    pub struct TabStripItemData {
        id: Option<String>, // 0
        index: Option<i32>, // 1
        caption: Option<String>, // 2
        tooltip: Option<String>, // 4
        enabled: Option<bool>, // 5
        is_closable: Option<bool>, // 9
        client_tab_select: Option<bool>, // 12
    }

    impl<'a> Element<'a> for TabStripItem<'a> {
        // Note: This element renders as "TSITM_ie6" if >= IE6
        const CONTROL_ID: &'static str = "TSITM_standards";
    }
    
    impl<'a> Parseable<'a> for TabStripItem<'a> {
    
        fn parser(&'a self, body: &'a WDBody) -> ElementParser<'a, TabStripItem<'a>> {
            ElementParser {
                component: &self,
                id: &self.id,
                body
            }
        }
    }
    
    impl<'a> TabStripItem<'a> {
        
        pub const fn new(id: &'a str) -> Self {
            Self {
                id
            }
        }
    }
}