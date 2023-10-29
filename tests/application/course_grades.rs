use anyhow::{Error, Result};
use std::sync::{Arc, OnceLock};

use dotenv::dotenv;
use rusaint::{
    application::{
        course_grades::{model::CourseType, CourseGrades},
        USaintApplicationBuilder,
    },
    model::SemesterType,
    USaintSession,
};
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
async fn summaries() {
    let session = get_session().await.unwrap();
    let mut app = USaintApplicationBuilder::new()
        .session(session)
        .build_into::<CourseGrades>()
        .await
        .unwrap();
    let recorded_summary = app.recorded_summary(CourseType::Bachelor).await.unwrap();
    println!("Recorded: {:?}", recorded_summary);
    let certificated_summary = app
        .certificated_summary(CourseType::Bachelor)
        .await
        .unwrap();
    println!("Certificated: {:?}", certificated_summary);
}
#[tokio::test]
#[serial]
async fn semesters() {
    let session = get_session().await.unwrap();
    let mut app = USaintApplicationBuilder::new()
        .session(session)
        .build_into::<CourseGrades>()
        .await
        .unwrap();
    let semesters = app.semesters(CourseType::Bachelor).await.unwrap();
    println!("{:?}", semesters);
    assert!(!semesters.is_empty());
}

#[tokio::test]
#[serial]
async fn classes_with_detail() {
    let session = get_session().await.unwrap();
    let mut app = USaintApplicationBuilder::new()
        .session(session)
        .build_into::<CourseGrades>()
        .await
        .unwrap();
    let details = app
        .classes(CourseType::Bachelor, "2022", SemesterType::Two, true)
        .await
        .unwrap();
    println!("{:?}", details);
    assert!(!details.is_empty());
    println!("Try to obtain class's detail");
    let detail_code = details
        .iter()
        .find(|grade| grade.detail().is_some())
        .unwrap();
    let detail = app
        .class_detail(
            CourseType::Bachelor,
            "2022",
            SemesterType::Two,
            detail_code.code(),
        )
        .await
        .unwrap();
    println!("{:?}", detail);
    assert!(!detail.is_empty());
}
