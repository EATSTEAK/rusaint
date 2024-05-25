use rusaint::{
    application::{
        course_schedule::{model::LectureCategory, CourseSchedule},
        USaintClientBuilder,
    },
    model::SemesterType,
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
