use anyhow::Result;
use std::ops::{Deref, DerefMut};

use super::BasicUSaintApplication;

pub struct StudentInformation(BasicUSaintApplication);

impl Deref for StudentInformation {
    type Target = BasicUSaintApplication;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for StudentInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl StudentInformation {
    const APP_NAME: &str = "ZCMW1001n";

    pub async fn new() -> Result<StudentInformation> {
        Ok(StudentInformation(
            BasicUSaintApplication::new(Self::APP_NAME).await?,
        ))
    }
}
