use std::sync::Arc;

use rusaint::{
    model::SemesterType,
};
use tokio::sync::RwLock;
use rusaint::application::personal_course_schedule::model::PersonalCourseSchedule;
use crate::{error::RusaintError, session::USaintSession};
use crate::application::chapel::{ChapelApplication, ChapelApplicationBuilder};

/// [개인수업시간표](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW2102)
#[derive(uniffi::Object)]
pub struct PersonalCourseScheduleApplication(
    RwLock<rusaint::application::personal_course_schedule::PersonalCourseScheduleApplication>,
);

#[uniffi::export(async_runtime = "tokio")]
impl PersonalCourseScheduleApplication {

    /// 해당 학기의 시간표 정보를 가져옵니다.
    pub async fn schedule(
        &self,
        year: u32,
        semester: SemesterType,
    ) -> Result<PersonalCourseSchedule, RusaintError> {
        Ok(self.0.write().await.schedule(year, semester).await?)
    }
}

/// [`PersonalCourseScheduleApplication`] 생성을 위한 빌더
#[derive(uniffi::Object)]
pub struct PersonalCourseScheduleApplicationBuilder {}

#[uniffi::export(async_runtime = "tokio")]
impl PersonalCourseScheduleApplicationBuilder {

    /// 새로운 [`PersonalCourseScheduleApplicationBuilder`]를 만듭니다.
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {}
    }

    /// 세션과 함께 [`PersonalCourseScheduleApplication`]을 만듭니다.
    pub async fn build(
        &self,
        session: Arc<USaintSession>,
    ) -> Result<PersonalCourseScheduleApplication, RusaintError> {
        let mut original_builder = rusaint::application::USaintClientBuilder::new();
        original_builder = original_builder.session(session.original());
        let original_app = original_builder
            .build_into::<rusaint::application::personal_course_schedule::PersonalCourseScheduleApplication>()
            .await?;
        Ok(PersonalCourseScheduleApplication(RwLock::new(original_app)))
    }
}
