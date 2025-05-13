use model::LectureAssessmentResult;

use super::{USaintApplication, USaintClient};
use crate::application::utils::sap_table::try_table_into_with_scroll;
use crate::application::utils::semester::get_selected_semester;
use crate::webdynpro::command::WebDynproCommandExecutor;
use crate::webdynpro::element::parser::ElementParser;
use crate::{
    ApplicationError, RusaintError, define_elements,
    model::SemesterType,
    webdynpro::{
        client::body::Body,
        command::element::{
            action::ButtonPressEventCommand,
            complex::{SapTableBodyCommand, SapTableLSDataCommand},
            selection::{
                ComboBoxChangeEventCommand, ComboBoxLSDataCommand, ComboBoxSelectEventCommand,
            },
        },
        element::{
            ElementDefWrapper,
            action::Button,
            complex::{
                SapTable,
                sap_table::cell::{SapTableCell, SapTableCellWrapper},
            },
            definition::ElementDefinition,
            selection::ComboBox,
        },
        error::{ElementError, WebDynproError},
    },
};

/// [강의평가조회](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMB2W1010)
#[derive(Debug)]
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

    fn semester_to_key(semester: SemesterType) -> &'static str {
        match semester {
            SemesterType::One => "090",
            SemesterType::Summer => "091",
            SemesterType::Two => "092",
            SemesterType::Winter => "093",
        }
    }

    /// 현재 페이지에 선택된 년도와 학기를 가져옵니다. 최초 로드 시 현재 학기를 가져올 가능성이 있습니다.
    /// 하지만 이 애플리케이션의 다른 함수를 호출하여 한번 정보를 가져왔다면 마지막으로 가져온 정보의 학기가 반환되므로 주의하여야 하며, 신뢰할 수 있는 현재 학기의 원천으로 사용되어서는 안됩니다.
    pub fn get_selected_semester(&self) -> Result<(u32, SemesterType), RusaintError> {
        Ok(get_selected_semester(
            &self.client,
            &Self::DDLB_01,
            &Self::DDLB_02,
        )?)
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
        let parser = ElementParser::new(self.body());
        let semester = Self::semester_to_key(semester);
        let year_combobox_lsdata = parser.read(ComboBoxLSDataCommand::new(Self::DDLB_01))?;
        let semester_combobox_lsdata = parser.read(ComboBoxLSDataCommand::new(Self::DDLB_02))?;
        if year_combobox_lsdata.key().map(String::as_str) != Some(year) {
            let event = parser.read(ComboBoxSelectEventCommand::new(Self::DDLB_01, year, false))?;
            self.client.process_event(false, event).await?;
        }
        if semester_combobox_lsdata.key().map(String::as_str) != Some(semester) {
            let event = parser.read(ComboBoxSelectEventCommand::new(
                Self::DDLB_02,
                semester,
                false,
            ))?;
            self.client.process_event(false, event).await?;
        }
        if let Some(lecture_name) = lecture_name {
            let event = parser.read(ComboBoxChangeEventCommand::new(
                Self::IF_01,
                lecture_name,
                false,
            ))?;
            self.client.process_event(false, event).await?;
        }
        if let Some(lecture_code) = lecture_code {
            let event = parser.read(ComboBoxChangeEventCommand::new(
                Self::ILSM_OBJID,
                &lecture_code.to_string(),
                false,
            ))?;
            self.client.process_event(false, event).await?;
        }
        if let Some(professor_name) = professor_name {
            let event = parser.read(ComboBoxChangeEventCommand::new(
                Self::IF_04,
                professor_name,
                false,
            ))?;
            self.client.process_event(false, event).await?;
        }
        let btn_press = parser.read(ButtonPressEventCommand::new(Self::BT_SEARCH))?;
        self.client.process_event(false, btn_press).await?;
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
        let parser = ElementParser::new(self.body());
        let row_count = parser
            .read(SapTableLSDataCommand::new(Self::TABLE))?
            .row_count()
            .map(|u| u.to_owned())
            .ok_or_else(|| {
                WebDynproError::Element(ElementError::NoSuchData {
                    element: Self::TABLE.id().to_string(),
                    field: "row_count".to_string(),
                })
            })?;
        let table = parser.read(SapTableBodyCommand::new(Self::TABLE))?;
        if row_count == 1 {
            let Some(first_row) = table.iter().next() else {
                return Err(ApplicationError::NoLectureAssessments.into());
            };
            if let Some(Ok(SapTableCellWrapper::Normal(cell))) =
                first_row.iter_value(&parser).next()
            {
                if let Some(ElementDefWrapper::TextView(tv_def)) = cell.content() {
                    if let Ok(tv) = parser.element_from_def(&tv_def) {
                        if tv.text().contains("없습니다.") {
                            return Err(ApplicationError::NoLectureAssessments.into());
                        }
                    }
                }
            }
        }
        Ok(try_table_into_with_scroll(&mut self.client, parser, Self::TABLE).await?)
    }
}

/// [`LectureAssessmentApplication`] 애플리케이션에 사용되는 데이터
pub mod model;
