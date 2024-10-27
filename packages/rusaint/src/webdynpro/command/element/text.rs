use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::{
    command::WebDynproCommand,
    element::{definition::ElementDefinition, text::InputFieldDef},
    error::{ElementError, WebDynproError},
};

/// [`InputField`](crate::webdynpro::element::text::InputField)의 값을 반환
pub struct InputFieldValueCommand {
    element_def: InputFieldDef,
}

impl InputFieldValueCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: InputFieldDef) -> InputFieldValueCommand {
        Self { element_def }
    }
}

impl WebDynproCommand for InputFieldValueCommand {
    type Result = String;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        let text = parser
            .element_from_def(&self.element_def)?
            .value()
            .map(str::to_string)
            .ok_or_else(|| ElementError::NoSuchContent {
                element: self.element_def.id().to_owned(),
                content: "value of InputField".to_string(),
            })?;
        Ok(text)
    }
}
