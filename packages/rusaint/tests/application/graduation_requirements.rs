use crate::get_session;
use rusaint::application::{
    graduation_requirements::GraduationRequirementsApplication, USaintClientBuilder,
};

#[tokio::test]
async fn student_info() {
    let session = get_session().await.unwrap().clone();
    let app = USaintClientBuilder::new()
        .session(session)
        .build_into::<GraduationRequirementsApplication>()
        .await
        .unwrap();
    let student_info = app.student_info().await.unwrap();
    println!("{:?}", student_info);
}

#[tokio::test]
async fn graduation_requirements() {
    let session = get_session().await.unwrap().clone();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<GraduationRequirementsApplication>()
        .await
        .unwrap();
    let graduation_requirements = app.requirements().await.unwrap();
    println!("{:?}", graduation_requirements);
}
