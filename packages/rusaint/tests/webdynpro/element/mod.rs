use crate::get_session;
use lazy_static::lazy_static;
use rusaint::{
    RusaintError,
    application::{USaintApplication, USaintClient, USaintClientBuilder},
    webdynpro::{
        client::{EventProcessResult, body::Body},
        error::WebDynproError,
        event::Event,
    },
};
use std::sync::{Arc, OnceLock};
use tokio::sync::{Mutex, RwLock};

lazy_static! {
    static ref SUITE: Mutex<OnceLock<Arc<RwLock<EventTestSuite>>>> = Mutex::new(OnceLock::new());
}

#[derive(Debug)]
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

pub(crate) async fn get_event_test_suite() -> Result<Arc<RwLock<EventTestSuite>>, RusaintError> {
    let suite_lock = SUITE.lock().await;
    if let Some(suite) = suite_lock.get() {
        Ok(Arc::clone(suite))
    } else {
        let session = get_session().await.unwrap().clone();
        suite_lock
            .set(Arc::new(RwLock::new(
                USaintClientBuilder::new()
                    .session(session)
                    .build_into()
                    .await?,
            )))
            .unwrap();
        let suite = suite_lock.get().unwrap();
        Ok(Arc::clone(suite))
    }
}

mod button;
mod link;
