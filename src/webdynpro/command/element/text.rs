use crate::webdynpro::{
    command::{WebDynproCommand, WebDynproReadCommand},
    element::{definition::ElementDefinition, text::InputFieldDef},
    error::{ElementError, WebDynproError},
};

/// [`InputField`]의 값을 반환
pub struct ReadInputFieldValueCommand {
    element_def: InputFieldDef,
}

impl ReadInputFieldValueCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: InputFieldDef) -> ReadInputFieldValueCommand {
        Self { element_def }
    }
}

impl WebDynproReadCommand for ReadInputFieldValueCommand {
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
                content: "value of InputField".to_string(),
            })?;
        Ok(text)
    }
}

impl WebDynproCommand for ReadInputFieldValueCommand {
    type Result = String;

    async fn dispatch(
        &self,
        client: &mut crate::webdynpro::client::WebDynproClient,
    ) -> Result<Self::Result, WebDynproError> {
        self.read(client.body())
    }
}
