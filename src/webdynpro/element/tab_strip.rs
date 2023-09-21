use indexmap::IndexMap;
use serde::Deserialize;

use crate::webdynpro::{event::Event, application::client::body::Body, error::{BodyError, ElementError}};

use super::{Element, EventParameterMap, ElementDef};

pub struct TabStrip<'a> {
    id: &'a str,
    lsdata: Option<TabStripLSData>,
    lsevents: Option<EventParameterMap>
}

#[derive(Deserialize, Debug, Default)]
pub struct TabStripLSData {
    #[serde(rename = "0")]
    current_index: Option<i32>,
    #[serde(rename = "1")]
    height: Option<String>,
    #[serde(rename = "2")]
    width: Option<String>,
    #[serde(rename = "3")]
    accessibility_description: Option<String>,
    #[serde(rename = "4")]
    visibility: Option<String>,
    #[serde(rename = "5")]
    first_visible_item_idx: Option<i32>,
    #[serde(rename = "6")]
    scrollable: Option<bool>,
    #[serde(rename = "7")]
    exact_tab_alignment: Option<bool>,
    #[serde(rename = "8")]
    client_tab_select: Option<bool>,
    #[serde(rename = "9")]
    drag_source_info: Option<String>,
    #[serde(rename = "10")]
    drop_target_info: Option<String>,
    #[serde(rename = "11")]
    tab_items_position: Option<String>,
    #[serde(rename = "12")]
    custom_data: Option<String>,
    #[serde(rename = "13")]
    custom_style: Option<String>,
    #[serde(rename = "14")]
    tab_items_design: Option<String>,
    #[serde(rename = "15")]
    heading_level: Option<i32>
}

impl<'a> Element for TabStrip<'a> {
    // Note: This element renders as "TS_ie6" if >= IE6
    const CONTROL_ID: &'static str = "TS_standards";

    const ELEMENT_NAME: &'static str = "TabStrip";

    type ElementLSData = TabStripLSData;

    fn lsdata(&self) -> Option<&Self::ElementLSData> {
        self.lsdata.as_ref()
    }

    fn lsevents(&self) -> Option<&EventParameterMap> {
        self.lsevents.as_ref()
    }
}

impl<'a> ElementDef<'a, TabStrip<'a>> {
    pub fn elem(&self, body: &'_ Body) -> Result<TabStrip<'a>, BodyError> {
        TabStrip::from_body(self, body)
    }
}

impl<'a> TabStrip<'a> {
    
    pub const fn new(id: &'a str, lsdata: Option<TabStripLSData>, lsevents: Option<EventParameterMap>) -> Self {
        Self {
            id,
            lsdata,
            lsevents
        }
    }

    pub fn from_body(elem_def: &ElementDef<'a, Self>, body: &'_ Body) -> Result<Self, BodyError> {
        let selector = &elem_def.selector().or(Err(BodyError::InvalidSelector))?;
        let lsdata_obj = Self::lsdata_elem(selector, body.document())?;
        let lsdata = serde_json::from_value::<TabStripLSData>(lsdata_obj).or(Err(ElementError::InvalidLSData))?;
        let lsevents = Self::lsevents_elem(selector, body.document())?;
        Ok(Self::new(elem_def.id, Some(lsdata), Some(lsevents)))
    }

    pub fn tab_select(&self, item_id: &str, item_index: u32, first_visible_item_index: u32) -> Result<Event, ElementError> {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("ItemId".to_string(), item_id.to_string());
        parameters.insert("ItemIndex".to_string(), item_index.to_string());
        parameters.insert("FirstVisibleItemIndex".to_string(), first_visible_item_index.to_string());
        self.fire_event("TabSelect", parameters)
    }
}

mod item {

    use serde::Deserialize;

    use crate::webdynpro::{element::{Element, EventParameterMap, ElementDef}, application::client::body::Body, error::{BodyError, ElementError}};

    
    pub struct TabStripItem<'a> {
        id: &'a str,
        lsdata: Option<TabStripItemLSData>
    }

    #[derive(Deserialize, Debug, Default)]
    pub struct TabStripItemLSData {
        #[serde(rename = "0")]
        id: Option<String>,
        #[serde(rename = "1")]
        index: Option<i32>,
        #[serde(rename = "2")]
        caption: Option<String>,
        #[serde(rename = "3")]
        has_title_caption: Option<bool>,
        #[serde(rename = "4")]
        tooltip: Option<String>,
        #[serde(rename = "5")]
        enabled: Option<bool>,
        #[serde(rename = "6")]
        scrolling_mode: Option<String>,
        #[serde(rename = "7")]
        has_toolbar: Option<bool>,
        #[serde(rename = "8")]
        default_button_id: Option<String>,
        #[serde(rename = "9")]
        is_closable: Option<bool>,
        #[serde(rename = "10")]
        scroll_top: Option<i32>,
        #[serde(rename = "11")]
        scroll_left: Option<i32>,
        #[serde(rename = "12")]
        client_tab_select: Option<bool>,
        #[serde(rename = "13")]
        hotkeys_id: Option<String>,
        #[serde(rename = "14")]
        access_key: Option<String>,
        #[serde(rename = "15")]
        has_editable_title: Option<bool>,
        #[serde(rename = "16")]
        area_design: Option<String>,
        #[serde(rename = "17")]
        custom_data: Option<String>,
        #[serde(rename = "18")]
        custom_style: Option<String>,
        #[serde(rename = "19")]
        visibility: Option<String>,
    }

    impl<'a> Element for TabStripItem<'a> {
        // Note: This element renders as "TSITM_ie6" if >= IE6
        const CONTROL_ID: &'static str = "TSITM_standards";

        // Unused
        const ELEMENT_NAME: &'static str = "TabStripTab";

        type ElementLSData = TabStripItemLSData;

        fn lsdata(&self) -> Option<&Self::ElementLSData> {
            self.lsdata.as_ref()
        }

        fn lsevents(&self) -> Option<&EventParameterMap> {
            None
        }
    }

    impl<'a> ElementDef<'a, TabStripItem<'a>> {
        pub fn elem(&self, body: &'_ Body) -> Result<TabStripItem<'a>, BodyError> {
            TabStripItem::from_body(self, body)
        }
    }
    
    impl<'a> TabStripItem<'a> {
        
        pub const fn new(id: &'a str, lsdata: Option<TabStripItemLSData>) -> Self {
            Self {
                id,
                lsdata
            }
        }

        pub fn from_body(elem_def: &ElementDef<'a, Self>, body: &'_ Body) -> Result<Self, BodyError> {
            let selector = &elem_def.selector().or(Err(BodyError::InvalidSelector))?;
            let lsdata_obj = Self::lsdata_elem(selector, body.document())?;
            let lsdata = serde_json::from_value::<TabStripItemLSData>(lsdata_obj).or(Err(ElementError::InvalidLSData))?;
            Ok(Self::new(elem_def.id, Some(lsdata)))
        }
    }
}