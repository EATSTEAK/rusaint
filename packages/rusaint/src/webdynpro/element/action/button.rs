use std::{borrow::Cow, cell::OnceCell, collections::HashMap};

use crate::webdynpro::{
    element::{
        Interactable,
        macros::define_element_interactable,
        property::{ContentVisibility, HotkeyValue, SemanticColor, TextDesign, Visibility},
    },
    error::WebDynproError,
    event::Event,
};

use super::{ButtonDesign, ButtonInteractionBehaviour, ButtonType};

pub mod property {
    use serde::Deserialize;

    /// 버튼의 외형 종류
    #[allow(missing_docs)]
    #[derive(Clone, Deserialize, Debug)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum ButtonDesign {
        Emphasized,
        Standard,
        Previous,
        Next,
        Transparent,
        Accept,
        Reject,
        Toggle,
    }

    /// 버튼의 동작 분류
    #[allow(missing_docs)]
    #[derive(Clone, Deserialize, Debug)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum ButtonType {
        None,
        Menu,
        Help,
        Personalize,
        Close,
        ExpandAll,
        CollapseAll,
        ScrollTop,
        Minimize,
        Maximize,
        Restore,
        CollapseBegin,
        CollapseEnd,
        ExpandBegin,
        ExpandEnd,
        Back,
        Forward,
        VariantManagement,
        Rte,
    }

    /// 버튼의 상호작용 동작
    #[allow(missing_docs)]
    #[derive(Clone, Deserialize, Debug)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum ButtonInteractionBehaviour {
        Push,
        Toggle,
    }
}

define_element_interactable! {
    #[doc = "누를 수 있는 버튼"]
    Button<"B", "Button"> {},
    #[doc = "[`Button`]의 정의"]
    ButtonDef,
    #[doc = "[`Button`]의 내부 데이터"]
    ButtonLSData {
        text: String => "0",
        text_design: TextDesign => "1",
        design: ButtonDesign => "2",
        width: String => "3",
        tooltip: String => "4",
        enabled: bool => "5",
        has_button_caption: bool => "6",
        visibility: Visibility => "7",
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
        hotkey: HotkeyValue => "19",
        up: bool => "20",
        text_overflow: bool => "21",
        fixed_height: bool => "22",
        button_type: ButtonType => "23", // This field originally named as "type"
        drag_source_info: String => "24",
        semantic_color: SemanticColor => "25",
        interaction_behaviour: ButtonInteractionBehaviour => "26",
        custom_style: String => "27",
        custom_data: String => "28",
        wrapping: bool => "29",
        height: String => "30",
        content_visibility: ContentVisibility => "31"
    }
}

impl<'a> Button<'a> {
    /// HTML 엘리먼트로부터 새로운 [`Button`] 엘리먼트를 생성합니다.
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
