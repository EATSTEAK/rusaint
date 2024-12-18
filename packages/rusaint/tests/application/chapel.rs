use crate::{get_semester, get_session, get_year};
use rusaint::model::SemesterType;
use rusaint::{
    application::{chapel::ChapelApplication, USaintClientBuilder},
    ApplicationError, RusaintError,
};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn chapel() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<ChapelApplication>()
        .await
        .unwrap();
    let info = app
        .information(get_year().unwrap(), get_semester().unwrap())
        .await
        .unwrap();
    println!("{:?}", info);
}

#[tokio::test]
#[serial]
async fn no_chapel() {
    let session = get_session().await.unwrap();
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
