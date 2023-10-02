use anyhow::Result;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::{
    define_elements,
    session::USaintSession,
    webdynpro::{
        application::client::body::Body,
        element::{
            button::Button,
            combo_box::ComboBox,
            input_field::InputField,
            popup_window::PopupWindow,
            sap_table::{cell::SapTableCellWrapper, SapTable},
            Element, ElementDef, ElementWrapper, SubElement,
        },
        error::ElementError,
        event::Event,
    },
};

use self::data::{ClassGrade, GradeSummary};

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

    // Elements for General Grades
    define_elements!(
        GRADES_SUMMARY_TABLE: SapTable<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.TABLE",
        GRADE_TYPE: ComboBox<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.PROGC_VAR",
        ATTM_GRADE: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.ATTM_CRD1",
        EARN_GRADE: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.EARN_CRD1",
        GT_GPA: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.GT_GPA1",
        CGPA: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.CGPA1",
        AVG: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.AVG1",
        PF_EARN: InputField<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.PF_EARN_CRD",
    );

    // Elements for Specific Grades
    define_elements!(
        PERIOD_YEAR: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69:VIW_MAIN.PERYR",
        PERIOD_SEMESTER: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69:VIW_MAIN.PERID",
        SPECIFIC_GRADE_SUMMARY_TABLE: SapTable<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.TABLE_1",
    );

    pub async fn new(session: Arc<USaintSession>) -> Result<CourseGrades> {
        Ok(CourseGrades(
            USaintApplication::with_session(Self::APP_NAME, session).await?,
        ))
    }

    pub async fn close_popups(&mut self) -> Result<()> {
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

    async fn select_year_semester(&mut self, year: &str, semester: &str) -> Result<()> {
        let select = {
            let mut vec = Vec::with_capacity(2);
            let year_combobox = Self::PERIOD_YEAR.from_body(self.body())?;
            if (|| Some(year_combobox.lsdata()?.key().as_ref()?.as_str()))() != Some(year) {
                vec.push(year_combobox.select(&year.to_string(), false)?);
            }
            let semester_combobox = Self::PERIOD_SEMESTER.from_body(self.body())?;
            if (|| Some(semester_combobox.lsdata()?.key().as_ref()?.as_str()))() != Some(semester) {
                vec.push(semester_combobox.select(semester, false)?);
            }
            Result::<Vec<Event>>::Ok(vec)
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

    pub fn grade_summary(&self) -> Result<Vec<GradeSummary>> {
        fn parse_rank(value: String) -> Option<(u32, u32)> {
            let mut spl = value.split("/");
            let first: u32 = spl.next()?.parse().ok()?;
            let second: u32 = spl.next()?.parse().ok()?;
            Some((first, second))
        }

        let table_elem = Self::GRADES_SUMMARY_TABLE.from_body(self.body())?;
        let table = table_elem.table().ok_or(ElementError::NoSuchElement)?;
        let ret = table
            .iter()
            .skip(1)
            .filter_map(Self::row_to_string)
            .filter_map(|values| {
                Some(GradeSummary::new(
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

    async fn grade_detail_in_popup<'f>(
        &mut self,
        open_button: ElementDef<'f, Button<'f>>,
    ) -> Result<HashMap<String, f32>> {
        fn parse_table_in_popup(body: &Body) -> Result<Vec<(String, f32)>> {
            let table_inside_popup_selector =
                scraper::Selector::parse(r#"[ct="PW"] [ct="ST"]"#).unwrap();
            let mut table_inside_popup = body.document().select(&table_inside_popup_selector);
            let table_ref = table_inside_popup
                .next()
                .ok_or(ElementError::NoSuchElement)?;
            let table_elem: SapTable<'_> = ElementWrapper::dyn_elem(table_ref)?.try_into()?;
            let zip = (|| {
                let mut iter = table_elem.table()?.iter();
                let head_str = CourseGrades::row_to_string(iter.next()?)?;
                let row_str = CourseGrades::row_to_string(iter.next()?)?;
                Some(head_str.into_iter().zip(row_str.into_iter()))
            })()
            .ok_or(ElementError::InvalidBody)?;
            zip.skip(4)
                .map(|(key, val)| Ok((key, val.trim().parse::<f32>()?)))
                .collect::<Result<Vec<(String, f32)>>>()
        };
        let event = { open_button.from_body(self.body())?.press() }?;
        self.send_events(vec![event]).await?;
        let table = parse_table_in_popup(self.body())?;
        self.close_popups().await?;
        Ok(HashMap::from_iter(table))
    }

    pub async fn grade_detail(
        &mut self,
        year: &str,
        semester: &str,
        include_details: bool,
    ) -> Result<Vec<ClassGrade>> {
        self.close_popups().await?;
        self.select_year_semester(year, semester).await?;
        let class_grades: Vec<(Option<String>, Vec<String>)> = {
            let grade_table_elem = Self::SPECIFIC_GRADE_SUMMARY_TABLE.from_body(self.body())?;
            let iter = grade_table_elem
                .table()
                .ok_or(ElementError::NoSuchElement)?
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
                    Some(self.grade_detail_in_popup(btn_def).await?)
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

pub mod data;

#[cfg(test)]
mod test {
    use anyhow::{Error, Result};
    use std::sync::{Arc, OnceLock};

    use crate::{
        application::course_grades::CourseGrades,
        session::USaintSession,
        webdynpro::element::{popup_window::PopupWindow, Element},
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

    #[tokio::test]
    async fn read_grades() {
        let session = get_session().await.unwrap();
        let app = CourseGrades::new(session).await.unwrap();
        let summary = app.grade_summary().unwrap();
        println!("{:?}", summary);
        assert!(!summary.is_empty());
    }

    #[tokio::test]
    async fn grade_detail() {
        let session = get_session().await.unwrap();
        let mut app = CourseGrades::new(session).await.unwrap();
        let detail = app.grade_detail("2022", "092", true).await.unwrap();
        println!("{:?}", detail);
        assert!(!detail.is_empty());
    }
}
