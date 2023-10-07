use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::{
    define_elements,
    model::SemesterType,
    session::USaintSession,
    webdynpro::{
        application::client::body::Body,
        element::{
            action::button::Button,
            complex::sap_table::{cell::SapTableCellWrapper, SapTable},
            layout::popup_window::PopupWindow,
            selection::combo_box::ComboBox,
            text::input_field::InputField,
            Element, ElementDef, ElementWrapper, SubElement,
        },
        error::{BodyError, ElementError, WebDynproError},
        event::Event,
    },
};

use self::model::{ClassGrade, GradeSummary, SemesterGrade};

use super::USaintApplication;

pub struct CourseGrades(USaintApplication);

impl Deref for CourseGrades {
    type Target = USaintApplication;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for CourseGrades {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[allow(unused)]
impl<'a> CourseGrades {
    const APP_NAME: &str = "ZCMB3W0017";

    // Elements for Grade Summaries
    define_elements!(
        // Grade summaries by semester
        GRADES_SUMMARY_TABLE: SapTable<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.TABLE",
        // Progress type
        PROGRESS_TYPE: ComboBox<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.PROGC_VAR",
        // Attempted Credits in Record
        ATTM_CRD1: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.ATTM_CRD1",
        // Earned Credits in Record
        EARN_CRD1: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.EARN_CRD1",
        // GPA in Record
        GT_GPA1: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.GT_GPA1",
        // Class GPA in Record
        CGPA1: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.CGPA1",
        // Average Points in Record
        AVG1: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.AVG1",
        // Credits earned in P/F Classes in Record
        PF_EARN_CRD: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.PF_EARN_CRD",
        // Attempted Credits in Certificate
        ATTM_CRD2: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.ATTM_CRD2",
        // Earned Credits in Certificate
        EARN_CRD2: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.EARN_CRD2",
        // GPA in Certificate
        GT_GPA2: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.GT_GPA2",
        // Class GPA in Certificate
        CGPA2: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.CGPA2",
        // Average Points in Certificate
        AVG2: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.AVG2",
        // Credits earned in P/F Classes in Certificate
        PF_EARN_CRD1: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.T_PF_EARN_CRD1",
    );

    // Elements for Class Grades
    define_elements!(
        PERIOD_YEAR: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69:VIW_MAIN.PERYR",
        PERIOD_SEMESTER: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69:VIW_MAIN.PERID",
        GRADE_BY_CLASSES_TABLE: SapTable<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.TABLE_1",
    );

    pub async fn new(session: Arc<USaintSession>) -> Result<CourseGrades, WebDynproError> {
        Ok(CourseGrades(
            USaintApplication::with_session(Self::APP_NAME, session).await?,
        ))
    }

    async fn close_popups(&mut self) -> Result<(), WebDynproError> {
        fn make_close_event(app: &CourseGrades) -> Option<Event> {
            let body = app.body();
            let popup_selector =
                scraper::Selector::parse(format!(r#"[ct="{}"]"#, PopupWindow::CONTROL_ID).as_str())
                    .unwrap();
            let mut popup_iter = body.document().select(&popup_selector);
            popup_iter.next().and_then(|elem| {
                let elem_wrapped = ElementWrapper::dyn_elem(elem).ok()?;
                if let ElementWrapper::PopupWindow(popup) = elem_wrapped {
                    popup.close().ok()
                } else {
                    None
                }
            })
        }
        while let Some(event) = make_close_event(self) {
            self.send_events(vec![event]).await?;
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

    async fn select_semester(
        &mut self,
        year: &str,
        semester: SemesterType,
    ) -> Result<(), WebDynproError> {
        let select = {
            let semester = Self::semester_to_key(semester);
            let mut vec = Vec::with_capacity(2);
            let year_combobox = Self::PERIOD_YEAR.from_body(self.body())?;
            if (|| Some(year_combobox.lsdata()?.key().as_ref()?.as_str()))() != Some(year) {
                vec.push(year_combobox.select(&year.to_string(), false)?);
            }
            let semester_combobox = Self::PERIOD_SEMESTER.from_body(self.body())?;
            if (|| Some(semester_combobox.lsdata()?.key().as_ref()?.as_str()))() != Some(semester) {
                vec.push(semester_combobox.select(semester, false)?);
            }
            Result::<Vec<Event>, WebDynproError>::Ok(vec)
        }?;
        if !select.is_empty() {
            self.send_events(select).await
        } else {
            Ok(())
        }
    }

    fn row_to_string(row: &Vec<SapTableCellWrapper>) -> Option<Vec<String>> {
        if row.len() == 0 || row[0].is_empty_row() {
            return None;
        };
        Some(
            row.iter()
                .map(|val| {
                    match val.content() {
                        Some(ElementWrapper::TextView(tv)) => Some(tv.text().to_owned()),
                        Some(ElementWrapper::Caption(cap)) => Some(cap.text().to_owned()),
                        _ => None,
                    }
                    .unwrap_or("".to_string())
                })
                .collect::<Vec<String>>(),
        )
    }

    fn value_as_f32(field: InputField<'_>) -> Result<f32, WebDynproError> {
        let Some(value) = field.lsdata().and_then(|lsdata| lsdata.value().as_ref()) else {
            return Err(ElementError::NoSuchData { element: field.id().to_string(), field: "value1".to_string() })?;
        };
        Ok(value.parse::<f32>().or(Err(ElementError::InvalidContent {
            element: field.id().to_string(),
            content: "value1(not an correct f32)".to_string(),
        }))?)
    }

    pub fn recorded_summary(&self) -> Result<GradeSummary, WebDynproError> {
        let body = self.body();
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

    pub fn certificated_summary(&self) -> Result<GradeSummary, WebDynproError> {
        let body = self.body();
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

    pub fn semesters(&self) -> Result<Vec<SemesterGrade>, WebDynproError> {
        fn parse_rank(value: String) -> Option<(u32, u32)> {
            let mut spl = value.split("/");
            let first: u32 = spl.next()?.parse().ok()?;
            let second: u32 = spl.next()?.parse().ok()?;
            Some((first, second))
        }

        let table_elem = Self::GRADES_SUMMARY_TABLE.from_body(self.body())?;
        let table = table_elem.table().ok_or(ElementError::NoSuchContent {
            element: table_elem.id().to_string(),
            content: "table".to_string(),
        })?;
        let ret = table
            .iter()
            .skip(1)
            .filter_map(Self::row_to_string)
            .filter_map(|values| {
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
            });
        Ok(ret.collect())
    }

    async fn class_detail_in_popup<'f>(
        &mut self,
        open_button: ElementDef<'f, Button<'f>>,
    ) -> Result<HashMap<String, f32>, WebDynproError> {
        fn parse_table_in_popup(body: &Body) -> Result<Vec<(String, f32)>, WebDynproError> {
            let table_inside_popup_selector =
                scraper::Selector::parse(r#"[ct="PW"] [ct="ST"]"#).unwrap();
            let mut table_inside_popup = body.document().select(&table_inside_popup_selector);
            let table_ref = table_inside_popup
                .next()
                .ok_or(BodyError::NoSuchElement("Table in popup".to_string()))?;
            let table_elem: SapTable<'_> = ElementWrapper::dyn_elem(table_ref)?.try_into()?;
            let zip = (|| {
                let mut iter = table_elem.table()?.iter();
                let head_str = CourseGrades::row_to_string(iter.next()?)?;
                let row_str = CourseGrades::row_to_string(iter.next()?)?;
                Some(head_str.into_iter().zip(row_str.into_iter()))
            })()
            .ok_or(ElementError::InvalidContent {
                element: table_elem.id().to_string(),
                content: "header and first row".to_string(),
            })?;
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
        let event = { open_button.from_body(self.body())?.press() }?;
        self.send_events(vec![event]).await?;
        let table = parse_table_in_popup(self.body())?;
        self.close_popups().await?;
        Ok(HashMap::from_iter(table))
    }

    pub async fn class_detail(
        &mut self,
        year: &str,
        semester: SemesterType,
        code: &str,
    ) -> Result<HashMap<String, f32>, WebDynproError> {
        self.select_semester(year, semester).await?;
        let table_elem = Self::GRADE_BY_CLASSES_TABLE.from_body(self.body())?;
        let Some(table) = table_elem.table()  else {
            return Err(ElementError::NoSuchContent { element: table_elem.id().to_string(), content: "Table body".to_string() })?;
        };
        let Some(btn) = ({
            table.iter().find(|row| {
                if let Some(ElementWrapper::TextView(code_elem)) = row[3].content() {
                    code_elem.text() == code
                } else {
                    false
                }
            }).and_then(|row| {
                if let Some(ElementWrapper::Button(btn)) = row[10].content() {
                    Some(ElementDef::<'_, Button<'_>>::new_dynamic(btn.id().to_owned()))
                } else {
                    None
                }
            })
        }) else {
            return Err(ElementError::NoSuchData { element: table_elem.id().to_string(), field: format!("details of class {}", code) })?;
        };
        self.class_detail_in_popup(btn).await
    }

    pub async fn classes(
        &mut self,
        year: &str,
        semester: SemesterType,
        include_details: bool,
    ) -> Result<Vec<ClassGrade>, WebDynproError> {
        self.close_popups().await?;
        self.select_semester(year, semester).await?;
        let class_grades: Vec<(Option<String>, Vec<String>)> = {
            let grade_table_elem = Self::GRADE_BY_CLASSES_TABLE.from_body(self.body())?;
            let iter = grade_table_elem
                .table()
                .ok_or(ElementError::NoSuchContent {
                    element: grade_table_elem.id().to_string(),
                    content: "Table body".to_string(),
                })?
                .iter()
                .skip(1);
            iter.map(|row| {
                let btn_cell = &row[10];
                let btn_id = if let Some(ElementWrapper::Button(btn)) = btn_cell.content() {
                    Some(btn.id().to_owned())
                } else {
                    None
                };
                (btn_id, row)
            })
            .filter_map(|(btn_id, row)| {
                Self::row_to_string(row).and_then(|row| Some((btn_id, row)))
            })
            .collect()
        };
        let mut ret: Vec<ClassGrade> = vec![];
        for (btn_id, values) in class_grades {
            let detail: Option<HashMap<String, f32>> = if let Some(btn_id) = btn_id {
                if include_details {
                    let btn_def = ElementDef::<Button<'_>>::new_dynamic(btn_id);
                    Some(self.class_detail_in_popup(btn_def).await?)
                } else {
                    None
                }
            } else {
                None
            };
            let parsed: Option<ClassGrade> = (|| {
                Some(ClassGrade::new(
                    values[1].trim().to_owned(),
                    values[2].trim().to_owned(),
                    values[3].trim().to_owned(),
                    values[4].trim().to_owned(),
                    values[5].parse().ok()?,
                    values[6].parse().ok()?,
                    values[7].trim().to_owned(),
                    values[8].trim().to_owned(),
                    detail,
                ))
            })();
            if let Some(parsed) = parsed {
                ret.push(parsed);
            }
        }
        Ok(ret)
    }
}

impl<'a> SapTableCellWrapper<'a> {
    fn is_empty_row(&self) -> bool {
        match self {
            SapTableCellWrapper::Normal(cell) => cell
                .lsdata()
                .is_some_and(|data| data.cell_type().as_ref().is_some_and(|s| s == "EMPTYROW")),
            SapTableCellWrapper::Header(cell) => cell.lsdata().is_some_and(|data| {
                data.header_cell_type()
                    .as_ref()
                    .is_some_and(|s| s == "EMPTYROW")
            }),
            SapTableCellWrapper::Selection(cell) => cell
                .lsdata()
                .is_some_and(|data| data.cell_type().as_ref().is_some_and(|s| s == "EMPTYROW")),
            _ => false,
        }
    }
}

pub mod model;

#[cfg(test)]
mod test {
    use anyhow::{Error, Result};
    use std::sync::{Arc, OnceLock};

    use crate::{
        application::course_grades::CourseGrades,
        session::USaintSession,
        webdynpro::element::{layout::popup_window::PopupWindow, Element},
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
        let mut app = CourseGrades::new(session).await.unwrap();
        app.close_popups().await.unwrap();
        let body = app.body();
        let popup_selector =
            scraper::Selector::parse(format!(r#"[ct="{}"]"#, PopupWindow::CONTROL_ID).as_str())
                .unwrap();
        let mut result = body.document().select(&popup_selector);
        assert!(result.next().is_none());
    }
}
