use anyhow::Result;
use std::{
    ops::{Deref, DerefMut},
    sync::RwLock,
};

use rusaint::application::BasicUSaintApplication;

pub(crate) struct EventTestSuite(BasicUSaintApplication);

impl Deref for EventTestSuite {
    type Target = BasicUSaintApplication;
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
        Ok(EventTestSuite(
            BasicUSaintApplication::with_auth(Self::APP_NAME, id, token).await?,
        ))
    }
}

mod button;
