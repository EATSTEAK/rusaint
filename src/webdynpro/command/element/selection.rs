use crate::webdynpro::{client::EventProcessResult, command::WebDynproCommand, element::{selection::ComboBox, ElementDef}, error::WebDynproError};


pub struct ComboBoxSelectCommand<'a> {
  element_def: ElementDef<'a, ComboBox<'a>>,
  key: String,
  by_enter: bool
}

impl<'a> WebDynproCommand for ComboBoxSelectCommand<'a> {
  type Result = EventProcessResult;

  async fn dispatch(&self, client: &mut crate::webdynpro::client::WebDynproClient) -> Result<Self::Result, WebDynproError> {
      let event = self.element_def.from_body(client.body())?.select(&self.key, self.by_enter)?;
      client.process_event(false, event).await
  }
}