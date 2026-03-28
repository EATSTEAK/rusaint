use crate::ApplicationError;
use crate::application::course_registration_status::model::RegisteredLecture;
use crate::application::utils::oz::{
    extract_oz_url_from_script_calls, fetch_data_module, parse_oz_url_params,
};
use crate::application::utils::semester::get_selected_semester;
use crate::client::{USaintApplication, USaintClient};
use crate::{RusaintError, model::SemesterType};
use wdpe::command::WebDynproCommandExecutor;
use wdpe::command::element::action::ButtonPressEventCommand;
use wdpe::element::action::Button;
use wdpe::element::parser::ElementParser;
use wdpe::state::EventProcessResult;
use wdpe::{
    body::Body, command::element::selection::ComboBoxSelectEventCommand, define_elements,
    element::selection::ComboBox, error::WebDynproError,
};

/// [수강신청조회(학생용)](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW2110)
#[derive(Debug)]
pub struct CourseRegistrationStatusApplication {
    client: USaintClient,
}

impl USaintApplication for CourseRegistrationStatusApplication {
    const APP_NAME: &'static str = "ZCMW2110";

    fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
        if client.name() != Self::APP_NAME {
            Err(RusaintError::InvalidClientError)
        } else {
            Ok(Self { client })
        }
    }
}

#[allow(unused)]
impl<'app> CourseRegistrationStatusApplication {
    fn semester_to_key(semester: SemesterType) -> &'static str {
        match semester {
            SemesterType::One => "090",
            SemesterType::Summer => "091",
            SemesterType::Two => "092",
            SemesterType::Winter => "093",
        }
    }

    async fn select_semester(
        &mut self,
        parser: &ElementParser,
        year: &str,
        semester: SemesterType,
    ) -> Result<(), WebDynproError> {
        let year_select_event = parser.read(ComboBoxSelectEventCommand::new(
            Self::PERIOD_YEAR,
            year,
            false,
        ))?;
        self.client.process_event(false, year_select_event).await?;
        let semester_select_event = parser.read(ComboBoxSelectEventCommand::new(
            Self::PERIOD_ID,
            Self::semester_to_key(semester),
            false,
        ))?;
        self.client
            .process_event(false, semester_select_event)
            .await?;
        Ok(())
    }

    fn body(&self) -> &Body {
        self.client.body()
    }

    /// 현재 페이지에 선택된 년도와 학기를 가져옵니다. 최초 로드 시 현재 학기를 가져올 가능성이 있습니다.
    /// 하지만 이 애플리케이션의 다른 함수를 호출하여 한번 정보를 가져왔다면 마지막으로 가져온 정보의 학기가 반환되므로 주의하여야 하며, 신뢰할 수 있는 현재 학기의 원천으로 사용되어서는 안됩니다.
    pub fn get_selected_semester(&self) -> Result<(u32, SemesterType), RusaintError> {
        Ok(get_selected_semester(
            &self.client,
            &Self::PERIOD_YEAR,
            &Self::PERIOD_ID,
        )?)
    }

    // 메인 요소
    define_elements! {
        PERIOD_YEAR: ComboBox<'app> = "ZCMW_PERIOD_RE.ID_57CC7986881470383154D0F1FF86642A:VIW_MAIN.PERYR";
        PERIOD_ID: ComboBox<'app> = "ZCMW_PERIOD_RE.ID_57CC7986881470383154D0F1FF86642A:VIW_MAIN.PERID";
        BUTTON_PRINT: Button<'app> = "ZCMW2110.ID_0001:VIW_MAIN.BUTTON_PRINT";
    }

    /// 개인이 수강신청한 내역을 학기별로 찾습니다.
    pub async fn lectures(
        &mut self,
        year: u32,
        semester: SemesterType,
    ) -> Result<impl Iterator<Item = RegisteredLecture>, RusaintError> {
        let parser = ElementParser::new(self.client.body());
        let button_press_event = parser.read(ButtonPressEventCommand::new(Self::BUTTON_PRINT))?;
        let result = self.client.process_event(true, button_press_event).await?;

        let script_calls = match result {
            EventProcessResult::Sent(body_update_result) => {
                body_update_result.script_calls.unwrap_or_default()
            }
            EventProcessResult::Enqueued => {
                return Err(ApplicationError::OzDataFetchError(
                    "BUTTON_PRINT event was enqueued but not sent".to_string(),
                )
                .into());
            }
        };

        let oz_url = extract_oz_url_from_script_calls(&script_calls)?;
        let mut oz_params = parse_oz_url_params(&oz_url)?;
        let semester_key = Self::semester_to_key(semester).to_string();
        let year_value = year.to_string();

        for (key, value) in &mut oz_params.params {
            if key == "arg1" {
                *value = year_value.clone();
            } else if key == "arg2" {
                *value = semester_key.clone();
            }
        }

        let response = fetch_data_module(&oz_params).await?;
        let lectures = RegisteredLecture::from_datasets(&response.datasets)?;
        Ok(lectures.into_iter())
    }

    /// 페이지를 새로고침합니다.
    pub async fn reload(&mut self) -> Result<(), RusaintError> {
        self.client.reload().await?;
        Ok(())
    }
}

/// [`CourseRegistrationStatusApplication`](CourseRegistrationStatusApplication) 에서 사용하는 데이터
pub mod model;
