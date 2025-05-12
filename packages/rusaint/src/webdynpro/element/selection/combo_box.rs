use std::{borrow::Cow, cell::OnceCell, collections::HashMap};

use crate::webdynpro::element::property::{
    EmbeddingBehaviour, IMEMode, InputFieldTextStyle, InputFieldType, SemanticColor,
    SuggestFilterCondition, SuggestFilterType, TabBehaviour, TableFieldDesign, Visibility,
};
use crate::webdynpro::error::{BodyError, WebDynproError};
use crate::webdynpro::{error::ElementError, event::Event};

use self::property::ComboBoxBehavior;
use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::element::{
    Element, ElementDefWrapper, Interactable, macros::define_element_interactable,
};

use super::list_box::ListBoxDefWrapper;

pub mod property {
    use serde::Deserialize;

    #[allow(missing_docs)]
    #[derive(Clone, Deserialize, Debug)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum ComboBoxBehavior {
        DropdownSelect,
        FreeText,
        SuggestList,
        SuggestFieldHelp,
    }
}

define_element_interactable! {
    #[doc = "목록 혹은 직접 입력하여 선택할 수 있는 콤보 박스"]
    ComboBox<"CB", "ComboBox"> {},
    #[doc = "[`ComboBox`]의 정의"]
    ComboBoxDef,
    #[doc = "[`ComboBox`] 내부 데이터"]
    ComboBoxLSData {
        width: String => "0",
        behavior: ComboBoxBehavior => "1",
        allow_virtual_typing: String => "2",
        item_list_box_id: String => "3",
        key: String => "4",
        value: String => "5",
        visibility: Visibility => "6",
        container_width_set: bool => "7",
        label_text: String => "8",
        label_for: String => "9",
        ime_mode: IMEMode => "10",
        component_type: InputFieldType => "11", // originally "type"
        show_help_button_always: String => "12",
        access_key: String => "13",
        suggest_filter: SuggestFilterType => "14",
        display_as_text: bool => "15",
        hide_field_help: bool => "16",
        show_help_button: bool => "17",
        suggest_auto_complete: bool => "18",
        suggest_filter_condition: SuggestFilterCondition => "19",
        field_help_floating: bool => "20",
        custom_data: String => "21",
        custom_style: String => "22",
        text_style: InputFieldTextStyle => "23",
        semantic_color: SemanticColor => "24",
        embedding_behaviour: EmbeddingBehaviour => "25",
        sap_table_field_design: TableFieldDesign => "26",
        field_help_embedding: bool => "27",
        height: String => "28",
        labelled_by: String => "29",
        tab_behaviour: TabBehaviour => "30",
        described_by: String => "31",
    }
}

impl<'a> ComboBox<'a> {
    /// HTML 엘리먼트로부터 새로운 [`ComboBox`] 엘리먼트를 생성합니다.
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }

    /// [`ComboBox`]의 선택지 역할을 하는 [`ListBox`](super::list_box::ListBox) 엘리먼트를 가져옵니다.
    pub fn item_list_box(
        &self,
        parser: &ElementParser,
    ) -> Result<ListBoxDefWrapper, WebDynproError> {
        let listbox_id = self
            .lsdata()
            .item_list_box_id()
            .ok_or(ElementError::NoSuchData {
                element: self.id().to_string(),
                field: "item_list_box_id".to_string(),
            })?;
        let selector = scraper::Selector::parse(format!(r#"[id="{listbox_id}"]"#).as_str())
            .or(Err(ElementError::InvalidId(listbox_id.to_owned())))?;
        let elem = parser
            .document()
            .select(&selector)
            .next()
            .ok_or(BodyError::NoSuchElement(listbox_id.to_owned()))?;
        Ok(
            ListBoxDefWrapper::from_def(ElementDefWrapper::from_ref(elem)?)
                .ok_or(BodyError::NoSuchElement(listbox_id.to_owned()))?,
        )
    }

    /// 선택지를 선택하는 이벤트를 반환합니다. `by_enter`가 참일 경우 엔터를 눌러 선택한 것으로 취급합니다.
    pub fn select(&self, key: &str, by_enter: bool) -> Result<Event, WebDynproError> {
        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("Key".to_string(), key.to_string());
        parameters.insert("ByEnter".to_string(), by_enter.to_string());
        self.fire_event("Select".to_string(), parameters)
    }

    /// 내용을 변경하는 이벤트를 반환합니다.
    pub fn change(&self, value: &str) -> Result<Event, WebDynproError> {
        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("Value".to_string(), value.to_string());
        self.fire_event("Change".to_string(), parameters)
    }

    /// 이 [`ComboBox`]의 값을 가져옵니다.
    pub fn value(&self) -> Option<&str> {
        self.element_ref.attr("value")
    }
}
