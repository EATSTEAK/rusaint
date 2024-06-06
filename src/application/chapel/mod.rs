use model::{ChapelAbsenceRequest, ChapelAttendance, ChapelInformation, GeneralChapelInformation};

use crate::{
    define_elements,
    model::SemesterType,
    webdynpro::{
        client::body::Body,
        command::element::{
            action::ButtonPressCommand,
            selection::{ComboBoxSelectCommand, ReadComboBoxLSDataCommand},
        },
        element::{action::Button, selection::ComboBox},
        error::{ElementError, WebDynproError},
    },
    RusaintError,
};

use super::{USaintApplication, USaintClient};

/// [채플정보조회](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW3681)
pub struct Chapel {
    client: USaintClient,
}

impl USaintApplication for Chapel {
    const APP_NAME: &'static str = "ZCMW3681";

    fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
        if client.name() != Self::APP_NAME {
            Err(RusaintError::InvalidClientError)
        } else {
            Ok(Self { client })
        }
    }
}

impl<'a> Chapel {
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
            SemesterType::Winter => "0923",
        }
    }

    fn body(&self) -> &Body {
        self.client.body()
    }

    async fn select_semester(
        &mut self,
        year: &str,
        semester: SemesterType,
    ) -> Result<(), WebDynproError> {
        let semester = Self::semester_to_key(semester);
        let year_combobox_lsdata = self
            .client
            .read(ReadComboBoxLSDataCommand::new(Self::SEL_PERYR))?;
        let semester_combobox_lsdata = self
            .client
            .read(ReadComboBoxLSDataCommand::new(Self::SEL_PERID))?;
        if (|| Some(year_combobox_lsdata.key()?.as_str()))() != Some(year) {
            self.client
                .send(ComboBoxSelectCommand::new(Self::SEL_PERYR, &year, false))
                .await?;
        }
        if (|| Some(semester_combobox_lsdata.key()?.as_str()))() != Some(semester) {
            self.client
                .send(ComboBoxSelectCommand::new(Self::SEL_PERID, semester, false))
                .await?;
        }
        self.client
            .send(ButtonPressCommand::new(Self::BTN_SEL))
            .await?;
        Ok(())
    }

    /// 해당 학기의 채플 정보를 가져옵니다.
    pub async fn information(
        &mut self,
        year: u32,
        semester: SemesterType,
    ) -> Result<ChapelInformation, RusaintError> {
        self.select_semester(&year.to_string(), semester).await?;
        let general_information = GeneralChapelInformation::from_body(self.body())?
            .pop()
            .ok_or_else(|| {
                Into::<RusaintError>::into(Into::<WebDynproError>::into(
                    ElementError::NoSuchContent {
                        element: "General Chapel Information".to_string(),
                        content: "No data provided".to_string(),
                    },
                ))
            })?;
        let attendances = ChapelAttendance::from_body(self.body())?;
        let absence_requests = ChapelAbsenceRequest::from_body(self.body())?;
        Ok(ChapelInformation::new(
            year,
            semester,
            general_information,
            attendances,
            absence_requests,
        ))
    }
}

/// [`Chapel`] 애플리케이션에 사용되는 데이터
pub mod model;
