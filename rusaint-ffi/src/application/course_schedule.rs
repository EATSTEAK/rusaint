use std::sync::Arc;

use rusaint::{
    application::course_schedule::model::{Lecture, LectureCategory},
    model::SemesterType,
};
use tokio::sync::RwLock;

use crate::{error::RusaintError, session::USaintSession};

#[derive(uniffi::Object)]
pub struct CourseScheduleApplication(
    RwLock<rusaint::application::course_schedule::CourseScheduleApplication>,
);

#[uniffi::export(async_runtime = "tokio")]
impl CourseScheduleApplication {
    /// 학기, 학년도, 강의 분류를 통해 강의를 찾습니다.
    pub async fn find_lectures(
        &self,
        year: u32,
        period: SemesterType,
        lecture_category: &LectureCategory,
    ) -> Result<Vec<Lecture>, RusaintError> {
        Ok(self
            .0
            .write()
            .await
            .find_lectures(year, period, lecture_category)
            .await?
            .collect())
    }
}

#[derive(uniffi::Object)]
pub struct CourseScheduleApplicationBuilder {}

#[uniffi::export(async_runtime = "tokio")]
impl CourseScheduleApplicationBuilder {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {}
    }

    pub async fn build(
        &self,
        session: Arc<USaintSession>,
    ) -> Result<CourseScheduleApplication, RusaintError> {
        let mut original_builder = rusaint::application::USaintClientBuilder::new();
        original_builder = original_builder.session(session.original());
        let original_app = original_builder
            .build_into::<rusaint::application::course_schedule::CourseScheduleApplication>()
            .await?;
        Ok(CourseScheduleApplication(RwLock::new(original_app)))
    }
}
