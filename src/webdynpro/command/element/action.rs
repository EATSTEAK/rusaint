use crate::webdynpro::{
    client::EventProcessResult,
    command::WebDynproCommand,
    element::{action::Button, ElementDef}, error::WebDynproError,
};

/// 주어진 [`Button`]을 누름
pub struct ButtonPressCommand<'a> {
    element_def: ElementDef<'a, Button<'a>>,
}

impl<'a> ButtonPressCommand<'a> {
  /// 새로운 명령 객체를 생성합니다.
  pub fn new(element_def: ElementDef<'a, Button<'a>>) -> ButtonPressCommand<'a> {
    ButtonPressCommand {
      element_def
    }
  }
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
