use crate::{get_session, TARGET_SEMESTER, TARGET_YEAR};
use rusaint::model::SemesterType;
use rusaint::{
    application::{chapel::ChapelApplication, USaintClientBuilder},
    ApplicationError, RusaintError,
};

#[tokio::test]
async fn chapel() {
    let session = get_session().await.unwrap().clone();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<ChapelApplication>()
        .await
        .unwrap();
    let info = app
        .information(*TARGET_YEAR, *TARGET_SEMESTER)
        .await
        .unwrap();
    println!("{:?}", info);
}

#[tokio::test]
async fn no_chapel() {
    let session = get_session().await.unwrap().clone();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<ChapelApplication>()
        .await
        .unwrap();
    let info = app.information(2017, SemesterType::Two).await.unwrap_err();
    assert!(matches!(
        info,
        RusaintError::ApplicationError(ApplicationError::NoChapelInformation)
    ));
    println!("{:?}", info);
    println!("{:?}", info);
}
