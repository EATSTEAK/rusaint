use rusaint::{
    application::{USaintApplication, USaintClient},
    webdynpro::{
        client::{body::Body, EventProcessResult},
        error::WebDynproError,
        event::Event,
    },
    RusaintError,
};

pub(crate) struct EventTestSuite {
    client: USaintClient,
}

impl EventTestSuite {
    pub fn body(&self) -> &Body {
        self.client.body()
    }

    pub async fn process_event(
        &mut self,
        force_send: bool,
        event: Event,
    ) -> Result<EventProcessResult, WebDynproError> {
        self.client.process_event(force_send, event).await
    }
}

impl USaintApplication for EventTestSuite {
    const APP_NAME: &'static str = "WDR_TEST_EVENTS";

    fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
        if client.name() != Self::APP_NAME {
            Err(RusaintError::InvalidClientError)
        } else {
            Ok(EventTestSuite { client })
        }
    }
}

mod button;
mod link;
