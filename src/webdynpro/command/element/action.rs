use crate::webdynpro::{
    client::EventProcessResult,
    command::WebDynproCommand,
    element::{action::ButtonDef, definition::ElementDefinition},
    error::WebDynproError,
};

/// 주어진 [`Button`](crate::webdynpro::element::action::Button)을 누름
pub struct ButtonPressCommand {
    element_def: ButtonDef,
}

impl ButtonPressCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: ButtonDef) -> Self {
        Self { element_def }
    }
}

impl WebDynproCommand for ButtonPressCommand {
    type Result = EventProcessResult;

    async fn dispatch(
        &self,
        client: &mut crate::webdynpro::client::WebDynproClient,
    ) -> Result<Self::Result, WebDynproError> {
        let event = (&self.element_def).from_body(client.body())?.press()?;
        client.process_event(false, event).await
    }
}
