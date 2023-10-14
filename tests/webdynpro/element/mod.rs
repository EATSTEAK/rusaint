use std::sync::Arc;

use rusaint::{
    application::USaintApplication, define_usaint_application, webdynpro::error::WebDynproError,
    USaintSession,
};

define_usaint_application!(pub(crate) struct EventTestSuite);

impl<'a> EventTestSuite {
    const APP_NAME: &str = "WDR_TEST_EVENTS";

    pub async fn new(session: Arc<USaintSession>) -> Result<EventTestSuite, WebDynproError> {
        Ok(EventTestSuite(
            USaintApplication::with_session(Self::APP_NAME, session).await?,
        ))
    }
}

mod button;
