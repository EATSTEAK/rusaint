use crate::{get_session, TARGET_SEMESTER, TARGET_YEAR};
use rusaint::model::SemesterType;
use rusaint::{
    application::{
        personal_course_schedule::PersonalCourseScheduleApplication, USaintClientBuilder,
    },
    ApplicationError, RusaintError,
};

#[tokio::test]
async fn schedule() {
    let session = get_session().await.unwrap().clone();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<PersonalCourseScheduleApplication>()
        .await
        .unwrap();
    let info = app.schedule(*TARGET_YEAR, *TARGET_SEMESTER).await.unwrap();
    println!("{:?}", info);
}

#[tokio::test]
async fn no_schedule() {
    let session = get_session().await.unwrap().clone();
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
