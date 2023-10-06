use anyhow::{Error, Result};
use std::sync::{Arc, OnceLock};

use dotenv::dotenv;
use rusaint::{application::CourseGrades, session::USaintSession};
use serial_test::serial;

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
#[serial]
async fn summary() {
    let session = get_session().await.unwrap();
    let app = CourseGrades::new(session).await.unwrap();
    let summary = app.summary().unwrap();
    println!("{:?}", summary);
}
#[tokio::test]
#[serial]
async fn semesters() {
    let session = get_session().await.unwrap();
    let app = CourseGrades::new(session).await.unwrap();
    let semesters = app.semesters().unwrap();
    println!("{:?}", semesters);
    assert!(!semesters.is_empty());
}

#[tokio::test]
#[serial]
async fn classes_with_detail() {
    let session = get_session().await.unwrap();
    let mut app = CourseGrades::new(session).await.unwrap();
    let detail = app.classes("2022", "092", true).await.unwrap();
    println!("{:?}", detail);
    assert!(!detail.is_empty());
}
