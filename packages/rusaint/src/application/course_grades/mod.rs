use self::model::{ClassGrade, CourseType, GradeSummary, SemesterGrade};
use super::{USaintApplication, USaintClient};
use crate::application::utils::sap_table::try_table_into_with_scroll;
use crate::application::utils::semester::get_selected_semester;
use crate::webdynpro::client::body::Body;
use crate::webdynpro::command::WebDynproCommandExecutor;
use crate::webdynpro::element::complex::sap_table::cell::SapTableCellWrapper;
use crate::webdynpro::element::parser::ElementParser;
use crate::{
    RusaintError, define_elements,
    model::SemesterType,
    webdynpro::{
        command::element::{
            complex::SapTableBodyCommand,
            selection::{ComboBoxLSDataCommand, ComboBoxSelectEventCommand},
        },
        element::{
            Element, ElementDefWrapper, ElementWrapper,
            complex::sap_table::{SapTable, cell::SapTableCell},
            definition::ElementDefinition,
            layout::PopupWindow,
            selection::ComboBox,
            text::InputField,
        },
        error::{BodyError, ElementError, WebDynproError},
        event::Event,
    },
};
use scraper::Selector;
use std::collections::HashMap;

/// [학생 성적 조회](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMB3W0017)
#[derive(Debug)]
pub struct CourseGradesApplication {
    client: USaintClient,
}

impl USaintApplication for CourseGradesApplication {
    const APP_NAME: &'static str = "ZCMB3W0017";

    fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
        if client.name() != Self::APP_NAME {
            Err(RusaintError::InvalidClientError)
        } else {
            Ok(Self { client })
        }
    }
}

