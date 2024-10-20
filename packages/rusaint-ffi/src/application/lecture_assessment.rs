use std::sync::Arc;

use rusaint::{
    model::SemesterType,
};
use tokio::sync::RwLock;
use rusaint::application::lecture_assessment::model::LectureAssessmentResult;
use crate::{error::RusaintError, session::USaintSession};

#[derive(uniffi::Object)]
pub struct LectureAssessmentApplication(
    RwLock<rusaint::application::lecture_assessment::LectureAssessmentApplication>,
);

#[uniffi::export(async_runtime = "tokio")]
impl LectureAssessmentApplication {
    
    /// 학기, 학년도, 강의 분류를 통해 강의를 찾습니다.
    #[uniffi::method(default(lecture_name = None, lecture_code = None, professor_name = None))]
    pub async fn find_assessments(
        &self,
        year: u32,
        period: SemesterType,
        lecture_name: Option<String>,
        lecture_code: Option<u32>,
        professor_name: Option<String>
    ) -> Result<Vec<LectureAssessmentResult>, RusaintError> {
        let lecture_name = lecture_name.as_ref().map(String::as_str);
        let professor_name = professor_name.as_ref().map(String::as_str);
        Ok(self
            .0
            .write()
            .await
            .find_assessments(year, period, lecture_name, lecture_code, professor_name)
            .await?)
    }
}

#[derive(uniffi::Object)]
pub struct LectureAssessmentApplicationBuilder {}

#[uniffi::export(async_runtime = "tokio")]
impl LectureAssessmentApplicationBuilder {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {}
    }

    pub async fn build(
        &self,
        session: Arc<USaintSession>,
    ) -> Result<LectureAssessmentApplication, RusaintError> {
        let mut original_builder = rusaint::application::USaintClientBuilder::new();
        original_builder = original_builder.session(session.original());
        let original_app = original_builder
            .build_into::<rusaint::application::lecture_assessment::LectureAssessmentApplication>()
            .await?;
        Ok(LectureAssessmentApplication(RwLock::new(original_app)))
    }
}
