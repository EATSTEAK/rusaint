use crate::webdynpro::{command::WebDynproCommand, element::{complex::{sap_table::SapTableBody, SapTableDef}, definition::ElementDefinition}, error::WebDynproError};

/// [`SapTableBody`]를 반환
pub struct ReadSapTableBodyCommand {
    element_def: SapTableDef,
}

impl ReadSapTableBodyCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: SapTableDef) -> ReadSapTableBodyCommand {
        Self { element_def }
    }
}

impl WebDynproCommand for ReadSapTableBodyCommand {
    type Result = SapTableBody;

    async fn dispatch(
        &self,
        client: &mut crate::webdynpro::client::WebDynproClient,
    ) -> Result<Self::Result, WebDynproError> {
        let body = self.element_def.from_body(client.body())?.table()?.clone();
        Ok(body)
    }
}