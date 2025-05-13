use std::collections::HashMap;

use model::{CourseScheduleInformation, PersonalCourseSchedule, Weekday};

use super::{USaintApplication, USaintClient};
use crate::application::utils::semester::get_selected_semester;
use crate::webdynpro::command::WebDynproCommandExecutor;
use crate::webdynpro::element::parser::ElementParser;
use crate::{
    RusaintError, define_elements,
    error::ApplicationError,
    model::SemesterType,
    webdynpro::{
        client::body::Body,
        command::element::selection::{ComboBoxLSDataCommand, ComboBoxSelectEventCommand},
        element::{complex::SapTable, selection::ComboBox},
        error::{ElementError, WebDynproError},
    },
};

/// [개인수업시간표](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW2102)
#[derive(Debug)]
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

    fn semester_to_key(semester: SemesterType) -> &'static str {
        match semester {
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
    ) -> Result<(), WebDynproError> {
        let semester = Self::semester_to_key(semester);
        let parser = ElementParser::new(self.body());
        let year_combobox_lsdata = parser.read(ComboBoxLSDataCommand::new(Self::PERYR))?;
        let semester_combobox_lsdata = parser.read(ComboBoxLSDataCommand::new(Self::PERID))?;
        if year_combobox_lsdata.key().map(String::as_str) != Some(year) {
            let event = parser.read(ComboBoxSelectEventCommand::new(Self::PERYR, year, false))?;
            self.client.process_event(false, event).await?;
        }
        if semester_combobox_lsdata.key().map(String::as_str) != Some(semester) {
            let event = parser.read(ComboBoxSelectEventCommand::new(
                Self::PERID,
                semester,
                false,
            ))?;
            self.client.process_event(false, event).await?;
        }
        Ok(())
    }

    /// 현재 페이지에 선택된 년도와 학기를 가져옵니다. 최초 로드 시 현재 학기를 가져올 가능성이 있습니다.
    /// 하지만 이 애플리케이션의 다른 함수를 호출하여 한번 정보를 가져왔다면 마지막으로 가져온 정보의 학기가 반환되므로 주의하여야 하며, 신뢰할 수 있는 현재 학기의 원천으로 사용되어서는 안됩니다.
    pub fn get_selected_semester(&self) -> Result<(u32, SemesterType), RusaintError> {
        Ok(get_selected_semester(
            &self.client,
            &Self::PERYR,
            &Self::PERID,
        )?)
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
                for row in row_string.into_iter().skip(1) {
                    row.into_iter()
                        .skip(1)
                        .enumerate()
                        .filter_map(|(col_idx, option)| option.map(|str| (col_idx, str)))
                        .for_each(|(col_idx, str)| match col_idx {
                            0 => {
                                schedule.entry(Weekday::Mon).or_default();
                                let mut iter = str.split("\n").peekable();
                                while iter.peek().is_some() {
                                    schedule
                                        .get_mut(&Weekday::Mon)
                                        .unwrap()
                                        .push(CourseScheduleInformation::from_iter(&mut iter))
                                }
                            }
                            1 => {
                                schedule.entry(Weekday::Tue).or_default();
                                let mut iter = str.split("\n").peekable();
                                while iter.peek().is_some() {
                                    schedule
                                        .get_mut(&Weekday::Tue)
                                        .unwrap()
                                        .push(CourseScheduleInformation::from_iter(&mut iter))
                                }
                            }
                            2 => {
                                schedule.entry(Weekday::Wed).or_default();
                                let mut iter = str.split("\n").peekable();
                                while iter.peek().is_some() {
                                    schedule
                                        .get_mut(&Weekday::Wed)
                                        .unwrap()
                                        .push(CourseScheduleInformation::from_iter(&mut iter))
                                }
                            }
                            3 => {
                                schedule.entry(Weekday::Thu).or_default();
                                let mut iter = str.split("\n").peekable();
                                while iter.peek().is_some() {
                                    schedule
                                        .get_mut(&Weekday::Thu)
                                        .unwrap()
                                        .push(CourseScheduleInformation::from_iter(&mut iter))
                                }
                            }
                            4 => {
                                schedule.entry(Weekday::Fri).or_default();
                                let mut iter = str.split("\n").peekable();
                                while iter.peek().is_some() {
                                    schedule
                                        .get_mut(&Weekday::Fri)
                                        .unwrap()
                                        .push(CourseScheduleInformation::from_iter(&mut iter))
                                }
                            }
                            5 => {
                                schedule.entry(Weekday::Sat).or_default();
                                let mut iter = str.split("\n").peekable();
                                while iter.peek().is_some() {
                                    schedule
                                        .get_mut(&Weekday::Sat)
                                        .unwrap()
                                        .push(CourseScheduleInformation::from_iter(&mut iter))
                                }
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
