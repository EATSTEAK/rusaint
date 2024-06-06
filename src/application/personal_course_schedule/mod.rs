use model::PersonalCourseSchedule;

use crate::{
    define_elements,
    error::ApplicationError,
    model::SemesterType,
    webdynpro::{
        client::body::Body,
        command::element::selection::{ComboBoxSelectCommand, ReadComboBoxLSDataCommand},
        element::{complex::SapTable, definition::ElementDefinition, selection::ComboBox},
        error::{ElementError, WebDynproError},
    },
    RusaintError,
};

use super::{USaintApplication, USaintClient};

/// [개인수업시간표](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW2102)
pub struct PersonalCourseScheduleApplication {
    client: USaintClient,
}

impl USaintApplication for PersonalCourseScheduleApplication {
    const APP_NAME: &'static str = "ZCMW2102";

    fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
        if client.name() != Self::APP_NAME {
            Err(RusaintError::InvalidClientError)
        } else {
            Ok(Self { client })
        }
    }
}

impl<'a> PersonalCourseScheduleApplication {
    define_elements! {
        PERYR: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_D8FB9BECD84FD622F5EAED9E0BE35D27:VIW_MAIN.PERYR";
        PERID: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_D8FB9BECD84FD622F5EAED9E0BE35D27:VIW_MAIN.PERID";
        TABLE: SapTable<'a> = "ZCMW2102.ID_0001:VIW_MAIN.TABLE";
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
            .read(ReadComboBoxLSDataCommand::new(Self::PERYR))?;
        let semester_combobox_lsdata = self
            .client
            .read(ReadComboBoxLSDataCommand::new(Self::PERID))?;
        if (|| Some(year_combobox_lsdata.key()?.as_str()))() != Some(year) {
            self.client
                .send(ComboBoxSelectCommand::new(Self::PERYR, &year, false))
                .await?;
        }
        if (|| Some(semester_combobox_lsdata.key()?.as_str()))() != Some(semester) {
            self.client
                .send(ComboBoxSelectCommand::new(Self::PERID, semester, false))
                .await?;
        }
        Ok(())
    }

    /// 해당 학기의 시간표 정보를 가져옵니다.
    pub async fn schedule(
        &mut self,
        year: u32,
        semester: SemesterType,
    ) -> Result<PersonalCourseSchedule, RusaintError> {
        self.select_semester(&year.to_string(), semester).await?;
        match Self::TABLE.from_body(self.body()) {
            Ok(table) => {
                let table_body = table.table()?;
                let row_string: Vec<Vec<Option<String>>> =
                    table_body.try_table_into::<Vec<Option<String>>>(self.body())?;
                let mut schedule: [[String; 10]; 7] = Default::default();
                for (row_idx, row) in row_string.into_iter().skip(1).enumerate() {
                    row.into_iter()
                        .skip(1)
                        .enumerate()
                        .filter_map(|(col_idx, option)| match option {
                            Some(str) => Some((col_idx, str)),
                            None => None,
                        })
                        .for_each(|(col_idx, str)| schedule[col_idx][row_idx] = str);
                }
                Ok(PersonalCourseSchedule::new(schedule))
            }
            Err(err) => match err {
                WebDynproError::Element(ElementError::InvalidId(_id)) => Err(
                    RusaintError::ApplicationError(ApplicationError::NoScheduleInformation),
                ),
                err => Err(RusaintError::WebDynproError(err)),
            },
        }
    }
}

/// [`PersonalCourseSchedule`] 애플리케이션에 사용되는 데이터
pub mod model;
