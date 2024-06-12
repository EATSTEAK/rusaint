use model::LectureAssessmentResult;

use crate::{
    define_elements,
    model::SemesterType,
    webdynpro::{
        client::body::Body,
        command::element::{
            action::ButtonPressCommand,
            complex::{
                ReadSapTableBodyCommand, ReadSapTableLSDataCommand, SapTableVerticalScrollCommand,
            },
            selection::{ComboBoxChangeCommand, ComboBoxSelectCommand, ReadComboBoxLSDataCommand},
        },
        element::{
            action::Button,
            complex::{
                sap_table::cell::{SapTableCell, SapTableCellWrapper},
                SapTable,
            },
            definition::ElementDefinition,
            selection::ComboBox,
            ElementDefWrapper,
        },
        error::{ElementError, WebDynproError},
    },
    ApplicationError, RusaintError,
};

use super::{USaintApplication, USaintClient};

/// [강의평가조회](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMB2W1010)
pub struct LectureAssessmentApplication {
    client: USaintClient,
}

impl USaintApplication for LectureAssessmentApplication {
    const APP_NAME: &'static str = "ZCMB2W1010";

    fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
        if client.name() != Self::APP_NAME {
            Err(RusaintError::InvalidClientError)
        } else {
            Ok(Self { client })
        }
    }
}

impl<'a> LectureAssessmentApplication {
    define_elements! {
        DDLB_01: ComboBox<'a> = "ZCMB2W1010.ID_0001:MAIN.DDLB_01";
        DDLB_02: ComboBox<'a> = "ZCMB2W1010.ID_0001:MAIN.DDLB_02";
        IF_01: ComboBox<'a> = "ZCMB2W1010.ID_0001:MAIN.IF_01";
        ILSM_OBJID: ComboBox<'a> = "WDR_SELECT_OPTIONS.ID_15B7446540DB284588CCE6BAC0049040:SELECTION_SCREEN.ILSM_OBJID";
        IF_04: ComboBox<'a> = "ZCMB2W1010.ID_0001:MAIN.IF_04";
        BT_SEARCH: Button<'a> = "ZCMB2W1010.ID_0001:MAIN.BT_SEARCH";
        TABLE: SapTable<'a> = "ZCMB2W1010.ID_0001:MAIN.TABLE";
    }

    fn semester_to_key(period: SemesterType) -> &'static str {
        match period {
            SemesterType::One => "090",
            SemesterType::Summer => "091",
            SemesterType::Two => "092",
            SemesterType::Winter => "0923",
        }
    }

    fn body(&self) -> &Body {
        self.client.body()
    }

    async fn search(
        &mut self,
        year: &str,
        semester: SemesterType,
        lecture_name: Option<&str>,
        lecture_code: Option<u32>,
        professor_name: Option<&str>,
    ) -> Result<(), WebDynproError> {
        let semester = Self::semester_to_key(semester);
        let year_combobox_lsdata = self
            .client
            .read(ReadComboBoxLSDataCommand::new(Self::DDLB_01))?;
        let semester_combobox_lsdata = self
            .client
            .read(ReadComboBoxLSDataCommand::new(Self::DDLB_02))?;
        if (|| Some(year_combobox_lsdata.key()?.as_str()))() != Some(year) {
            self.client
                .send(ComboBoxSelectCommand::new(Self::DDLB_01, &year, false))
                .await?;
        }
        if (|| Some(semester_combobox_lsdata.key()?.as_str()))() != Some(semester) {
            self.client
                .send(ComboBoxSelectCommand::new(Self::DDLB_02, semester, false))
                .await?;
        }
        if let Some(lecture_name) = lecture_name {
            self.client
                .send(ComboBoxChangeCommand::new(Self::IF_01, lecture_name, false))
                .await?;
        }
        if let Some(lecture_code) = lecture_code {
            self.client
                .send(ComboBoxChangeCommand::new(
                    Self::ILSM_OBJID,
                    &lecture_code.to_string(),
                    false,
                ))
                .await?;
        }
        if let Some(professor_name) = professor_name {
            self.client
                .send(ComboBoxChangeCommand::new(
                    Self::IF_04,
                    professor_name,
                    false,
                ))
                .await?;
        }
        self.client
            .send(ButtonPressCommand::new(Self::BT_SEARCH))
            .await?;
        Ok(())
    }

    /// 검색 조건에 맞는 강의평가 정보를 가져옵니다.
    pub async fn find_assessments(
        &mut self,
        year: u32,
        semester: SemesterType,
        lecture_name: Option<&str>,
        lecture_code: Option<u32>,
        professor_name: Option<&str>,
    ) -> Result<Vec<LectureAssessmentResult>, RusaintError> {
        self.search(
            &year.to_string(),
            semester,
            lecture_name,
            lecture_code,
            professor_name,
        )
        .await?;
        let row_count = self
            .body()
            .read(ReadSapTableLSDataCommand::new(Self::TABLE))?
            .row_count()
            .map(|u| u.to_owned())
            .ok_or_else(|| {
                WebDynproError::Element(ElementError::NoSuchData {
                    element: Self::TABLE.id().to_string(),
                    field: "row_count".to_string(),
                })
            })?
            .try_into()
            .unwrap();
        let mut table = self
            .body()
            .read(ReadSapTableBodyCommand::new(Self::TABLE))?;
        if row_count == 1 {
            let Some(first_row) = table.iter().next() else {
                return Err(ApplicationError::NoLectureAssessments.into());
            };
            if let Some(Ok(SapTableCellWrapper::Normal(cell))) =
                first_row.iter_value(self.body()).next()
            {
                if let Some(ElementDefWrapper::TextView(tv_def)) = cell.content() {
                    if let Ok(tv) = tv_def.from_body(self.body()) {
                        if tv.text().contains("없습니다.") {
                            return Err(ApplicationError::NoLectureAssessments.into());
                        }
                    }
                }
            }
        }
        let mut results: Vec<LectureAssessmentResult> = Vec::with_capacity(row_count);
        while results.len() < row_count {
            let mut partial_results =
                table.try_table_into::<LectureAssessmentResult>(self.body())?;
            if results.len() + partial_results.len() > row_count {
                let overflowed = results.len() + partial_results.len() - row_count;
                partial_results.drain(0..overflowed);
            }
            results.append(&mut partial_results);
            if results.len() < row_count {
                self.client
                    .send(SapTableVerticalScrollCommand::new(
                        Self::TABLE,
                        results.len().try_into().unwrap(),
                        "",
                        "SCROLLBAR",
                        false,
                        false,
                        false,
                        false,
                    ))
                    .await?;
                table = self
                    .body()
                    .read(ReadSapTableBodyCommand::new(Self::TABLE))?;
            }
        }
        Ok(results)
    }
}

/// [`LectureAssessmentApplication`] 애플리케이션에 사용되는 데이터
pub mod model;
