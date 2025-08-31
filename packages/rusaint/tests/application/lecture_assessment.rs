use crate::get_session;
use rusaint::{
    application::lecture_assessment::LectureAssessmentApplication, client::USaintClientBuilder,
    model::SemesterType,
};
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
async fn lecture_assessment() {
    let session = get_session().await.unwrap().clone();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<LectureAssessmentApplication>()
        .await
        .unwrap();
    let info = app
        .find_assessments(2023, SemesterType::Two, Some("마케팅"), None, None)
        .await
        .unwrap();
    assert_eq!(info.len(), 29);
    tracing::info!("{} results: {:?}", info.len(), info);
}
