use std::sync::Arc;

use crate::{error::RusaintError, session::USaintSession};
use rusaint::application::graduation_requirements::model::{
    GraduationRequirements, GraduationStudent,
};
use tokio::sync::RwLock;

/// [졸업사정표](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW8015)
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

    /// 졸업사정 결과와 졸업 필요 요건별 충족 여부와 세부 정보를 반환합니다.
    pub async fn requirements(&self) -> Result<GraduationRequirements, RusaintError> {
        Ok(self.0.write().await.requirements().await?)
    }
}

/// [`GraduationRequirementsApplication`] 생성을 위한 빌더
#[derive(uniffi::Object)]
pub struct GraduationRequirementsApplicationBuilder {}

#[uniffi::export(async_runtime = "tokio")]
impl GraduationRequirementsApplicationBuilder {
    /// 새로운 [`GraduationRequirementsApplicationBuilder`]를 만듭니다.
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {}
    }

    /// 세션과 함께 [`GraduationRequirementsApplication`]을 만듭니다.
    pub async fn build(
        &self,
        session: Arc<USaintSession>,
    ) -> Result<GraduationRequirementsApplication, RusaintError> {
        let original_builder =
            rusaint::application::USaintClientBuilder::new().session(session.original());
        let original_app = original_builder
            .build_into::<rusaint::application::graduation_requirements::GraduationRequirementsApplication>()
            .await?;
        Ok(GraduationRequirementsApplication(RwLock::new(original_app)))
    }
}

impl Default for GraduationRequirementsApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
