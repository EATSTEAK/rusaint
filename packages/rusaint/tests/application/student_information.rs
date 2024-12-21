use crate::get_session;
use rusaint::{
    application::{student_information::StudentInformationApplication, USaintClientBuilder},
    webdynpro::error::{ElementError, WebDynproError},
    RusaintError,
};

#[tokio::test]
async fn general() {
    let session = get_session().await.unwrap().clone();
    let app = USaintClientBuilder::new()
        .session(session)
        .build_into::<StudentInformationApplication>()
        .await
        .unwrap();
    let student_info = app.general().unwrap();
    println!("{:?}", student_info);
}

#[tokio::test]
async fn graduation() {
    let session = get_session().await.unwrap().clone();
    let app = USaintClientBuilder::new()
        .session(session)
        .build_into::<StudentInformationApplication>()
        .await
        .unwrap();
    let student_info = app.graduation();
    match student_info {
        Err(RusaintError::WebDynproError(WebDynproError::Element(
            ElementError::NoSuchContent {
                element: _,
                content: _,
            },
        ))) => (),
        Err(err) => {
            panic!("{:?}", err);
        }
        Ok(_) => (),
    }
    println!("{:?}", student_info);
}

#[tokio::test]
async fn qualifications() {
    let session = get_session().await.unwrap().clone();
    let app = USaintClientBuilder::new()
        .session(session)
        .build_into::<StudentInformationApplication>()
        .await
        .unwrap();
    let student_info = app.qualifications();
    println!("{:?}", student_info);
}

#[tokio::test]
async fn work() {
    let session = get_session().await.unwrap().clone();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<StudentInformationApplication>()
        .await
        .unwrap();
    let student_info = app.work().await.unwrap();
    println!("{:?}", student_info);
}

#[tokio::test]
async fn family() {
    let session = get_session().await.unwrap().clone();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<StudentInformationApplication>()
        .await
        .unwrap();
    let student_info = app.family().await.unwrap();
    println!("{:?}", student_info);
}

#[tokio::test]
async fn religion() {
    let session = get_session().await.unwrap().clone();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<StudentInformationApplication>()
        .await
        .unwrap();
    let student_info = app.religion().await.unwrap();
    println!("{:?}", student_info);
}

#[tokio::test]
async fn transfer() {
    let session = get_session().await.unwrap().clone();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<StudentInformationApplication>()
        .await
        .unwrap();
    let student_info = app.transfer().await.unwrap();
    println!("{:?}", student_info);
}

#[tokio::test]
async fn bank_account() {
    let session = get_session().await.unwrap().clone();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<StudentInformationApplication>()
        .await
        .unwrap();
    let student_info = app.bank_account().await.unwrap();
    println!("{:?}", student_info);
}

#[tokio::test]
async fn academic_record() {
    let session = get_session().await.unwrap().clone();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<StudentInformationApplication>()
        .await
        .unwrap();
    let student_info = app.academic_record().await.unwrap();
    println!("{:?}", student_info);
}

#[tokio::test]
async fn research_bank_account() {
    let session = get_session().await.unwrap().clone();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<StudentInformationApplication>()
        .await
        .unwrap();
    let student_info = app.research_bank_account().await.unwrap();
    println!("{:?}", student_info);
}
