use std::collections::HashMap;

use model::{CourseScheduleInformation, PersonalCourseSchedule, Weekday};

use super::{USaintApplication, USaintClient};
use crate::webdynpro::command::WebDynproCommandExecutor;
use crate::webdynpro::element::parser::ElementParser;
use crate::{
    define_elements,
    error::ApplicationError,
    model::SemesterType,
    webdynpro::{
        client::body::Body,
        command::element::selection::{ComboBoxSelectCommand, ReadComboBoxLSDataCommand},
        element::{complex::SapTable, selection::ComboBox},
        error::{ElementError, WebDynproError},
    },
    RusaintError,
};

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
        let parser = ElementParser::new(self.body());
        let year_combobox_lsdata = parser.read(ReadComboBoxLSDataCommand::new(Self::PERYR))?;
        let semester_combobox_lsdata = parser.read(ReadComboBoxLSDataCommand::new(Self::PERID))?;
        if year_combobox_lsdata.key().map(String::as_str) != Some(year) {
            let event = parser.read(ComboBoxSelectCommand::new(Self::PERYR, &year, false))?;
            self.client.process_event(false, event).await?;
        }
        if semester_combobox_lsdata.key().map(String::as_str) != Some(semester) {
            let event = parser.read(ComboBoxSelectCommand::new(Self::PERID, semester, false))?;
            self.client.process_event(false, event).await?;
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
        let parser = ElementParser::new(self.body());
        let table = parser.element_from_def(&Self::TABLE);
        match table {
            Ok(table) => {
                let table_body = table.table()?;
                let row_string: Vec<Vec<Option<String>>> =
                    table_body.try_table_into::<Vec<Option<String>>>(&parser)?;
                let mut schedule: HashMap<Weekday, Vec<CourseScheduleInformation>> =
                    Default::default();
                for (_row_idx, row) in row_string.into_iter().skip(1).enumerate() {
                    row.into_iter()
                        .skip(1)
                        .enumerate()
                        .filter_map(|(col_idx, option)| match option {
                            Some(str) => Some((col_idx, str)),
                            None => None,
                        })
                        .for_each(|(col_idx, str)| match col_idx {
                            0 => {
                                if !schedule.contains_key(&Weekday::Mon) {
                                    schedule.insert(Weekday::Mon, Vec::new());
                                }
                                str.split("\n\n").for_each(|str| {
                                    schedule
                                        .get_mut(&Weekday::Mon)
                                        .unwrap()
                                        .push(CourseScheduleInformation::from_string(str))
                                });
                            }
                            1 => {
                                if !schedule.contains_key(&Weekday::Tue) {
                                    schedule.insert(Weekday::Tue, Vec::new());
                                }
                                str.split("\n\n").for_each(|str| {
                                    schedule
                                        .get_mut(&Weekday::Tue)
                                        .unwrap()
                                        .push(CourseScheduleInformation::from_string(str))
                                });
                            }
                            2 => {
                                if !schedule.contains_key(&Weekday::Wed) {
                                    schedule.insert(Weekday::Wed, Vec::new());
                                }
                                str.split("\n\n").for_each(|str| {
                                    schedule
                                        .get_mut(&Weekday::Wed)
                                        .unwrap()
                                        .push(CourseScheduleInformation::from_string(str))
                                });
                            }
                            3 => {
                                if !schedule.contains_key(&Weekday::Thu) {
                                    schedule.insert(Weekday::Thu, Vec::new());
                                }
                                str.split("\n\n").for_each(|str| {
                                    schedule
                                        .get_mut(&Weekday::Thu)
                                        .unwrap()
                                        .push(CourseScheduleInformation::from_string(str))
                                });
                            }
                            4 => {
                                if !schedule.contains_key(&Weekday::Fri) {
                                    schedule.insert(Weekday::Fri, Vec::new());
                                }
                                str.split("\n\n").for_each(|str| {
                                    schedule
                                        .get_mut(&Weekday::Fri)
                                        .unwrap()
                                        .push(CourseScheduleInformation::from_string(str))
                                });
                            }
                            5 => {
                                if !schedule.contains_key(&Weekday::Sat) {
                                    schedule.insert(Weekday::Sat, Vec::new());
                                }
                                str.split("\n\n").for_each(|str| {
                                    schedule
                                        .get_mut(&Weekday::Sat)
                                        .unwrap()
                                        .push(CourseScheduleInformation::from_string(str))
                                });
                            }
                            _ => {}
                        });
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
