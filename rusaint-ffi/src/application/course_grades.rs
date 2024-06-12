use std::sync::{Arc, RwLock};

use crate::{error::RusaintError, session::USaintSession};

#[derive(uniffi::Object)]
pub struct CourseGradesApplication(
    RwLock<rusaint::application::course_grades::CourseGradesApplication>,
);

unsafe impl Send for CourseGradesApplication {}

unsafe impl Sync for CourseGradesApplication {}

#[derive(uniffi::Object)]
pub struct CourseGradesApplicationBuilder {}

#[uniffi::export]
impl CourseGradesApplicationBuilder {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {}
    }

    pub async fn build_with_password(&self) -> Result<CourseGradesApplication, RusaintError> {
        let mut original_builder = rusaint::application::USaintClientBuilder::new();
        if let Some(session_base) = &self.session {
            let session = session_base.construct().await?;
            original_builder = original_builder.session(Arc::new(session));
        }
        let original_app = original_builder
            .build_into::<rusaint::application::course_grades::CourseGradesApplication>()
            .await?;
        Ok(CourseGradesApplication(RwLock::new(original_app)))
    }
}
