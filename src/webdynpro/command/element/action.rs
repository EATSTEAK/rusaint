use crate::webdynpro::{
    client::EventProcessResult,
    command::WebDynproCommand,
    element::{action::Button, ElementDef}, error::WebDynproError,
};

pub struct ButtonPressCommand<'a> {
    element_def: ElementDef<'a, Button<'a>>,
}

impl<'a> WebDynproCommand for ButtonPressCommand<'a> {
    type Result = EventProcessResult;

    async fn dispatch(
        &self,
        client: &mut crate::webdynpro::client::WebDynproClient,
    ) -> Result<Self::Result, WebDynproError> {
        let event = (&self.element_def).from_body(client.body())?.press()?;
        client.process_event(false, event).await
    }
}
