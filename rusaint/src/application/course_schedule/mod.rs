use crate::{
    application::course_schedule::model::{Lecture, LectureCategory},
    define_elements,
    model::SemesterType,
    webdynpro::{
        client::body::Body,
        command::element::{complex::ReadSapTableBodyCommand, selection::ComboBoxSelectCommand},
        element::{
            complex::{
                sap_table::cell::{SapTableCell, SapTableCellWrapper},
                SapTable,
            },
            definition::ElementDefinition,
            layout::TabStrip,
            selection::ComboBox,
            ElementDefWrapper,
        },
        error::WebDynproError,
    },
    ApplicationError, RusaintError,
};

use super::{USaintApplication, USaintClient};

/// [강의시간표](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW2100)
pub struct CourseScheduleApplication {
    client: USaintClient,
}

impl USaintApplication for CourseScheduleApplication {
    const APP_NAME: &'static str = "ZCMW2100";

    fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
        if client.name() != Self::APP_NAME {
            Err(RusaintError::InvalidClientError)
        } else {
            Ok(Self { client })
        }
    }
}

#[allow(unused)]
impl<'a> CourseScheduleApplication {
    // 메인 요소
    define_elements! {
        PERIOD_YEAR: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERYR";
        PERIOD_ID: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERID";
        TABLE_ROWS: ComboBox<'a> = "ZCMW2100.ID_0001:VIW_MODULES.ROWS";
        TABSTRIP: TabStrip<'a> = "ZCMW2100.ID_0001:VIW_MAIN.MODULE_TABSTRIP";
        MAIN_TABLE: SapTable<'a> = "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.SALV_WD_UIE_TABLE";
    }

    fn semester_to_key(period: SemesterType) -> &'static str {
        match period {
            SemesterType::One => "090",
            SemesterType::Summer => "091",
            SemesterType::Two => "092",
            SemesterType::Winter => "093",
        }
    }

    async fn select_period(
        &mut self,
        year: &str,
        period: SemesterType,
    ) -> Result<(), WebDynproError> {
        self.client
            .send(ComboBoxSelectCommand::new(Self::PERIOD_YEAR, year, false))
            .await?;
        self.client
            .send(ComboBoxSelectCommand::new(
                Self::PERIOD_ID,
                Self::semester_to_key(period),
                false,
            ))
            .await?;
        Ok(())
    }

    async fn select_rows(&mut self, row: u32) -> Result<(), WebDynproError> {
        self.client
            .send(ComboBoxSelectCommand::new(
                Self::TABLE_ROWS,
                row.to_string().as_str(),
                false,
            ))
            .await?;
        Ok(())
    }

    fn body(&self) -> &Body {
        self.client.body()
    }

    /// 학기, 학년도, 강의 분류를 통해 강의를 찾습니다.
    pub async fn find_lectures(
        &mut self,
        year: u32,
        period: SemesterType,
        lecture_category: &LectureCategory,
    ) -> Result<impl Iterator<Item = Lecture>, RusaintError> {
        let year_str = format!("{}", year);
        self.select_rows(500).await?;
        self.select_period(&year_str, period).await?;
        lecture_category.request_query(&mut self.client.0).await?;
        let table = self
            .client
            .read(ReadSapTableBodyCommand::new(Self::MAIN_TABLE))?;
        let Some(first_row) = table.iter().next() else {
            return Err(ApplicationError::NoLectureResult.into());
        };
        if let Some(Ok(SapTableCellWrapper::Normal(cell))) =
            first_row.iter_value(self.body()).next()
        {
            if let Some(ElementDefWrapper::TextView(tv_def)) = cell.content() {
                if let Ok(tv) = tv_def.from_body(self.body()) {
                    if tv.text().contains("없습니다.") {
                        return Err(ApplicationError::NoLectureResult.into());
                    }
                }
            }
        }
        let lectures = table.try_table_into::<Lecture>(self.client.body())?;
        Ok(lectures.into_iter())
    }

    // TO-DO: 카테고리 별 선택지 가져오기 기능
}

#[cfg(test)]
mod test {}

/// 강의시간표 애플리케이션에서 사용하는 데이터 모델
pub mod model;
