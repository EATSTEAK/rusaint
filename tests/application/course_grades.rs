use rusaint::{
    application::{
        course_grades::{model::CourseType, CourseGradesApplication},
        USaintClientBuilder,
    },
    model::SemesterType,
};
use serial_test::serial;

use crate::get_session;

#[tokio::test]
#[serial]
async fn recorded_summary() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseGradesApplication>()
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
async fn certificated_summary() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseGradesApplication>()
        .await
        .unwrap();
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
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseGradesApplication>()
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
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseGradesApplication>()
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
