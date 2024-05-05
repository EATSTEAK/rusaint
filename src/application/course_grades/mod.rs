use std::collections::HashMap;

use crate::{
    define_elements,
    model::SemesterType,
    webdynpro::{
        client::body::Body,
        command::element::{
            action::ButtonPressCommand,
            selection::{ComboBoxSelectCommand, ReadComboBoxLSDataCommand},
        },
        element::{
            action::ButtonDef,
            complex::sap_table::{
                cell::{SapTableCell, SapTableCellWrapper},
                property::SapTableCellType,
                SapTable, SapTableRow,
            },
            definition::ElementDefinition,
            layout::PopupWindow,
            selection::ComboBox,
            sub::SubElement,
            text::InputField,
            Element, ElementDefWrapper, ElementWrapper,
        },
        error::{BodyError, ElementError, WebDynproError},
        event::Event,
    },
    RusaintError,
};

use self::model::{ClassGrade, CourseType, GradeSummary, SemesterGrade};

use super::{USaintApplication, USaintClient};

/// [학생 성적 조회](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMB3W0017)
pub struct CourseGrades {
    client: USaintClient,
}

impl USaintApplication for CourseGrades {
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
impl<'a> CourseGrades {
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
        fn make_close_event(app: &CourseGrades) -> Option<Event> {
            let body = app.client.body();
            let popup_selector =
                scraper::Selector::parse(format!(r#"[ct="{}"]"#, PopupWindow::CONTROL_ID).as_str())
                    .unwrap();
            let mut popup_iter = body.document().select(&popup_selector);
            popup_iter.next().and_then(|elem| {
                let elem_wrapped = ElementWrapper::dyn_element(elem).ok()?;
                if let ElementWrapper::PopupWindow(popup) = elem_wrapped {
                    popup.close().ok()
                } else {
                    None
                }
            })
        }
        while let Some(event) = make_close_event(self) {
            self.client.process_event(false, event).await?;
        }
        Ok(())
    }

    fn semester_to_key(period: SemesterType) -> &'static str {
        match period {
            SemesterType::One => "090",
            SemesterType::Summer => "091",
            SemesterType::Two => "092",
            SemesterType::Winter => "0923",
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

    async fn select_course(&mut self, course: CourseType) -> Result<(), WebDynproError> {
        let course = Self::course_type_to_key(course);
        let combobox_lsdata = self
            .client
            .send(ReadComboBoxLSDataCommand::new(Self::PROGRESS_TYPE))
            .await?;
        if (|| Some(combobox_lsdata.key()?.as_str()))() != Some(course) {
            self.client
                .send(ComboBoxSelectCommand::new(
                    Self::PROGRESS_TYPE,
                    course,
                    false,
                ))
                .await?;
        }
        Ok(())
    }

    async fn select_semester(
        &mut self,
        year: &str,
        semester: SemesterType,
    ) -> Result<(), WebDynproError> {
        let semester = Self::semester_to_key(semester);
        let year_combobox_lsdata = self
            .client
            .send(ReadComboBoxLSDataCommand::new(Self::PERIOD_YEAR))
            .await?;
        let semester_combobox_lsdata = self
            .client
            .send(ReadComboBoxLSDataCommand::new(Self::PERIOD_SEMESTER))
            .await?;
        if (|| Some(year_combobox_lsdata.key()?.as_str()))() != Some(year) {
            self.client
                .send(ComboBoxSelectCommand::new(Self::PERIOD_YEAR, &year, false))
                .await?;
        }
        if (|| Some(semester_combobox_lsdata.key()?.as_str()))() != Some(semester) {
            self.client
                .send(ComboBoxSelectCommand::new(
                    Self::PERIOD_SEMESTER,
                    semester,
                    false,
                ))
                .await?;
        }
        Ok(())
    }

    fn value_as_f32(field: InputField<'_>) -> Result<f32, WebDynproError> {
        let Some(value) = field.lsdata().value() else {
            return Err(ElementError::NoSuchData {
                element: field.id().to_string(),
                field: "value1".to_string(),
            })?;
        };
        Ok(value.parse::<f32>().or(Err(ElementError::InvalidContent {
            element: field.id().to_string(),
            content: "value1(not an correct f32)".to_string(),
        }))?)
    }

    /// 전체 학기의 학적부 평점 정보를 가져옵니다.
    /// ### 예시
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// # use std::sync::Arc;
    /// # use rusaint::USaintSession;
    /// # use rusaint::application::course_grades::{ model::CourseType, CourseGrades };
    /// # use rusaint::application::USaintClientBuilder;
    /// # let session = Arc::new(USaintSession::with_password("20212345", "password").await.unwrap());
    /// let mut app = USaintClientBuilder::new().session(session).build_into::<CourseGrades>().await.unwrap();
    /// let summary = app.recorded_summary(CourseType::Bachelor).await.unwrap();
    /// println!("{:?}", summary);
    /// // GradeSummary { ... }
    /// # })
    /// ```
    pub async fn recorded_summary(
        &mut self,
        course_type: CourseType,
    ) -> Result<GradeSummary, WebDynproError> {
        self.close_popups().await?;
        self.select_course(course_type).await?;
        let body = self.client.body();
        let attempted_credits = Self::value_as_f32(Self::ATTM_CRD1.from_body(body)?)?;
        let earned_credits = Self::value_as_f32(Self::EARN_CRD1.from_body(body)?)?;
        let gpa = Self::value_as_f32(Self::GT_GPA1.from_body(body)?)?;
        let cgpa = Self::value_as_f32(Self::CGPA1.from_body(body)?)?;
        let avg = Self::value_as_f32(Self::AVG1.from_body(body)?)?;
        let pf_earned_credits = Self::value_as_f32(Self::PF_EARN_CRD.from_body(body)?)?;
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
    /// # use rusaint::application::course_grades::{ model::CourseType, CourseGrades };
    /// # let session = Arc::new(USaintSession::with_password("20212345", "password").await.unwrap());
    /// let mut app = USaintClientBuilder::new().session(session).build_into::<CourseGrades>().await.unwrap();
    /// let summary = app.certificated_summary(CourseType::Bachelor).await.unwrap();
    /// println!("{:?}", summary);
    /// // GradeSummary { ... }
    /// # })
    /// ```
    pub async fn certificated_summary(
        &mut self,
        course_type: CourseType,
    ) -> Result<GradeSummary, WebDynproError> {
        self.close_popups().await?;
        self.select_course(course_type).await?;
        let body = self.client.body();
        let attempted_credits = Self::value_as_f32(Self::ATTM_CRD2.from_body(body)?)?;
        let earned_credits = Self::value_as_f32(Self::EARN_CRD2.from_body(body)?)?;
        let gpa = Self::value_as_f32(Self::GT_GPA2.from_body(body)?)?;
        let cgpa = Self::value_as_f32(Self::CGPA2.from_body(body)?)?;
        let avg = Self::value_as_f32(Self::AVG2.from_body(body)?)?;
        let pf_earned_credits = Self::value_as_f32(Self::PF_EARN_CRD1.from_body(body)?)?;
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
    /// # use rusaint::application::course_grades::{ model::CourseType, CourseGrades };
    /// # use rusaint::application::USaintClientBuilder;
    /// # let session = Arc::new(USaintSession::with_password("20212345", "password").await.unwrap());
    /// let mut app = USaintClientBuilder::new().session(session).build_into::<CourseGrades>().await.unwrap();
    /// let semesters = app.semesters(CourseType::Bachelor).await.unwrap();
    /// println!("{:?}", semesters);
    /// // [SemesterGrade { ... }, SemesterGrade { ... }]
    /// # })
    /// ```
    pub async fn semesters(
        &mut self,
        course_type: CourseType,
    ) -> Result<Vec<SemesterGrade>, WebDynproError> {
        fn parse_rank(value: String) -> Option<(u32, u32)> {
            let mut spl = value.split("/");
            let first: u32 = spl.next()?.parse().ok()?;
            let second: u32 = spl.next()?.parse().ok()?;
            Some((first, second))
        }
        self.close_popups().await?;
        self.select_course(course_type).await?;

        let table_elem = Self::GRADES_SUMMARY_TABLE.from_body(self.client.body())?;
        let table = table_elem.table()?;
        let ret = table
            .try_table_into::<Vec<String>>(self.client.body())?
            .into_iter()
            .filter_map(|values| {
                if values.len() == 14 {
                    Some(SemesterGrade::new(
                        values[1].parse().ok()?,
                        values[2].clone(),
                        values[3].parse().ok()?,
                        values[4].parse().ok()?,
                        values[5].parse().ok()?,
                        values[6].parse().ok()?,
                        values[7].parse().ok()?,
                        values[8].parse().ok()?,
                        parse_rank(values[9].clone())?,
                        parse_rank(values[10].clone())?,
                        !values[11].trim().is_empty(),
                        !values[12].trim().is_empty(),
                        !values[13].trim().is_empty(),
                    ))
                } else {
                    Some(SemesterGrade::new(
                        values[1].parse().ok()?,
                        values[2].clone(),
                        values[3].parse().ok()?,
                        values[4].parse().ok()?,
                        values[5].parse().ok()?,
                        values[6].parse().ok()?,
                        values[7].parse().ok()?,
                        values[8].parse().ok()?,
                        parse_rank(values[9].clone())?,
                        parse_rank(values[10].clone())?,
                        false,
                        !values[11].trim().is_empty(),
                        !values[12].trim().is_empty(),
                    ))
                }
            });
        Ok(ret.collect())
    }

    async fn class_detail_in_popup(
        &mut self,
        open_button: ButtonDef,
    ) -> Result<HashMap<String, f32>, WebDynproError> {
        self.client
            .send(ButtonPressCommand::new(open_button))
            .await?;

        let parse_table_in_popup = |body: &Body| -> Result<Vec<(String, f32)>, WebDynproError> {
            let table_inside_popup_selector =
                scraper::Selector::parse(r#"[ct="PW"] [ct="ST"]"#).unwrap();
            let mut table_inside_popup = body.document().select(&table_inside_popup_selector);
            let table_ref = table_inside_popup
                .next()
                .ok_or(BodyError::NoSuchElement("Table in popup".to_string()))?;
            let table_elem: SapTable<'_> = ElementWrapper::dyn_element(table_ref)?.try_into()?;
            let zip = table_elem
                .table()?
                .iter()
                .next()
                .ok_or(ElementError::InvalidContent {
                    element: table_elem.id().to_string(),
                    content: "header and first row".to_string(),
                })?
                .try_row_into::<Vec<(String, String)>>(body)?
                .into_iter();
            zip.skip(4)
                .map(|(key, val)| {
                    Ok((
                        key,
                        val.trim()
                            .parse::<f32>()
                            .or(Err(ElementError::InvalidContent {
                                element: table_elem.id().to_string(),
                                content: "(not an correct f32)".to_string(),
                            }))?,
                    ))
                })
                .collect::<Result<Vec<(String, f32)>, WebDynproError>>()
        };
        let table = parse_table_in_popup(self.client.body())?;
        self.close_popups().await?;
        Ok(HashMap::from_iter(table))
    }

    /// 주어진 학기의 수업별 성적을 가져옵니다. `include_details`가 `true`인 경우 수업의 상세 성적도 가져옵니다.
    /// 수업의 상세 성적까지 가져올 경우 상세 성적이 있는 수업의 수 만큼 서버에 요청을 보내므로 반드시 상세 성적도 한번에 가져와야 할 때에만 사용하십시오.
    ///
    /// 수업 성적을 가져온 이후 상세 성적 또한 가져오려면 `[class_detail()]`함수를 이용하십시오.
    /// ### 예시
    /// 상세 성적을 가져오지 않을 경우
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// # use std::sync::Arc;
    /// # use rusaint::USaintSession;
    /// # use rusaint::application::course_grades::{ model::CourseType, CourseGrades };
    /// # use rusaint::application::USaintClientBuilder;
    /// # use rusaint::model::SemesterType;
    /// # let session = Arc::new(USaintSession::with_password("20212345", "password").await.unwrap());
    /// let mut app = USaintClientBuilder::new().session(session).build_into::<CourseGrades>().await.unwrap();
    /// let classes = app.classes(CourseType::Bachelor, "2022", SemesterType::Two, false).await.unwrap();
    /// println!("{:?}", classes); // around 3s(depends on network environment)
    /// // [ClassGrade { ... }, ClassGrade { ... }]
    /// # })
    /// ```
    /// 상세 성적을 가져올 경우
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// # use std::sync::Arc;
    /// # use rusaint::USaintSession;
    /// # use rusaint::application::course_grades::{ model::CourseType, CourseGrades };
    /// # use rusaint::model::SemesterType;
    /// # use rusaint::application::USaintClientBuilder;
    /// # let session = Arc::new(USaintSession::with_password("20212345", "password").await.unwrap());
    /// let mut app = USaintClientBuilder::new().session(session).build_into::<CourseGrades>().await.unwrap();
    /// let classes = app.classes(CourseType::Bachelor, "2022", SemesterType::Two, true).await.unwrap();
    /// println!("{:?}", classes); // around 10s(depends on network environment)
    /// // [ClassGrade { ... }, ClassGrade { ... }]
    /// # })
    /// ```
    pub async fn classes(
        &mut self,
        course_type: CourseType,
        year: &str,
        semester: SemesterType,
        include_details: bool,
    ) -> Result<Vec<ClassGrade>, WebDynproError> {
        self.close_popups().await?;
        self.select_course(course_type).await?;
        self.select_semester(year, semester).await?;
        let class_grades: Vec<(Option<String>, Vec<String>)> = {
            let grade_table_elem = Self::GRADE_BY_CLASSES_TABLE.from_body(self.client.body())?;
            let iter = grade_table_elem.table()?.iter();
            iter.map(|row| {
                let btn_id = row[4]
                    .clone()
                    .from_body(self.client.body())
                    .ok()
                    .and_then(|cell| {
                        if let Some(ElementDefWrapper::Button(btn)) = cell.content() {
                            Some(btn.id().to_owned())
                        } else {
                            None
                        }
                    });
                (btn_id, row)
            })
            .filter_map(|(btn_id, row)| {
                row.try_row_into::<Vec<String>>(self.client.body())
                    .ok()
                    .and_then(|row| Some((btn_id, row)))
            })
            .collect()
        };
        let mut ret: Vec<ClassGrade> = vec![];
        for (btn_id, values) in class_grades {
            let detail: Option<HashMap<String, f32>> = if let Some(btn_id) = btn_id {
                if include_details {
                    let btn_def = ButtonDef::new_dynamic(btn_id);
                    Some(self.class_detail_in_popup(btn_def).await?)
                } else {
                    None
                }
            } else {
                None
            };
            let parsed: Option<ClassGrade> = (|| {
                Some(ClassGrade::new(
                    year.to_owned(),
                    semester.to_string(),
                    values[8].trim().to_owned(),
                    values[3].trim().to_owned(),
                    values[5].parse().ok()?,
                    values[1].parse().ok()?,
                    values[2].trim().to_owned(),
                    values[6].trim().to_owned(),
                    detail,
                ))
            })();
            if let Some(parsed) = parsed {
                ret.push(parsed);
            }
        }
        Ok(ret)
    }

    /// 주어진 수업의 상세 성적 정보를 가져옵니다.
    /// ### 예시
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// # use std::sync::Arc;
    /// # use rusaint::USaintSession;
    /// # use rusaint::model::SemesterType;
    /// # use rusaint::application::course_grades::{ model::CourseType, CourseGrades };
    /// # use rusaint::application::USaintClientBuilder;
    /// # let session = Arc::new(USaintSession::with_password("20212345", "password").await.unwrap());
    /// let mut app = USaintClientBuilder::new().session(session).build_into::<CourseGrades>().await.unwrap();
    /// let classes = app.classes(CourseType::Bachelor, "2022", SemesterType::Two, false).await.unwrap();
    /// let class = classes.iter().next().unwrap();
    /// let class_detail = app.class_detail(CourseType::Bachelor, "2022", SemesterType::Two, class.code()).await.unwrap();
    /// println!("{:?}", class_detail);
    /// // {"출석(20.000)": 20.0, "중간고사(30.000)": 30.0, "과제(20.000)": 20.0, "기말고사(30.000)": 28.0}
    /// # })
    /// ```
    pub async fn class_detail(
        &mut self,
        course_type: CourseType,
        year: &str,
        semester: SemesterType,
        code: &str,
    ) -> Result<HashMap<String, f32>, WebDynproError> {
        self.close_popups().await?;
        self.select_course(course_type).await?;
        self.select_semester(year, semester).await?;
        let table_elem = Self::GRADE_BY_CLASSES_TABLE.from_body(self.client.body())?;
        let table = table_elem.table()?;
        let Some(btn) = ({
            table
                .iter()
                .find(|row| match row[8].clone().from_body(self.client.body()) {
                    Ok(cell) => {
                        if let Some(ElementDefWrapper::TextView(code_elem)) = cell.content() {
                            code_elem
                                .from_body(self.client.body())
                                .is_ok_and(|elem| elem.text() == code)
                        } else {
                            false
                        }
                    }
                    Err(_) => false,
                })
                .and_then(|row| match row[4].clone().from_body(self.client.body()) {
                    Ok(cell) => {
                        if let Some(ElementDefWrapper::Button(btn)) = cell.content() {
                            Some(ButtonDef::new_dynamic(btn.id().to_owned()))
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                })
        }) else {
            return Err(ElementError::NoSuchData {
                element: table_elem.id().to_string(),
                field: format!("details of class {}", code),
            })?;
        };
        self.class_detail_in_popup(btn).await
    }
}

// TODO: Implement empty row check in SapTableRow struct
impl<'a> SapTableCellWrapper<'a> {
    fn is_empty_row(&self) -> bool {
        match self {
            SapTableCellWrapper::Normal(cell) => cell
                .lsdata()
                .cell_type()
                .is_some_and(|s| matches!(s, SapTableCellType::EmptyRow)),
            SapTableCellWrapper::Header(_cell) => false,
            SapTableCellWrapper::Selection(cell) => cell
                .lsdata()
                .cell_type()
                .is_some_and(|s| matches!(s, SapTableCellType::EmptyRow)),
            _ => false,
        }
    }
}

/// [`CourseGrades`]에서 사용하는 데이터
pub mod model;

#[cfg(test)]
mod test {
    use anyhow::{Error, Result};
    use std::sync::{Arc, OnceLock};

    use crate::{
        application::{course_grades::CourseGrades, USaintClientBuilder},
        session::USaintSession,
        webdynpro::element::{layout::PopupWindow, Element},
    };
    use dotenv::dotenv;

    static SESSION: OnceLock<Arc<USaintSession>> = OnceLock::new();

    async fn get_session() -> Result<Arc<USaintSession>> {
        if let Some(session) = SESSION.get() {
            Ok(session.to_owned())
        } else {
            dotenv().ok();
            let id = std::env::var("SSO_ID").unwrap();
            let password = std::env::var("SSO_PASSWORD").unwrap();
            let session = USaintSession::with_password(&id, &password).await?;
            let _ = SESSION.set(Arc::new(session));
            SESSION
                .get()
                .and_then(|arc| Some(arc.to_owned()))
                .ok_or(Error::msg("Session is not initsiated"))
        }
    }

    #[tokio::test]
    async fn close_popups() {
        let session = get_session().await.unwrap();
        let mut app = USaintClientBuilder::new()
            .session(session)
            .build_into::<CourseGrades>()
            .await
            .unwrap();
        app.close_popups().await.unwrap();
        let body = app.client.body();
        let popup_selector =
            scraper::Selector::parse(format!(r#"[ct="{}"]"#, PopupWindow::CONTROL_ID).as_str())
                .unwrap();
        let mut result = body.document().select(&popup_selector);
        assert!(result.next().is_none());
    }
}
