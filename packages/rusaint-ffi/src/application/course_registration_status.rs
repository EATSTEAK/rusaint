use std::sync::Arc;

use rusaint::{
    application::course_registration_status::model::RegisteredLecture, model::SemesterType,
};
use tokio::sync::RwLock;

use crate::application::model::YearSemester;
use crate::{error::RusaintError, session::USaintSession};

/// [수강신청 내역 조회](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW2110)
#[derive(uniffi::Object)]
pub struct CourseRegistrationStatusApplication(
    RwLock<rusaint::application::course_registration_status::CourseRegistrationStatusApplication>,
);

#[uniffi::export(async_runtime = "tokio")]
impl CourseRegistrationStatusApplication {
    /// 현재 페이지에 선택된 년도와 학기를 가져옵니다. 최초 로드 시 현재 학기를 가져올 가능성이 있습니다.
    /// 하지만 이 애플리케이션의 다른 함수를 호출하여 한번 정보를 가져왔다면 마지막으로 가져온 정보의 학기가 반환되므로 주의하여야 하며, 신뢰할 수 있는 현재 학기의 원천으로 사용되어서는 안됩니다.
    pub async fn get_selected_semester(&self) -> Result<YearSemester, RusaintError> {
        let (year, semester) = self.0.read().await.get_selected_semester()?;
        Ok(YearSemester::new(year, semester))
    }

    /// 개인이 수강신청한 내역을 학기별로 찾습니다.
    pub async fn lectures(
        &self,
        year: u32,
        semester: SemesterType,
    ) -> Result<Vec<RegisteredLecture>, RusaintError> {
        Ok(self
            .0
            .write()
            .await
            .lectures(year, semester)
            .await?
            .collect())
    }

    /// 페이지를 새로고침합니다.
    pub async fn reload(&self) -> Result<(), RusaintError> {
        Ok(self.0.write().await.reload().await?)
    }
}

/// [`CourseRegistrationStatusApplication`] 생성을 위한 빌더
#[derive(uniffi::Object)]
pub struct CourseRegistrationStatusApplicationBuilder {}

#[uniffi::export(async_runtime = "tokio")]
impl CourseRegistrationStatusApplicationBuilder {
    /// 새로운 [`CourseRegistrationStatusApplicationBuilder`]를 만듭니다.
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {}
    }

    /// 세션과 함께 [`CourseRegistrationStatusApplication`]을 만듭니다.
    pub async fn build(
        &self,
        session: Arc<USaintSession>,
    ) -> Result<CourseRegistrationStatusApplication, RusaintError> {
        let original_builder =
            rusaint::client::USaintClientBuilder::new().session(session.original());
        let original_app = original_builder
            .build_into::<rusaint::application::course_registration_status::CourseRegistrationStatusApplication>()
            .await?;
        Ok(CourseRegistrationStatusApplication(RwLock::new(
            original_app,
        )))
    }
}

impl Default for CourseRegistrationStatusApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
