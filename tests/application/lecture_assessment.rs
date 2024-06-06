use rusaint::{
    application::{lecture_assessment::LectureAssessment, USaintClientBuilder},
    model::SemesterType,
};
use serial_test::serial;

use crate::get_session;

#[tokio::test]
#[serial]
async fn lecture_assessment() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<LectureAssessment>()
        .await
        .unwrap();
    let info = app
        .find_assessments(2023, SemesterType::Two, Some("마케팅"), None, None)
        .await
        .unwrap();
    assert!(info.len() == 29);
    println!("{} results: {:?}", info.len(), info);
}
