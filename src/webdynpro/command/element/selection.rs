use crate::webdynpro::{
    client::EventProcessResult,
    command::WebDynproCommand,
    element::{
        definition::ElementDefinition,
        selection::{ComboBoxDef, ComboBoxLSData},
        Element,
    },
    error::WebDynproError,
};

/// [`ComboBox`]의 선택지를 선택하도록 함
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

    async fn dispatch(
        &self,
        client: &mut crate::webdynpro::client::WebDynproClient,
    ) -> Result<Self::Result, WebDynproError> {
        let lsdata = self.element_def.from_body(client.body())?.lsdata().clone();
        Ok(lsdata)
    }
}
