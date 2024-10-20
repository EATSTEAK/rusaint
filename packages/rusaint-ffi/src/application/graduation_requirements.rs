use std::sync::Arc;

use crate::{error::RusaintError, session::USaintSession};
use rusaint::application::graduation_requirements::model::GraduationStudent;
use tokio::sync::RwLock;

#[derive(uniffi::Object)]
pub struct GraduationRequirementsApplication(
    RwLock<rusaint::application::graduation_requirements::GraduationRequirementsApplication>,
);

#[uniffi::export(async_runtime = "tokio")]
impl GraduationRequirementsApplication {
    /// 학생 정보를 반환합니다.
    pub async fn student_info(&self) -> Result<GraduationStudent, RusaintError> {
        Ok(self.0.read().await.student_info().await?)
    }
}

#[derive(uniffi::Object)]
pub struct GraduationRequirementsApplicationBuilder {}

#[uniffi::export(async_runtime = "tokio")]
impl GraduationRequirementsApplicationBuilder {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {}
    }

    pub async fn build(
        &self,
        session: Arc<USaintSession>,
    ) -> Result<GraduationRequirementsApplication, RusaintError> {
        let mut original_builder = rusaint::application::USaintClientBuilder::new();
        original_builder = original_builder.session(session.original());
        let original_app = original_builder
            .build_into::<rusaint::application::graduation_requirements::GraduationRequirementsApplication>()
            .await?;
        Ok(GraduationRequirementsApplication(RwLock::new(original_app)))
    }
}
