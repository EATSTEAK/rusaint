use std::sync::Arc;

use rusaint::{
    application::course_schedule::model::{Lecture, LectureCategory},
    model::SemesterType,
};
use tokio::sync::RwLock;

use crate::application::model::YearSemester;
use crate::{error::RusaintError, session::USaintSession};

/// [강의시간표](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW2100)
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
        semester: SemesterType,
        lecture_category: &LectureCategory,
    ) -> Result<Vec<Lecture>, RusaintError> {
        Ok(self
            .0
            .write()
            .await
            .find_lectures(year, semester, lecture_category)
            .await?
            .collect())
    }

    /// 현재 페이지에 선택된 년도와 학기를 가져옵니다. 최초 로드 시 현재 학기를 가져올 가능성이 있습니다.
    /// 하지만 이 애플리케이션의 다른 함수를 호출하여 한번 정보를 가져왔다면 마지막으로 가져온 정보의 학기가 반환되므로 주의하여야 하며, 신뢰할 수 있는 현재 학기의 원천으로 사용되어서는 안됩니다.
    pub async fn get_selected_semester(&self) -> Result<YearSemester, RusaintError> {
        let (year, semester) = self.0.read().await.get_selected_semester()?;
        Ok(YearSemester::new(year, semester))
    }

    /// 선택한 학기 기준 단과대 목록을 가져옵니다.
    pub async fn collages(
        &self,
        year: u32,
        semester: SemesterType,
    ) -> Result<Vec<String>, RusaintError> {
        Ok(self.0.write().await.collages(year, semester).await?)
    }

    /// 선택한 학기 기준 주어진 단과대의 학과(부) 목록을 가져옵니다.
    pub async fn departments(
        &self,
        year: u32,
        semester: SemesterType,
        collage: &str,
    ) -> Result<Vec<String>, RusaintError> {
        Ok(self
            .0
            .write()
            .await
            .departments(year, semester, collage)
            .await?)
    }

    /// 선택한 학과(부)의 전공 목록을 가져옵니다.
    pub async fn majors(
        &self,
        year: u32,
        semester: SemesterType,
        collage: &str,
        department: &str,
    ) -> Result<Vec<String>, RusaintError> {
        Ok(self
            .0
            .write()
            .await
            .majors(year, semester, collage, department)
            .await?)
    }

    /// 선택한 학기의 교양필수 과목명 목록을 가져옵니다.
    pub async fn required_electives(
        &self,
        year: u32,
        semester: SemesterType,
    ) -> Result<Vec<String>, RusaintError> {
        Ok(self
            .0
            .write()
            .await
            .required_electives(year, semester)
            .await?)
    }

    /// 선택한 학기의 교양선택 분야 목록을 가져옵니다.
    pub async fn optional_elective_categories(
        &self,
        year: u32,
        semester: SemesterType,
    ) -> Result<Vec<String>, RusaintError> {
        Ok(self
            .0
            .write()
            .await
            .optional_elective_categories(year, semester)
            .await?)
    }

    /// 선택한 학기의 채플 과목 분류 목록을 가져옵니다.
    pub async fn chapel_categories(
        &self,
        year: u32,
        semester: SemesterType,
    ) -> Result<Vec<String>, RusaintError> {
        Ok(self
            .0
            .write()
            .await
            .chapel_categories(year, semester)
            .await?)
    }

    /// 선택한 학기의 대학원 단과대학 목록을 가져옵니다.
    pub async fn graduated_collages(
        &self,
        year: u32,
        semester: SemesterType,
    ) -> Result<Vec<String>, RusaintError> {
        Ok(self
            .0
            .write()
            .await
            .graduated_collages(year, semester)
            .await?)
    }

    /// 선택한 학기의 주어진 대학원 단과대의 학과 목록을 가져옵니다.
    pub async fn graduated_departments(
        &self,
        year: u32,
        semester: SemesterType,
        collage: &str,
    ) -> Result<Vec<String>, RusaintError> {
        Ok(self
            .0
            .write()
            .await
            .graduated_departments(year, semester, collage)
            .await?)
    }

    /// 선택한 학기의 연계전공 목록을 가져옵니다.
    pub async fn connected_majors(
        &self,
        year: u32,
        semester: SemesterType,
    ) -> Result<Vec<String>, RusaintError> {
        Ok(self
            .0
            .write()
            .await
            .connected_majors(year, semester)
            .await?)
    }

    /// 선택한 학기의 융합전공 목록을 가져옵니다.
    pub async fn united_majors(
        &self,
        year: u32,
        semester: SemesterType,
    ) -> Result<Vec<String>, RusaintError> {
        Ok(self.0.write().await.united_majors(year, semester).await?)
    }
}

/// [`CourseScheduleApplication`] 생성을 위한 빌더
#[derive(uniffi::Object)]
pub struct CourseScheduleApplicationBuilder {}

#[uniffi::export(async_runtime = "tokio")]
impl CourseScheduleApplicationBuilder {
    /// 새로운 [`CourseScheduleApplicationBuilder`]를 만듭니다.
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {}
    }

    /// 세션과 함께 [`CourseScheduleApplication`]을 만듭니다.
    pub async fn build(
        &self,
        session: Arc<USaintSession>,
    ) -> Result<CourseScheduleApplication, RusaintError> {
        let original_builder =
            rusaint::application::USaintClientBuilder::new().session(session.original());
        let original_app = original_builder
            .build_into::<rusaint::application::course_schedule::CourseScheduleApplication>()
            .await?;
        Ok(CourseScheduleApplication(RwLock::new(original_app)))
    }
}

impl Default for CourseScheduleApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
