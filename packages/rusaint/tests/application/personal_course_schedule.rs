use crate::{get_semester, get_session, get_year};
use rusaint::model::SemesterType;
use rusaint::{
    application::{
        personal_course_schedule::PersonalCourseScheduleApplication, USaintClientBuilder,
    },
    ApplicationError, RusaintError,
};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn schedule() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<PersonalCourseScheduleApplication>()
        .await
        .unwrap();
    let info = app
        .schedule(get_year().unwrap(), get_semester().unwrap())
        .await
        .unwrap();
    println!("{:?}", info);
}

#[tokio::test]
#[serial]
async fn no_schedule() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<PersonalCourseScheduleApplication>()
        .await
        .unwrap();
    let info = app.schedule(2017, SemesterType::Two).await.unwrap_err();
    assert!(matches!(
        info,
        RusaintError::ApplicationError(ApplicationError::NoScheduleInformation)
    ));
    println!("{:?}", info);
}
