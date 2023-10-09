use std::{borrow::Cow, cell::OnceCell, collections::HashMap};

use crate::webdynpro::{
    element::{define_element_interactable, Interactable},
    error::WebDynproError,
    event::Event,
};

define_element_interactable! {
    #[doc = "버튼"]
    Button<"B", "Button"> {},
    ButtonLSData {
        text: String => "0",
        text_design: String => "1",
        design: String => "2",
        width: String => "3",
        tooltip: String => "4",
        enabled: bool => "5",
        has_button_caption: bool => "6",
        visibility: String => "7",
        show_help: bool => "8",
        down: bool => "9",
        has_icon: bool => "10",
        disabled_icon_src: String => "11",
        up_icon_src: String => "12",
        down_icon_src: String => "13",
        has_popup_menu: bool => "14",
        popup_menu_id: String => "15",
        has_popup_menu_section: bool => "16",
        image_first: bool => "17",
        access_key: String => "18",
        hotkey: String => "19",
        up: bool => "20",
        text_overflow: bool => "21",
        fixed_height: bool => "22",
        button_type: String => "23", // This field originally named as "type"
        drag_source_info: String => "24",
        semantic_color: String => "25",
        interaction_behaviour: String => "26",
        custom_style: String => "27",
        custom_data: String => "28",
        wrapping: bool => "29",
        height: String => "30",
        content_visibility: String => "31"
    }
}

impl<'a> Button<'a> {
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }

    /// 버튼 누름 이벤트를 반환합니다.
    pub fn press(&self) -> Result<Event, WebDynproError> {
        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        self.fire_event("Press".to_string(), parameters)
    }
}
