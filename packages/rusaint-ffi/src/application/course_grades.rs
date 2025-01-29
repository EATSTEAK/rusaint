use std::{collections::HashMap, sync::Arc};

use rusaint::{
    application::course_grades::model::{ClassGrade, CourseType, GradeSummary, SemesterGrade},
    model::SemesterType,
};
use tokio::sync::RwLock;

use crate::application::model::YearSemester;
use crate::{error::RusaintError, session::USaintSession};

/// [학생 성적 조회](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMB3W0017)
#[derive(uniffi::Object)]
pub struct CourseGradesApplication(
    RwLock<rusaint::application::course_grades::CourseGradesApplication>,
);

#[uniffi::export(async_runtime = "tokio")]
impl CourseGradesApplication {
    /// 전체 학기의 학적부 평점 정보를 가져옵니다.
    pub async fn recorded_summary(
        &self,
        course_type: CourseType,
    ) -> Result<GradeSummary, RusaintError> {
        Ok(self.0.write().await.recorded_summary(course_type).await?)
    }

    /// 전체 학기의 증명 평점 정보를 가져옵니다.
    pub async fn certificated_summary(
        &self,
        course_type: CourseType,
    ) -> Result<GradeSummary, RusaintError> {
        Ok(self
            .0
            .write()
            .await
            .certificated_summary(course_type)
            .await?)
    }

    /// 학기별 평점 정보를 가져옵니다.
    pub async fn semesters(
        &self,
        course_type: CourseType,
    ) -> Result<Vec<SemesterGrade>, RusaintError> {
        Ok(self.0.write().await.semesters(course_type).await?)
    }

    /// 주어진 학기의 수업별 성적을 가져옵니다. `include_details`가 `true`인 경우 수업의 상세 성적도 가져옵니다.
    /// 수업의 상세 성적까지 가져올 경우 상세 성적이 있는 수업의 수 만큼 서버에 요청을 보내므로 반드시 상세 성적도 한번에 가져와야 할 때에만 사용하십시오.
    ///
    /// 수업 성적을 가져온 이후 상세 성적 또한 가져오려면 `[class_detail()]`함수를 이용하십시오.
    pub async fn classes(
        &self,
        course_type: CourseType,
        year: u32,
        semester: SemesterType,
        include_details: bool,
    ) -> Result<Vec<ClassGrade>, RusaintError> {
        Ok(self
            .0
            .write()
            .await
            .classes(course_type, year, semester, include_details)
            .await?)
    }

    /// 주어진 수업의 상세 성적 정보를 가져옵니다.
    pub async fn class_detail(
        &self,
        course_type: CourseType,
        year: u32,
        semester: SemesterType,
        code: &str,
    ) -> Result<HashMap<String, f32>, RusaintError> {
        Ok(self
            .0
            .write()
            .await
            .class_detail(course_type, year, semester, code)
            .await?)
    }

    /// 현재 페이지에 선택된 년도와 학기를 가져옵니다. 최초 로드 시 현재 학기를 가져올 가능성이 있습니다.
    /// 하지만 이 애플리케이션의 다른 함수를 호출하여 한번 정보를 가져왔다면 마지막으로 가져온 정보의 학기가 반환되므로 주의하여야 하며, 신뢰할 수 있는 현재 학기의 원천으로 사용되어서는 안됩니다.
    pub async fn get_selected_semester(&self) -> Result<YearSemester, RusaintError> {
        let (year, semester) = self.0.read().await.get_selected_semester()?;
        Ok(YearSemester::new(year, semester))
    }
}

/// [`CourseGradesApplication`] 생성을 위한 빌더
#[derive(uniffi::Object)]
pub struct CourseGradesApplicationBuilder {}

#[uniffi::export(async_runtime = "tokio")]
impl CourseGradesApplicationBuilder {
    /// 새로운 [`CourseGradesApplicationBuilder`]를 만듭니다.
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {}
    }

    /// 세션과 함께 [`CourseGradesApplication`]을 만듭니다.
    pub async fn build(
        &self,
        session: Arc<USaintSession>,
    ) -> Result<CourseGradesApplication, RusaintError> {
        let original_builder =
            rusaint::application::USaintClientBuilder::new().session(session.original());
        let original_app = original_builder
            .build_into::<rusaint::application::course_grades::CourseGradesApplication>()
            .await?;
        Ok(CourseGradesApplication(RwLock::new(original_app)))
    }
}

impl Default for CourseGradesApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
