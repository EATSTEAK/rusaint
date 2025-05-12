use crate::webdynpro::command::WebDynproCommandExecutor;
use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::event::Event;
use crate::webdynpro::{
    command::WebDynproCommand,
    element::{
        Element,
        definition::ElementDefinition,
        selection::{
            ComboBoxDef, ComboBoxLSData,
            list_box::{ListBoxDefWrapper, ListBoxWrapper, item::ListBoxItemInfo},
        },
    },
    error::{ElementError, WebDynproError},
};

/// [`ComboBox`](crate::webdynpro::element::selection::ComboBox)의 선택지를 선택하도록 하는 이벤트를 반환
pub struct ComboBoxSelectEventCommand {
    element_def: ComboBoxDef,
    key: String,
    by_enter: bool,
}

impl ComboBoxSelectEventCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: ComboBoxDef, key: &str, by_enter: bool) -> ComboBoxSelectEventCommand {
        Self {
            element_def,
            key: key.to_string(),
            by_enter,
        }
    }
}

impl WebDynproCommand for ComboBoxSelectEventCommand {
    type Result = Event;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        parser
            .element_from_def(&self.element_def)?
            .select(&self.key, self.by_enter)
    }
}

/// [`ComboBox`](crate::webdynpro::element::selection::ComboBox)의 내용을 바꾸는 이벤트를 반환
#[allow(unused)]
pub struct ComboBoxChangeEventCommand {
    element_def: ComboBoxDef,
    value: String,
    by_enter: bool,
}

impl ComboBoxChangeEventCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(
        element_def: ComboBoxDef,
        value: &str,
        by_enter: bool,
    ) -> ComboBoxChangeEventCommand {
        Self {
            element_def,
            value: value.to_string(),
            by_enter,
        }
    }
}

impl WebDynproCommand for ComboBoxChangeEventCommand {
    type Result = Event;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        parser
            .element_from_def(&self.element_def)?
            .change(&self.value)
    }
}

/// [`ComboBox`](crate::webdynpro::element::selection::ComboBox)의 선택지를 `value1`의 값을 기반으로 선택하도록 하는 이벤트를 반환
#[allow(unused)]
pub struct ComboBoxSelectByValue1EventCommand {
    element_def: ComboBoxDef,
    value: String,
    by_enter: bool,
}

impl ComboBoxSelectByValue1EventCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(
        element_def: ComboBoxDef,
        value: &str,
        by_enter: bool,
    ) -> ComboBoxSelectByValue1EventCommand {
        Self {
            element_def,
            value: value.to_string(),
            by_enter,
        }
    }
}

impl WebDynproCommand for ComboBoxSelectByValue1EventCommand {
    type Result = Event;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        let listbox_def = parser.read(ComboBoxItemListBoxCommand::new(self.element_def.clone()))?;
        let items = parser.read(ListBoxItemInfoCommand::new(listbox_def))?;
        let item_key = items
            .iter()
            .find_map(|info| match info {
                ListBoxItemInfo::Item { value1, key, .. } => {
                    if value1 == &self.value {
                        Some(key)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .ok_or(ElementError::InvalidContent {
                element: self.element_def.id().to_string(),
                content: format!("Cannot find {} option", self.value),
            })?
            .to_owned();
        parser.read(ComboBoxSelectEventCommand::new(
            self.element_def.clone(),
            &item_key,
            false,
        ))
    }
}

/// [`ComboBoxLSData`]를 반환
pub struct ComboBoxLSDataCommand {
    element_def: ComboBoxDef,
}

impl ComboBoxLSDataCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: ComboBoxDef) -> ComboBoxLSDataCommand {
        Self { element_def }
    }
}

impl WebDynproCommand for ComboBoxLSDataCommand {
    type Result = ComboBoxLSData;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        let lsdata = parser.element_from_def(&self.element_def)?.lsdata().clone();
        Ok(lsdata)
    }
}

/// [`ComboBox`](crate::webdynpro::element::selection::ComboBox)의 참조 [`ListBoxDefWrapper`]를 반환
pub struct ComboBoxItemListBoxCommand {
    element_def: ComboBoxDef,
}

impl ComboBoxItemListBoxCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: ComboBoxDef) -> ComboBoxItemListBoxCommand {
        Self { element_def }
    }
}

impl WebDynproCommand for ComboBoxItemListBoxCommand {
    type Result = ListBoxDefWrapper;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        let listbox_def = parser
            .element_from_def(&self.element_def)?
            .item_list_box(parser)?;
        Ok(listbox_def)
    }
}

/// [`ComboBox`](crate::webdynpro::element::selection::ComboBox)의 값을 반환
pub struct ComboBoxValueCommand {
    element_def: ComboBoxDef,
}

impl ComboBoxValueCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: ComboBoxDef) -> Self {
        Self { element_def }
    }
}

impl WebDynproCommand for ComboBoxValueCommand {
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
pub struct ListBoxItemInfoCommand {
    element_def: ListBoxDefWrapper,
}

impl ListBoxItemInfoCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: ListBoxDefWrapper) -> Self {
        Self { element_def }
    }
}

impl WebDynproCommand for ListBoxItemInfoCommand {
    type Result = Vec<ListBoxItemInfo>;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        let element = &self.element_def.value(parser)?;
        match element {
            ListBoxWrapper::ListBoxPopup(list_box) => Ok(list_box
                .list_box()
                .item_infos()?
                .collect::<Vec<ListBoxItemInfo>>()),
            ListBoxWrapper::ListBoxPopupJson(list_box) => Ok(list_box
                .list_box()
                .item_infos()?
                .collect::<Vec<ListBoxItemInfo>>()),
            ListBoxWrapper::ListBoxPopupFiltered(list_box) => Ok(list_box
                .list_box()
                .item_infos()?
                .collect::<Vec<ListBoxItemInfo>>()),
            ListBoxWrapper::ListBoxPopupJsonFiltered(list_box) => Ok(list_box
                .list_box()
                .item_infos()?
                .collect::<Vec<ListBoxItemInfo>>()),
            ListBoxWrapper::ListBoxMultiple(list_box) => Ok(list_box
                .list_box()
                .item_infos()?
                .collect::<Vec<ListBoxItemInfo>>()),
            ListBoxWrapper::ListBoxSingle(list_box) => Ok(list_box
                .list_box()
                .item_infos()?
                .collect::<Vec<ListBoxItemInfo>>()),
        }
    }
}
