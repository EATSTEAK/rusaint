use std::{borrow::Cow, cell::OnceCell, collections::HashMap};

use crate::webdynpro::error::WebDynproError;
use crate::webdynpro::event::Event;

use crate::webdynpro::element::{define_element_interactable, Interactable};

// TODO: Implement additional events and data
define_element_interactable! {
    PopupWindow<"PW", "PopupWindow"> {},
    PopupWindowLSData {
        value: String => "0",
        show_help_button: bool => "1",
        // This field originally named "type"
        input_field_type: String => "2",
        visibility: String => "3",
        label_text: String => "4",
        hide_field_help: bool => "5",
        container_width_set: bool => "6",
        ime_mode: String => "7",
        auto_complete: bool => "8",
        format_string: String => "9",
        show_help_button_always: bool => "10",
        date_picker_start_ref_id: String => "11",
        access_key: String => "12",
        display_as_text: bool => "13",
        text_style: String => "14",
        spinner_increment: i32 => "15",
        spinner_bounds_check: bool => "16",
        spinner_max: i32 => "17",
        spinner_min: i32 => "18",
        sap_table_field_design: String => "19",
        validation_trigger: String => "20",
        tab_behaviour: String => "21",
        semantic_color: String => "22",
        embedding_behaviour: String => "23",
        field_help_floating: bool => "24",
        first_day_of_week: i32 => "25",
        custom_data: String => "26",
        custom_style: String => "27",
        field_help_embedding: bool => "28",
        labelled_by: String => "29",
        described_by: String => "30",
    }
}

impl<'a> PopupWindow<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }

    pub fn close(&self) -> Result<Event, WebDynproError> {
        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        self.fire_event("Close".to_string(), parameters)
    }

    pub fn help(&self) -> Result<Event, WebDynproError> {
        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        self.fire_event("Help".to_string(), parameters)
    }
}
