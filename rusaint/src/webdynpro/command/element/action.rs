use crate::webdynpro::command::WebDynproCommand;
use crate::webdynpro::event::Event;
use crate::webdynpro::{
    element::{action::ButtonDef, parser::ElementParser},
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
    type Result = Event;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        parser.element_from_def(&self.element_def)?.press()
    }
}
