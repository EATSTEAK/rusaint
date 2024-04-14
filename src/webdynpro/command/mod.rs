use super::{client::WebDynproClient, error::WebDynproError};

pub(crate) trait WebDynproCommand {
    type Result: WebDynproCommandResult;

    async fn dispatch(&self, client: &mut WebDynproClient) -> Result<Self::Result, WebDynproError>;
}

pub(crate) trait WebDynproCommandResult {}
