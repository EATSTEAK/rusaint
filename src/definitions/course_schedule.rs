use std::ops::{Deref, DerefMut};

use crate::webdynpro::application::client::WDClientError;

use super::BasicUSaintApplication;

pub struct CourseSchedule(BasicUSaintApplication);

impl Deref for CourseSchedule {
    type Target = BasicUSaintApplication;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for CourseSchedule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CourseSchedule {
    const APP_NAME: &str = "ZCMW2100";

    pub async fn new() -> Result<CourseSchedule, WDClientError> {
        Ok(CourseSchedule(BasicUSaintApplication::new(Self::APP_NAME).await?))
    }
}