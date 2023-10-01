use anyhow::Result;
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use rusaint::{application::USaintApplication, session::USaintSession};

pub(crate) struct EventTestSuite(USaintApplication);

impl Deref for EventTestSuite {
    type Target = USaintApplication;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for EventTestSuite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> EventTestSuite {
    const APP_NAME: &str = "WDR_TEST_EVENTS";

    pub async fn new(id: &str, token: &str) -> Result<EventTestSuite> {
        let session = USaintSession::with_password(id, token).await?;
        Ok(EventTestSuite(
            USaintApplication::with_session(Self::APP_NAME, Arc::new(session)).await?,
        ))
    }
}

mod button;
