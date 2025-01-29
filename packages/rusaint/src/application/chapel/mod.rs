use model::{ChapelAbsenceRequest, ChapelAttendance, ChapelInformation, GeneralChapelInformation};

use super::{USaintApplication, USaintClient};
use crate::webdynpro::command::WebDynproCommandExecutor;
use crate::webdynpro::element::parser::ElementParser;
use crate::{
    define_elements,
    model::SemesterType,
    webdynpro::{
        client::body::Body,
        command::element::{
            action::ButtonPressEventCommand,
            selection::{ComboBoxLSDataCommand, ComboBoxSelectEventCommand},
        },
        element::{action::Button, selection::ComboBox},
        error::{ElementError, WebDynproError},
    },
    RusaintError,
};

/// [채플정보조회](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW3681)
#[derive(Debug)]
pub struct ChapelApplication {
    client: USaintClient,
}

impl USaintApplication for ChapelApplication {
    const APP_NAME: &'static str = "ZCMW3681";

    fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
        if client.name() != Self::APP_NAME {
            Err(RusaintError::InvalidClientError)
        } else {
            Ok(Self { client })
        }
    }
}

impl<'a> ChapelApplication {
    define_elements! {
        SEL_PERYR: ComboBox<'a> = "ZCMW3681.ID_0001:V_MAIN.TC_SEL_PERYR";
        SEL_PERID: ComboBox<'a> = "ZCMW3681.ID_0001:V_MAIN.TC_SEL_PERID";
        BTN_SEL: Button<'a> = "ZCMW3681.ID_0001:V_MAIN.BTN_SEL";
    }

    fn semester_to_key(period: SemesterType) -> &'static str {
        match period {
            SemesterType::One => "090",
            SemesterType::Summer => "091",
            SemesterType::Two => "092",
            SemesterType::Winter => "093",
        }
    }

    fn body(&self) -> &Body {
        self.client.body()
    }

    async fn select_semester(
        &mut self,
        year: &str,
        semester: SemesterType,
    ) -> Result<(), RusaintError> {
        let semester = Self::semester_to_key(semester);
        let parser = ElementParser::new(self.body());
        let year_combobox_lsdata = parser.read(ComboBoxLSDataCommand::new(Self::SEL_PERYR))?;
        let semester_combobox_lsdata = parser.read(ComboBoxLSDataCommand::new(Self::SEL_PERID))?;
        if year_combobox_lsdata.key().map(String::as_str) != Some(year) {
            let year_select_event = parser.read(ComboBoxSelectEventCommand::new(
                Self::SEL_PERYR,
                year,
                false,
            ))?;
            self.client.process_event(false, year_select_event).await?;
        }
        if semester_combobox_lsdata.key().map(String::as_str) != Some(semester) {
            let semester_select_event = parser.read(ComboBoxSelectEventCommand::new(
                Self::SEL_PERID,
                semester,
                false,
            ))?;
            self.client
                .process_event(false, semester_select_event)
                .await?;
        }
        let button_press_event = parser.read(ButtonPressEventCommand::new(Self::BTN_SEL))?;
        self.client.process_event(false, button_press_event).await?;
        Ok(())
    }

    /// 해당 학기의 채플 정보를 가져옵니다.
    pub async fn information(
        &mut self,
        year: u32,
        semester: SemesterType,
    ) -> Result<ChapelInformation, RusaintError> {
        self.select_semester(&year.to_string(), semester).await?;
        let parser = ElementParser::new(self.body());
        let general_information = GeneralChapelInformation::with_parser(&parser)?
            .pop()
            .ok_or_else(|| {
                Into::<RusaintError>::into(Into::<WebDynproError>::into(
                    ElementError::NoSuchContent {
                        element: "General Chapel Information".to_string(),
                        content: "No data provided".to_string(),
                    },
                ))
            })?;
        let attendances = ChapelAttendance::with_parser(&parser)?;
        let absence_requests = ChapelAbsenceRequest::with_parser(&parser)?;
        Ok(ChapelInformation::new(
            year,
            semester,
            general_information,
            attendances,
            absence_requests,
        ))
    }
}

/// [`ChapelApplication`] 애플리케이션에 사용되는 데이터
pub mod model;
