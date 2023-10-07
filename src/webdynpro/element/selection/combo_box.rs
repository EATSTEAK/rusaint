use std::{borrow::Cow, cell::OnceCell, collections::HashMap};

use crate::webdynpro::error::{BodyError, WebDynproError};
use crate::webdynpro::{application::client::body::Body, error::ElementError, event::Event};

use crate::webdynpro::element::{
    define_element_interactable, Element, ElementWrapper, Interactable,
};

use super::list_box::ListBoxWrapper;

define_element_interactable! {
    ComboBox<"CB", "ComboBox"> {},
    ComboBoxLSData {
        behavior: String => "0",
        allow_virtual_typing: String => "1",
        item_list_box_id: String => "2",
        key: String => "3",
        value: String => "4",
        visibility: String => "5",
        container_width_set: bool => "6",
        label_text: String => "7",
        label_for: String => "8",
        ime_mode: String => "9",
        component_type: String => "10", // originally "type"
        show_help_button_always: String => "11",
        access_key: String => "12",
        suggest_filter: String => "13",
        display_as_text: bool => "14",
        hide_field_help: bool => "15",
        show_help_button: bool => "16",
        suggest_auto_complete: bool => "17",
        suggest_filter_condition: String => "18",
        field_help_floating: bool => "19",
        custom_data: String => "20",
        custom_style: String => "21",
        text_style: String => "22",
        semantic_color: String => "23",
        embedding_behaviour: String => "24",
        sap_table_field_design: String => "25",
        field_help_embedding: bool => "26",
        labelled_by: String => "27",
        tab_behaviour: String => "28",
        described_by: String => "29",
    }
}

impl<'a> ComboBox<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }

    pub fn item_list_box(&self, body: &'a Body) -> Result<ListBoxWrapper<'a>, WebDynproError> {
        let listbox_id = self
            .lsdata()
            .and_then(|lsdata| lsdata.item_list_box_id.as_ref())
            .ok_or(ElementError::NoSuchData {
                element: self.id().to_string(),
                field: "item_list_box_id".to_string(),
            })?;
        let selector = scraper::Selector::parse(format!(r#"[id="{}"]"#, listbox_id).as_str())
            .or(Err(ElementError::InvalidId(listbox_id.to_owned())))?;
        let elem = body
            .document()
            .select(&selector)
            .next()
            .ok_or(BodyError::NoSuchElement(listbox_id.to_owned()))?;
        Ok(
            ListBoxWrapper::from_elements(ElementWrapper::dyn_elem(elem)?)
                .ok_or(BodyError::NoSuchElement(listbox_id.to_owned()))?,
        )
    }

    pub fn select(&self, key: &str, by_enter: bool) -> Result<Event, WebDynproError> {
        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("Key".to_string(), key.to_string());
        parameters.insert("ByEnter".to_string(), by_enter.to_string());
        self.fire_event("Select".to_string(), parameters)
    }
}
