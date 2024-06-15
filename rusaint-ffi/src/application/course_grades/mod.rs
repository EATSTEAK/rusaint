use std::sync::Arc;

use model::{CourseType, GradeSummary};
use tokio::sync::RwLock;

use crate::{error::RusaintError, session::USaintSession};

#[derive(uniffi::Object)]
pub struct CourseGradesApplication(RwLock<rusaint::application::course_grades::CourseGradesApplication>);


#[uniffi::export]
impl CourseGradesApplication {
    pub async fn recorded_summary(
        &self,
        course_type: CourseType,
    ) -> Result<GradeSummary, RusaintError> {
        Ok(self.0.write().await.recorded_summary(course_type.into()).await?.into())
    }
}

#[derive(uniffi::Object)]
pub struct CourseGradesApplicationBuilder {}

#[uniffi::export]
impl CourseGradesApplicationBuilder {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {}
    }

    pub async fn build(
        &self,
        session: Arc<USaintSession>,
    ) -> Result<CourseGradesApplication, RusaintError> {
        let mut original_builder = rusaint::application::USaintClientBuilder::new();
        original_builder = original_builder.session(session.original());
        let original_app = original_builder
            .build_into::<rusaint::application::course_grades::CourseGradesApplication>()
            .await?;
        Ok(CourseGradesApplication(RwLock::new(original_app)))
    }
}

pub mod model;
