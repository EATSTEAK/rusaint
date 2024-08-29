use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::event::Event;
use crate::webdynpro::{
    command::WebDynproCommand,
    element::{
        definition::ElementDefinition,
        selection::{
            list_box::{item::ListBoxItemInfo, ListBoxDefWrapper, ListBoxWrapper},
            ComboBoxDef, ComboBoxLSData,
        },
        Element,
    },
    error::{ElementError, WebDynproError},
};

/// [`ComboBox`](crate::webdynpro::element::selection::ComboBox)의 선택지를 선택하도록 함
pub struct ComboBoxSelectCommand {
    element_def: ComboBoxDef,
    key: String,
    by_enter: bool,
}

impl ComboBoxSelectCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: ComboBoxDef, key: &str, by_enter: bool) -> ComboBoxSelectCommand {
        Self {
            element_def,
            key: key.to_string(),
            by_enter,
        }
    }
}

impl WebDynproCommand for ComboBoxSelectCommand {
    type Result = Event;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        parser
            .element_from_def(&self.element_def)?
            .select(&self.key, self.by_enter)
    }
}

/// [`ComboBox`](crate::webdynpro::element::selection::ComboBox)의 내용을 바꿈
#[allow(unused)]
pub struct ComboBoxChangeCommand {
    element_def: ComboBoxDef,
    value: String,
    by_enter: bool,
}

impl ComboBoxChangeCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: ComboBoxDef, value: &str, by_enter: bool) -> ComboBoxChangeCommand {
        Self {
            element_def,
            value: value.to_string(),
            by_enter,
        }
    }
}

impl WebDynproCommand for ComboBoxChangeCommand {
    type Result = Event;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        parser
            .element_from_def(&self.element_def)?
            .change(&self.value)
    }
}

/// [`ComboBoxLSData`]를 반환
pub struct ReadComboBoxLSDataCommand {
    element_def: ComboBoxDef,
}

impl ReadComboBoxLSDataCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: ComboBoxDef) -> ReadComboBoxLSDataCommand {
        Self { element_def }
    }
}

impl WebDynproCommand for ReadComboBoxLSDataCommand {
    type Result = ComboBoxLSData;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        Ok(parser.element_from_def(&self.element_def)?.lsdata().clone())
    }
}

/// [`ComboBox`](crate::webdynpro::element::selection::ComboBox)의 참조 [`ListBoxDefWrapper`]를 반환
pub struct ReadComboBoxItemListBoxCommand {
    element_def: ComboBoxDef,
}

impl ReadComboBoxItemListBoxCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: ComboBoxDef) -> ReadComboBoxItemListBoxCommand {
        Self { element_def }
    }
}

impl WebDynproCommand for ReadComboBoxItemListBoxCommand {
    type Result = ListBoxDefWrapper;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        parser
            .element_from_def(&self.element_def)?
            .item_list_box(parser)
    }
}

/// [`ComboBox`](crate::webdynpro::element::selection::ComboBox)의 값을 반환
pub struct ReadComboBoxValueCommand {
    element_def: ComboBoxDef,
}

impl ReadComboBoxValueCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: ComboBoxDef) -> Self {
        Self { element_def }
    }
}

impl WebDynproCommand for ReadComboBoxValueCommand {
    type Result = String;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        let text = parser
            .element_from_def(&self.element_def)?
            .value()
            .map(str::to_string)
            .ok_or_else(|| ElementError::NoSuchContent {
                element: self.element_def.id().to_owned(),
                content: "value of ComboBox".to_string(),
            })?;
        Ok(text)
    }
}

/// [`ListBox`](crate::webdynpro::element::selection::list_box::ListBox)의 아이템 정보를 가져옴
pub struct ReadListBoxItemInfoCommand {
    element_def: ListBoxDefWrapper,
}

impl ReadListBoxItemInfoCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: ListBoxDefWrapper) -> Self {
        Self { element_def }
    }
}

impl WebDynproCommand for ReadListBoxItemInfoCommand {
    type Result = Vec<ListBoxItemInfo>;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        let element = self.element_def.value(&parser)?;
        match element {
            ListBoxWrapper::ListBoxPopup(list_box) => Ok(list_box
                .list_box()
                .item_infos(&parser)?
                .collect::<Vec<ListBoxItemInfo>>()),
            ListBoxWrapper::ListBoxPopupJson(list_box) => Ok(list_box
                .list_box()
                .item_infos(&parser)?
                .collect::<Vec<ListBoxItemInfo>>()),
            ListBoxWrapper::ListBoxPopupFiltered(list_box) => Ok(list_box
                .list_box()
                .item_infos(&parser)?
                .collect::<Vec<ListBoxItemInfo>>()),
            ListBoxWrapper::ListBoxPopupJsonFiltered(list_box) => Ok(list_box
                .list_box()
                .item_infos(&parser)?
                .collect::<Vec<ListBoxItemInfo>>()),
            ListBoxWrapper::ListBoxMultiple(list_box) => Ok(list_box
                .list_box()
                .item_infos(&parser)?
                .collect::<Vec<ListBoxItemInfo>>()),
            ListBoxWrapper::ListBoxSingle(list_box) => Ok(list_box
                .list_box()
                .item_infos(&parser)?
                .collect::<Vec<ListBoxItemInfo>>()),
        }
    }
}
