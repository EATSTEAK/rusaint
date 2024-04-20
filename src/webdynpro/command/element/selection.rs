use crate::webdynpro::{
    client::EventProcessResult,
    command::WebDynproCommand,
    element::{
        selection::{ComboBox, ComboBoxLSData},
        Element, ElementDef,
    },
    error::WebDynproError,
};

pub struct ComboBoxSelectCommand<'a> {
    element_def: ElementDef<'a, ComboBox<'a>>,
    key: String,
    by_enter: bool,
}

impl<'a> ComboBoxSelectCommand<'a> {
  pub fn new(element_def: ElementDef<'a, ComboBox<'a>>, key: &str, by_enter: bool) -> ComboBoxSelectCommand<'a> {
    Self {
      element_def,
      key: key.to_string(),
      by_enter
    }
  }
}

impl<'a> WebDynproCommand for ComboBoxSelectCommand<'a> {
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

pub struct ReadComboBoxLSDataCommand<'a> {
    element_def: ElementDef<'a, ComboBox<'a>>,
}

impl<'a> ReadComboBoxLSDataCommand<'a> {
    pub fn new(element_def: ElementDef<'a, ComboBox<'a>>) -> ReadComboBoxLSDataCommand<'a> {
        Self { element_def }
    }
}

impl<'a> WebDynproCommand for ReadComboBoxLSDataCommand<'a> {
    type Result = ComboBoxLSData;

    async fn dispatch(
        &self,
        client: &mut crate::webdynpro::client::WebDynproClient,
    ) -> Result<Self::Result, WebDynproError> {
        let lsdata = self.element_def.from_body(client.body())?.lsdata().clone();
        Ok(lsdata)
    }
}
