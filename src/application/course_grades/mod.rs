use anyhow::Result;
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::{
    element_ref,
    session::USaintSession,
    webdynpro::{
        element::{
            combo_box::ComboBox,
            input_field::InputField,
            popup_window::PopupWindow,
            sap_table::{
                cell::{SapTableCell, SapTableCellWrapper},
                SapTable,
            },
            Element, ElementWrapper, SubElement,
        },
        error::ElementError,
        event::Event,
    },
};

use self::data::GradeSummary;

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
    element_ref!(
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
    element_ref!(
        PERIOD_YEAR: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69:VIW_MAIN.PERYR",
        PERIOD_SEMESTER: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69:VIW_MAIN.PERID",
        SPECIFIC_GRADE_SUMMARY_TABLE: SapTable<'a> = "ZCMB3W0017.ID_0001:VIW_MAIN.TABLE_1",
    );

    pub async fn new(session: Arc<USaintSession>) -> Result<CourseGrades> {
        Ok(CourseGrades(
            USaintApplication::with_session(Self::APP_NAME, session).await?,
        ))
    }

    pub fn grade_summary(&self) -> Result<Vec<GradeSummary>> {
        fn row_to_string(row: &Vec<SapTableCellWrapper>) -> Option<Vec<String>> {
            if row.len() > 0 && !row[0].is_empty_row() {
                Some(
                    row.iter()
                        .map(|val| {
                            let cell_content: Option<String> = match val {
                                SapTableCellWrapper::Normal(cell) => {
                                    let tv = cell.content();
                                    if let Some(ElementWrapper::TextView(tv)) = tv {
                                        Some(tv.text().to_owned())
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            };
                            cell_content.unwrap_or("".to_string())
                        })
                        .collect::<Vec<String>>(),
                )
            } else {
                None
            }
        }
        let table_elem = Self::GRADES_SUMMARY_TABLE.from_body(self.body())?;
        let table = table_elem.table().ok_or(ElementError::NoSuchElement)?;
        let iter = table.iter();
        let ret = iter.skip(1).filter_map(row_to_string).filter_map(|values| {
            fn parse_rank(value: String) -> Option<(u32, u32)> {
                let mut spl = value.split("/");
                let first: u32 = spl.next()?.parse().ok()?;
                let second: u32 = spl.next()?.parse().ok()?;
                Some((first, second))
            }
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
        webdynpro::element::{popup_window::PopupWindow, Element, ElementWrapper},
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
        app.load_placeholder().await.unwrap();
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
        let mut app = CourseGrades::new(session).await.unwrap();
        app.load_placeholder().await.unwrap();
        let summary = app.grade_summary().unwrap();
        println!("{:?}", summary);
        assert!(!summary.is_empty());
    }

    #[tokio::test]
    async fn examine_elements() {
        let session = get_session().await.unwrap();
        let mut app = CourseGrades::new(session).await.unwrap();
        app.load_placeholder().await.unwrap();
        let ct_selector = scraper::Selector::parse("[ct]").unwrap();
        for elem_ref in app.body().document().select(&ct_selector) {
            let elem = ElementWrapper::dyn_elem(elem_ref);
            if let Ok(elem) = elem {
                println!("{:?}", elem);
            }
        }
    }
}
