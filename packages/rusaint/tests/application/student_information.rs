use crate::get_session;
use lazy_static::lazy_static;
use rusaint::{
    RusaintError,
    application::{USaintClientBuilder, student_information::StudentInformationApplication},
    webdynpro::error::{ElementError, WebDynproError},
};
use std::sync::{Arc, OnceLock};
use test_log::test;
use tokio::sync::{Mutex, RwLock};

lazy_static! {
    static ref APP: Mutex<OnceLock<Arc<RwLock<StudentInformationApplication>>>> =
        Mutex::new(OnceLock::new());
}

async fn get_app() -> Result<Arc<RwLock<StudentInformationApplication>>, RusaintError> {
    let app_lock = APP.lock().await;
    if let Some(lock) = app_lock.get() {
        Ok(lock.clone())
    } else {
        let session = get_session().await.unwrap().clone();
        app_lock
            .set(Arc::new(RwLock::new(
                USaintClientBuilder::new()
                    .session(session)
                    .build_into()
                    .await?,
            )))
            .unwrap();
        Ok(app_lock.get().unwrap().clone())
    }
}

#[test(tokio::test)]
async fn general() {
    let lock = get_app().await.unwrap();
    let app = lock.read().await;
    let student_info = app.general().unwrap();
    println!("{:?}", student_info);
}

#[test(tokio::test)]
async fn graduation() {
    let lock = get_app().await.unwrap();
    let app = lock.read().await;
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

#[test(tokio::test)]
async fn qualifications() {
    let lock = get_app().await.unwrap();
    let app = lock.read().await;
    let student_info = app.qualifications();
    println!("{:?}", student_info);
}

#[test(tokio::test)]
async fn work() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let student_info = app.work().await.unwrap();
    println!("{:?}", student_info);
}

#[test(tokio::test)]
async fn family() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let student_info = app.family().await.unwrap();
    println!("{:?}", student_info);
}

#[test(tokio::test)]
async fn religion() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let student_info = app.religion().await.unwrap();
    println!("{:?}", student_info);
}

#[test(tokio::test)]
async fn transfer() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let student_info = app.transfer().await.unwrap();
    println!("{:?}", student_info);
}

#[test(tokio::test)]
async fn bank_account() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let student_info = app.bank_account().await.unwrap();
    println!("{:?}", student_info);
}

#[test(tokio::test)]
async fn academic_record() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let student_info = app.academic_record().await.unwrap();
    println!("{:?}", student_info);
}

#[test(tokio::test)]
async fn research_bank_account() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let student_info = app.research_bank_account().await.unwrap();
    println!("{:?}", student_info);
}
