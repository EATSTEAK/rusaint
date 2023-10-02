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

    pub async fn new(session: Arc<USaintSession>) -> Result<EventTestSuite> {
        Ok(EventTestSuite(
            USaintApplication::with_session(Self::APP_NAME, session).await?,
        ))
    }
}

mod button;
