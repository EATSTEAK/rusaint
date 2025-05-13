use crate::{TARGET_SEMESTER, TARGET_YEAR, get_session};
use lazy_static::lazy_static;
use rusaint::model::SemesterType;
use rusaint::{
    ApplicationError, RusaintError,
    application::{USaintClientBuilder, chapel::ChapelApplication},
};
use std::sync::{Arc, OnceLock};
use test_log::test;
use tokio::sync::{Mutex, RwLock};

lazy_static! {
    static ref APP: Mutex<OnceLock<Arc<RwLock<ChapelApplication>>>> = Mutex::new(OnceLock::new());
}

async fn get_app() -> Result<Arc<RwLock<ChapelApplication>>, RusaintError> {
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
async fn chapel() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let info = app
        .information(*TARGET_YEAR, *TARGET_SEMESTER)
        .await
        .unwrap();
    println!("{:?}", info);
}

#[test(tokio::test)]
async fn no_chapel() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let info = app.information(2017, SemesterType::Two).await.unwrap_err();
    assert!(matches!(
        info,
        RusaintError::ApplicationError(ApplicationError::NoChapelInformation)
    ));
    println!("{:?}", info);
    println!("{:?}", info);
}
