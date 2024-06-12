use crate::webdynpro::{
    client::EventProcessResult,
    command::{WebDynproCommand, WebDynproReadCommand},
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
    type Result = EventProcessResult;

    async fn dispatch(
        &self,
        client: &mut crate::webdynpro::client::WebDynproClient,
    ) -> Result<Self::Result, WebDynproError> {
        let event = self
            .element_def
            .from_body(client.body())?
            .select(&self.key, self.by_enter)?;
        client.process_event(false, event).await
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
    type Result = EventProcessResult;

    async fn dispatch(
        &self,
        client: &mut crate::webdynpro::client::WebDynproClient,
    ) -> Result<Self::Result, WebDynproError> {
        let event = self
            .element_def
            .from_body(client.body())?
            .change(&self.value)?;
        client.process_event(false, event).await
    }
}

/// [`ComboBox`](crate::webdynpro::element::selection::ComboBox)의 선택지를 `value1`의 값을 기반으로 선택하도록 함
#[allow(unused)]
pub struct ComboBoxSelectByValue1Command {
    element_def: ComboBoxDef,
    value: String,
    by_enter: bool,
}

impl ComboBoxSelectByValue1Command {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(
        element_def: ComboBoxDef,
        value: &str,
        by_enter: bool,
    ) -> ComboBoxSelectByValue1Command {
        Self {
            element_def,
            value: value.to_string(),
            by_enter,
        }
    }
}

impl WebDynproCommand for ComboBoxSelectByValue1Command {
    type Result = EventProcessResult;

    async fn dispatch(
        &self,
        client: &mut crate::webdynpro::client::WebDynproClient,
    ) -> Result<Self::Result, WebDynproError> {
        let listbox_def = client.read(ReadComboBoxItemListBoxCommand::new(
            self.element_def.clone(),
        ))?;
        let items = client.read(ReadListBoxItemInfoCommand::new(listbox_def))?;
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
        client
            .send(ComboBoxSelectCommand::new(
                self.element_def.clone(),
                &item_key,
                false,
            ))
            .await
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

impl WebDynproReadCommand for ReadComboBoxLSDataCommand {
    fn read(
        &self,
        body: &crate::webdynpro::client::body::Body,
    ) -> Result<Self::Result, WebDynproError> {
        let lsdata = self.element_def.from_body(body)?.lsdata().clone();
        Ok(lsdata)
    }
}

impl WebDynproCommand for ReadComboBoxLSDataCommand {
    type Result = ComboBoxLSData;

    async fn dispatch(
        &self,
        client: &mut crate::webdynpro::client::WebDynproClient,
    ) -> Result<Self::Result, WebDynproError> {
        self.read(client.body())
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

impl WebDynproReadCommand for ReadComboBoxItemListBoxCommand {
    fn read(
        &self,
        body: &crate::webdynpro::client::body::Body,
    ) -> Result<Self::Result, WebDynproError> {
        let listbox_def = self.element_def.from_body(body)?.item_list_box(body)?;
        Ok(listbox_def)
    }
}

impl WebDynproCommand for ReadComboBoxItemListBoxCommand {
    type Result = ListBoxDefWrapper;

    async fn dispatch(
        &self,
        client: &mut crate::webdynpro::client::WebDynproClient,
    ) -> Result<Self::Result, WebDynproError> {
        self.read(client.body())
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

impl WebDynproReadCommand for ReadComboBoxValueCommand {
    fn read(
        &self,
        body: &crate::webdynpro::client::body::Body,
    ) -> Result<Self::Result, WebDynproError> {
        let text = self
            .element_def
            .from_body(body)?
            .value()
            .map(str::to_string)
            .ok_or_else(|| ElementError::NoSuchContent {
                element: self.element_def.id().to_owned(),
                content: "value of ComboBox".to_string(),
            })?;
        Ok(text)
    }
}

impl WebDynproCommand for ReadComboBoxValueCommand {
    type Result = String;

    async fn dispatch(
        &self,
        client: &mut crate::webdynpro::client::WebDynproClient,
    ) -> Result<Self::Result, WebDynproError> {
        self.read(client.body())
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

impl WebDynproReadCommand for ReadListBoxItemInfoCommand {
    fn read(
        &self,
        body: &crate::webdynpro::client::body::Body,
    ) -> Result<Self::Result, WebDynproError> {
        let element = self.element_def.from_body(body)?;
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

impl WebDynproCommand for ReadListBoxItemInfoCommand {
    type Result = Vec<ListBoxItemInfo>;

    async fn dispatch(
        &self,
        client: &mut crate::webdynpro::client::WebDynproClient,
    ) -> Result<Self::Result, WebDynproError> {
        self.read(client.body())
    }
}
