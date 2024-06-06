use rusaint::{
    application::{chapel::Chapel, USaintClientBuilder},
    model::SemesterType,
    ApplicationError, RusaintError,
};
use serial_test::serial;

use crate::get_session;

#[tokio::test]
#[serial]
async fn chapel() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<Chapel>()
        .await
        .unwrap();
    let info = app.information(2022, SemesterType::Two).await.unwrap();
    println!("{:?}", info);
}

#[tokio::test]
#[serial]
async fn no_chapel() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<Chapel>()
        .await
        .unwrap();
    let info = app.information(2024, SemesterType::Two).await.unwrap_err();
    assert!(matches!(
        info,
        RusaintError::ApplicationError(ApplicationError::NoChapelInformation)
    ));
    println!("{:?}", info);
    println!("{:?}", info);
}