#[allow(unused)]
impl<'a> CourseGradesApplication {
    // Elements for Grade Summaries
    define_elements!(
        // Grade summaries by semester
        GRADES_SUMMARY_TABLE: SapTable<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.TABLE";
        // Progress type
        PROGRESS_TYPE: ComboBox<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.PROGC_VAR";
        // Attempted Credits in Record
        ATTM_CRD1: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.ATTM_CRD1";
        // Earned Credits in Record
        EARN_CRD1: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.EARN_CRD1";
        // GPA in Record
        GT_GPA1: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.GT_GPA1";
        // Class GPA in Record
        CGPA1: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.CGPA1";
        // Average Points in Record
        AVG1: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.AVG1";
        // Credits earned in P/F Classes in Record
        PF_EARN_CRD: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.PF_EARN_CRD";
        // Attempted Credits in Certificate
        ATTM_CRD2: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.ATTM_CRD2";
        // Earned Credits in Certificate
        EARN_CRD2: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.EARN_CRD2";
        // GPA in Certificate
        GT_GPA2: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.GT_GPA2";
        // Class GPA in Certificate
        CGPA2: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.CGPA2";
        // Average Points in Certificate
        AVG2: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.AVG2";
        // Credits earned in P/F Classes in Certificate
        PF_EARN_CRD1: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.T_PF_ERN_CRD1";
    );

    // Elements for Class Grades
    define_elements!(
        PERIOD_YEAR: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69:VIW_MAIN.PERYR";
        PERIOD_SEMESTER: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69:VIW_MAIN.PERID";
        GRADE_BY_CLASSES_TABLE: SapTable<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.TABLE_1";
    );

    async fn close_popups(&mut self) -> Result<(), WebDynproError> {
        let popup_selector =
            Selector::parse(format!(r#"[ct="{}"]"#, PopupWindow::CONTROL_ID).as_str()).unwrap();
        fn make_close_event(body: &Body, selector: &Selector) -> Option<Event> {
            let parser = ElementParser::new(body);
            let mut popup_iter = parser.document().select(selector);
            popup_iter.next().and_then(|elem| {
                let elem_wrapped = ElementWrapper::from_ref(elem).ok()?;
                if let ElementWrapper::PopupWindow(popup) = elem_wrapped {
                    popup.close().ok()
                } else {
                    None
                }
            })
        }
        while let Some(event) = make_close_event(self.body(), &popup_selector) {
            self.client.process_event(false, event).await?;
        }
        Ok(())
    }

    fn semester_to_key(semester: SemesterType) -> &'static str {
        match semester {
            SemesterType::One => "090",
            SemesterType::Summer => "091",
            SemesterType::Two => "092",
            SemesterType::Winter => "093",
        }
    }

    fn course_type_to_key(course_type: CourseType) -> &'static str {
        match course_type {
            CourseType::Phd => "DR",
            CourseType::Master => "MA",
            CourseType::PhdIntergrated => "MP",
            CourseType::Research => "RE",
            CourseType::Bachelor => "UG",
        }
    }

    async fn select_course(
        &mut self,
        parser: &ElementParser,
        course: CourseType,
    ) -> Result<(), WebDynproError> {
        let course = Self::course_type_to_key(course);
        let combobox_lsdata = parser.read(ComboBoxLSDataCommand::new(Self::PROGRESS_TYPE))?;
        if combobox_lsdata.key().map(String::as_str) != Some(course) {
            let select_event = parser.read(ComboBoxSelectEventCommand::new(
                Self::PROGRESS_TYPE,
                course,
                false,
            ))?;
            self.client.process_event(false, select_event).await?;
        }
        Ok(())
    }

    async fn select_semester(
        &mut self,
        parser: &ElementParser,
        year: &str,
        semester: SemesterType,
    ) -> Result<(), WebDynproError> {
        let semester = Self::semester_to_key(semester);
        let year_combobox_lsdata = parser.read(ComboBoxLSDataCommand::new(Self::PERIOD_YEAR))?;
        let semester_combobox_lsdata =
            parser.read(ComboBoxLSDataCommand::new(Self::PERIOD_SEMESTER))?;
        if year_combobox_lsdata.key().map(String::as_str) != Some(year) {
            let year_select_event = parser.read(ComboBoxSelectEventCommand::new(
                Self::PERIOD_YEAR,
                year,
                false,
            ))?;
            self.client.process_event(false, year_select_event).await?;
        }
        if semester_combobox_lsdata.key().map(String::as_str) != Some(semester) {
            let semester_select_event = parser.read(ComboBoxSelectEventCommand::new(
                Self::PERIOD_SEMESTER,
                semester,
                false,
            ))?;
            self.client
                .process_event(false, semester_select_event)
                .await?;
        }
        Ok(())
    }

    /// 현재 페이지에 선택된 년도와 학기를 가져옵니다. 최초 로드 시 현재 학기를 가져올 가능성이 있습니다.
    /// 하지만 이 애플리케이션의 다른 함수를 호출하여 한번 정보를 가져왔다면 마지막으로 가져온 정보의 학기가 반환되므로 주의하여야 하며, 신뢰할 수 있는 현재 학기의 원천으로 사용되어서는 안됩니다.
    pub fn get_selected_semester(&self) -> Result<(u32, SemesterType), RusaintError> {
        Ok(get_selected_semester(
            &self.client,
            &Self::PERIOD_YEAR,
            &Self::PERIOD_SEMESTER,
        )?)
    }

    /// 전체 학기의 학적부 평점 정보를 가져옵니다.
    /// ### 예시
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// # use std::sync::Arc;
    /// # use rusaint::USaintSession;
    /// # use rusaint::application::course_grades::{ model::CourseType, CourseGradesApplication };
    /// # use rusaint::application::USaintClientBuilder;
    /// # let session = Arc::new(USaintSession::with_password("20212345", "password").await.unwrap());
    /// let mut app = USaintClientBuilder::new().session(session).build_into::<CourseGradesApplication>().await.unwrap();
    /// let summary = app.recorded_summary(CourseType::Bachelor).await.unwrap();
    /// println!("{:?}", summary);
    /// // GradeSummary { ... }
    /// # })
    /// ```
    pub async fn recorded_summary(
        &mut self,
        course_type: CourseType,
    ) -> Result<GradeSummary, RusaintError> {
        self.close_popups().await?;
        let parser = ElementParser::new(self.client.body());
        self.select_course(&parser, course_type).await?;
        self.read_recorded_summary()
    }

    fn read_recorded_summary(&self) -> Result<GradeSummary, RusaintError> {
        let parser = ElementParser::new(self.client.body());
        let attempted_credits = parser
            .element_from_def(&Self::ATTM_CRD1)?
            .value_into_f32()?;
        let earned_credits = parser
            .element_from_def(&Self::EARN_CRD1)?
            .value_into_f32()?;
        let gpa = parser.element_from_def(&Self::GT_GPA1)?.value_into_f32()?;
        let cgpa = parser.element_from_def(&Self::CGPA1)?.value_into_f32()?;
        let avg = parser.element_from_def(&Self::AVG1)?.value_into_f32()?;
        let pf_earned_credits = parser
            .element_from_def(&Self::PF_EARN_CRD)?
            .value_into_f32()?;
        Ok(GradeSummary::new(
            attempted_credits,
            earned_credits,
            gpa,
            cgpa,
            avg,
            pf_earned_credits,
        ))
    }

    /// 전체 학기의 증명 평점 정보를 가져옵니다.
    /// ### 예시
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// # use std::sync::Arc;
    /// # use rusaint::USaintSession;
    /// # use rusaint::application::USaintClientBuilder;
    /// # use rusaint::application::course_grades::{ model::CourseType, CourseGradesApplication };
    /// # let session = Arc::new(USaintSession::with_password("20212345", "password").await.unwrap());
    /// let mut app = USaintClientBuilder::new().session(session).build_into::<CourseGradesApplication>().await.unwrap();
    /// let summary = app.certificated_summary(CourseType::Bachelor).await.unwrap();
    /// println!("{:?}", summary);
    /// // GradeSummary { ... }
    /// # })
    /// ```
    pub async fn certificated_summary(
        &mut self,
        course_type: CourseType,
    ) -> Result<GradeSummary, RusaintError> {
        self.close_popups().await?;
        let parser = ElementParser::new(self.client.body());
        self.select_course(&parser, course_type).await?;
        self.read_certificated_summary()
    }

    fn read_certificated_summary(&self) -> Result<GradeSummary, RusaintError> {
        let parser = ElementParser::new(self.client.body());
        let attempted_credits = parser
            .element_from_def(&Self::ATTM_CRD2)?
            .value_into_f32()?;
        let earned_credits = parser
            .element_from_def(&Self::EARN_CRD2)?
            .value_into_f32()?;
        let gpa = parser.element_from_def(&Self::GT_GPA2)?.value_into_f32()?;
        let cgpa = parser.element_from_def(&Self::CGPA2)?.value_into_f32()?;
        let avg = parser.element_from_def(&Self::AVG2)?.value_into_f32()?;
        let pf_earned_credits = parser
            .element_from_def(&Self::PF_EARN_CRD1)?
            .value_into_f32()?;
        Ok(GradeSummary::new(
            attempted_credits,
            earned_credits,
            gpa,
            cgpa,
            avg,
            pf_earned_credits,
        ))
    }

    /// 학기별 평점 정보를 가져옵니다.
    /// ### 예시
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// # use std::sync::Arc;
    /// # use rusaint::USaintSession;
    /// # use rusaint::application::course_grades::{ model::CourseType, CourseGradesApplication };
    /// # use rusaint::application::USaintClientBuilder;
    /// # let session = Arc::new(USaintSession::with_password("20212345", "password").await.unwrap());
    /// let mut app = USaintClientBuilder::new().session(session).build_into::<CourseGradesApplication>().await.unwrap();
    /// let semesters = app.semesters(CourseType::Bachelor).await.unwrap();
    /// println!("{:?}", semesters);
    /// // [SemesterGrade { ... }, SemesterGrade { ... }]
    /// # })
    /// ```
    pub async fn semesters(
        &mut self,
        course_type: CourseType,
    ) -> Result<Vec<SemesterGrade>, RusaintError> {
        self.close_popups().await?;
        let parser = ElementParser::new(self.client.body());
        self.select_course(&parser, course_type).await?;
        self.read_semesters().await
    }

    async fn read_semesters(&mut self) -> Result<Vec<SemesterGrade>, RusaintError> {
        let parser = ElementParser::new(self.client.body());
        let ret = try_table_into_with_scroll::<SemesterGrade>(
            &mut self.client,
            parser,
            Self::GRADES_SUMMARY_TABLE,
        )
        .await?;
        Ok(ret)
    }

    async fn class_detail_in_popup(
        &mut self,
        press_event: Event,
    ) -> Result<HashMap<String, f32>, RusaintError> {
        self.client.process_event(false, press_event).await?;

        let parse_table_in_popup = |body: &Body| -> Result<HashMap<String, f32>, WebDynproError> {
            let table_inside_popup_selector = Selector::parse(r#"[ct="PW"] [ct="ST"]"#).unwrap();
            let parser = ElementParser::new(body);
            let mut table_inside_popup = parser.document().select(&table_inside_popup_selector);
            let table_ref = table_inside_popup
                .next()
                .ok_or(BodyError::NoSuchElement("Table in popup".to_string()))?;
            let table_elem: SapTable<'_> = ElementWrapper::from_ref(table_ref)?.try_into()?;
            let table_body = table_elem.table()?;
            let zip = table_body
                .iter()
                .next()
                .ok_or(ElementError::InvalidContent {
                    element: table_elem.id().to_string(),
                    content: "header and first row".to_string(),
                })?
                .try_row_into::<Vec<(String, String)>>(table_body.header(), &parser)?
                .into_iter();
            zip.skip(4)
                .map(|(key, val)| {
                    let str = val.trim();
                    if str.is_empty() {
                        return Ok((key, -1.0));
                    }
                    let float = str.parse::<f32>().or(Err(ElementError::InvalidContent {
                        element: format!("TABLE: {}, key: {}", table_elem.id(), key),
                        content: "(not an correct f32)".to_string(),
                    }))?;
                    Ok((key, float))
                })
                .collect::<Result<HashMap<String, f32>, WebDynproError>>()
        };
        let table = parse_table_in_popup(self.client.body())?;
        self.close_popups().await?;
        Ok(HashMap::from_iter(table))
    }

    /// 주어진 학기의 수업별 성적을 가져옵니다. `include_details`가 `true`인 경우 수업의 상세 성적도 가져옵니다.
    /// 수업의 상세 성적까지 가져올 경우 상세 성적이 있는 수업의 수 만큼 서버에 요청을 보내므로 반드시 상세 성적도 한번에 가져와야 할 때에만 사용하십시오.
    ///
    /// 수업 성적을 가져온 이후 상세 성적 또한 가져오려면 [`class_detail()`]함수를 이용하십시오.
    /// ### 예시
    /// 상세 성적을 가져오지 않을 경우
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// # use std::sync::Arc;
    /// # use rusaint::USaintSession;
    /// # use rusaint::application::course_grades::{ model::CourseType, CourseGradesApplication };
    /// # use rusaint::application::USaintClientBuilder;
    /// # use rusaint::model::SemesterType;
    /// # let session = Arc::new(USaintSession::with_password("20212345", "password").await.unwrap());
    /// let mut app = USaintClientBuilder::new().session(session).build_into::<CourseGradesApplication>().await.unwrap();
    /// let classes = app.classes(CourseType::Bachelor, 2022, SemesterType::Two, false).await.unwrap();
    /// println!("{:?}", classes); // around 3s(depends on network environment)
    /// // [ClassGrade { ... }, ClassGrade { ... }]
    /// # })
    /// ```
    /// 상세 성적을 가져올 경우
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// # use std::sync::Arc;
    /// # use rusaint::USaintSession;
    /// # use rusaint::application::course_grades::{ model::CourseType, CourseGradesApplication };
    /// # use rusaint::model::SemesterType;
    /// # use rusaint::application::USaintClientBuilder;
    /// # let session = Arc::new(USaintSession::with_password("20212345", "password").await.unwrap());
    /// let mut app = USaintClientBuilder::new().session(session).build_into::<CourseGradesApplication>().await.unwrap();
    /// let classes = app.classes(CourseType::Bachelor, 2022, SemesterType::Two, true).await.unwrap();
    /// println!("{:?}", classes); // around 10s(depends on network environment)
    /// // [ClassGrade { ... }, ClassGrade { ... }]
    /// # })
    /// ```
    pub async fn classes(
        &mut self,
        course_type: CourseType,
        year: u32,
        semester: SemesterType,
        include_details: bool,
    ) -> Result<Vec<ClassGrade>, RusaintError> {
        {
            self.close_popups().await?;
            let parser = ElementParser::new(self.client.body());
            self.select_course(&parser, course_type).await?;
            self.select_semester(&parser, &year.to_string(), semester)
                .await?;
        }
        let parser = ElementParser::new(self.client.body());
        let class_grades: Vec<(Option<Event>, HashMap<String, String>)> = {
            let grade_table_body =
                parser.read(SapTableBodyCommand::new(Self::GRADE_BY_CLASSES_TABLE))?;
            let iter = grade_table_body.iter();
            iter.map(|row| {
                let btn_event = SapTableCellWrapper::from_def(&row[4], &parser)
                    .ok()
                    .and_then(|cell| {
                        if let Some(ElementDefWrapper::Button(btn)) = cell.content() {
                            parser.element_from_def(&btn).ok()?.press().ok()
                        } else {
                            None
                        }
                    });
                (btn_event, row)
            })
            .filter_map(|(btn_event, row)| {
                row.try_row_into::<HashMap<String, String>>(grade_table_body.header(), &parser)
                    .ok()
                    .map(|row| (btn_event, row))
            })
            .collect()
        };
        let mut ret: Vec<ClassGrade> = vec![];
        for (btn_event, values) in class_grades {
            let detail: Option<HashMap<String, f32>> = if let Some(btn_event) = btn_event {
                if include_details {
                    Some(self.class_detail_in_popup(btn_event).await?)
                } else {
                    None
                }
            } else {
                None
            };
            let parsed: Option<ClassGrade> = (|| {
                Some(ClassGrade::new(
                    year,
                    semester,
                    values["과목코드"].trim().to_owned(),
                    values["과목명"].trim().to_owned(),
                    values["과목학점"].parse().ok()?,
                    values["성적"].parse().ok()?,
                    values["등급"].trim().to_owned(),
                    values["교수명"].trim().to_owned(),
                    detail,
                ))
            })();
            if let Some(parsed) = parsed {
                ret.push(parsed);
            }
        }
        Ok(ret)
    }

    /// 주어진 수업의 상세 성적 정보를 가져옵니다. 만약 상세 성적이 음수라면, 성적이 비어 있다는 의미입니다.
    /// ### 예시
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// # use std::sync::Arc;
    /// # use rusaint::USaintSession;
    /// # use rusaint::model::SemesterType;
    /// # use rusaint::application::course_grades::{ model::CourseType, CourseGradesApplication };
    /// # use rusaint::application::USaintClientBuilder;
    /// # let session = Arc::new(USaintSession::with_password("20212345", "password").await.unwrap());
    /// let mut app = USaintClientBuilder::new().session(session).build_into::<CourseGradesApplication>().await.unwrap();
    /// let classes = app.classes(CourseType::Bachelor, 2022, SemesterType::Two, false).await.unwrap();
    /// let class = classes.iter().next().unwrap();
    /// let class_detail = app.class_detail(CourseType::Bachelor, 2022, SemesterType::Two, class.code()).await.unwrap();
    /// println!("{:?}", class_detail);
    /// // {"출석(20.000)": 20.0, "중간고사(30.000)": 30.0, "과제(20.000)": 20.0, "기말고사(30.000)": 28.0}
    /// # })
    /// ```
    pub async fn class_detail(
        &mut self,
        course_type: CourseType,
        year: u32,
        semester: SemesterType,
        code: &str,
    ) -> Result<HashMap<String, f32>, RusaintError> {
        let year = year.to_string();
        {
            self.close_popups().await?;
            let parser = ElementParser::new(self.client.body());
            self.select_course(&parser, course_type).await?;
            self.select_semester(&parser, &year, semester).await?;
        }
        let parser = ElementParser::new(self.client.body());
        let table = parser.read(SapTableBodyCommand::new(Self::GRADE_BY_CLASSES_TABLE))?;
        let Some(btn) = ({
            table
                .iter()
                .find(
                    |row| match SapTableCellWrapper::from_def(&row[8], &parser) {
                        Ok(cell) => {
                            if let Some(ElementDefWrapper::TextView(code_elem)) = cell.content() {
                                parser
                                    .element_from_def(&code_elem)
                                    .is_ok_and(|elem| elem.text() == code)
                            } else {
                                false
                            }
                        }
                        Err(_) => false,
                    },
                )
                .and_then(
                    |row| match SapTableCellWrapper::from_def(&row[4], &parser) {
                        Ok(cell) => {
                            if let Some(ElementDefWrapper::Button(btn)) = cell.content() {
                                parser.element_from_def(&btn).ok()?.press().ok()
                            } else {
                                None
                            }
                        }
                        Err(_) => None,
                    },
                )
        }) else {
            return Err(WebDynproError::from(ElementError::NoSuchData {
                element: Self::GRADE_BY_CLASSES_TABLE.id().to_string(),
                field: format!("details of class {code}"),
            }))?;
        };
        self.class_detail_in_popup(btn).await
    }

    fn body(&self) -> &Body {
        self.client.body()
    }
}

/// [`CourseGradesApplication`]에서 사용하는 데이터
pub mod model;

#[cfg(test)]
mod test {
    use crate::webdynpro::element::parser::ElementParser;
    use crate::{
        application::{USaintClientBuilder, course_grades::CourseGradesApplication},
        global_test_utils::get_session,
        webdynpro::element::{Element, layout::PopupWindow},
    };

    #[tokio::test]
    async fn close_popups() {
        let session = get_session().await.unwrap();
        let mut app = USaintClientBuilder::new()
            .session(session)
            .build_into::<CourseGradesApplication>()
            .await
            .unwrap();
        app.close_popups().await.unwrap();
        let popup_selector =
            scraper::Selector::parse(format!(r#"[ct="{}"]"#, PopupWindow::CONTROL_ID).as_str())
                .unwrap();
        let parser = ElementParser::new(app.client.body());
        let result = parser.document().select(&popup_selector).next().is_none();
        assert!(result);
    }
}
