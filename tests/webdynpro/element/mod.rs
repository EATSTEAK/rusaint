use std::sync::Arc;

use rusaint::{application::USaintApplication, webdynpro::error::WebDynproError, USaintSession};

pub(crate) struct EventTestSuite(USaintApplication);

impl<'a> EventTestSuite {
    const APP_NAME: &str = "WDR_TEST_EVENTS";

    pub async fn new(session: Arc<USaintSession>) -> Result<EventTestSuite, WebDynproError> {
        Ok(EventTestSuite(
            USaintApplication::with_session(Self::APP_NAME, session).await?,
        ))
    }
}

mod button;
