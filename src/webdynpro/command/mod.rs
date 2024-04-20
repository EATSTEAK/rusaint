use super::{
    client::{EventProcessResult, WebDynproClient},
    error::WebDynproError,
};

pub trait WebDynproCommand {
    type Result: WebDynproCommandResult;

    async fn dispatch(&self, client: &mut WebDynproClient) -> Result<Self::Result, WebDynproError>;
}

pub trait WebDynproCommandResult {}

impl WebDynproCommandResult for EventProcessResult {}
