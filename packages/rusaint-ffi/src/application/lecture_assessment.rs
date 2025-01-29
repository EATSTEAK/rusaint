use std::sync::Arc;

use crate::{error::RusaintError, session::USaintSession};
use rusaint::application::lecture_assessment::model::LectureAssessmentResult;
use rusaint::model::SemesterType;
use tokio::sync::RwLock;

/// [강의평가조회](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMB2W1010)
#[derive(uniffi::Object)]
pub struct LectureAssessmentApplication(
    RwLock<rusaint::application::lecture_assessment::LectureAssessmentApplication>,
);

#[uniffi::export(async_runtime = "tokio")]
impl LectureAssessmentApplication {
    /// 검색 조건에 맞는 강의평가 정보를 가져옵니다.
    #[uniffi::method(default(lecture_name = None, lecture_code = None, professor_name = None))]
    pub async fn find_assessments(
        &self,
        year: u32,
        semester: SemesterType,
        lecture_name: Option<String>,
        lecture_code: Option<u32>,
        professor_name: Option<String>,
    ) -> Result<Vec<LectureAssessmentResult>, RusaintError> {
        let lecture_name = lecture_name.as_deref();
        let professor_name = professor_name.as_deref();
        Ok(self
            .0
            .write()
            .await
            .find_assessments(year, semester, lecture_name, lecture_code, professor_name)
            .await?)
    }
}

/// [`LectureAssessmentApplication`] 생성을 위한 빌더
#[derive(uniffi::Object)]
pub struct LectureAssessmentApplicationBuilder {}

#[uniffi::export(async_runtime = "tokio")]
impl LectureAssessmentApplicationBuilder {
    /// 새로운 [`LectureAssessmentApplicationBuilder`]를 만듭니다.
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {}
    }

    /// 세션과 함께 [`LectureAssessmentApplication`]을 만듭니다.
    pub async fn build(
        &self,
        session: Arc<USaintSession>,
    ) -> Result<LectureAssessmentApplication, RusaintError> {
        let original_builder =
            rusaint::application::USaintClientBuilder::new().session(session.original());
        let original_app = original_builder
            .build_into::<rusaint::application::lecture_assessment::LectureAssessmentApplication>()
            .await?;
        Ok(LectureAssessmentApplication(RwLock::new(original_app)))
    }
}

impl Default for LectureAssessmentApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
