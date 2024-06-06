use rusaint::{
    application::{
        course_schedule::{model::LectureCategory, CourseSchedule},
        USaintClientBuilder,
    },
    model::SemesterType,
    ApplicationError, RusaintError,
};
use serial_test::serial;

use crate::get_session;

#[tokio::test]
#[serial]
async fn find_major() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseSchedule>()
        .await
        .unwrap();
    let category = LectureCategory::major("IT대학", "글로벌미디어학부", None);
    let lectures = app
        .find_lectures(2023, SemesterType::Two, category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
    assert!(true);
}

#[tokio::test]
#[serial]
async fn find_nothing() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseSchedule>()
        .await
        .unwrap();
    let category = LectureCategory::find_by_lecture("내가A+받는강의");
    let Some(err) = app
        .find_lectures(2023, SemesterType::Two, category)
        .await
        .err()
    else {
        panic!("this lecture query should return error");
    };
    assert!(matches!(
        err,
        RusaintError::ApplicationError(ApplicationError::NoLectureResult)
    ));
}
