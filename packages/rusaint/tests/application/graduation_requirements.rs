use crate::get_session;
use lazy_static::lazy_static;
use rusaint::RusaintError;
use rusaint::application::{
    USaintClientBuilder, graduation_requirements::GraduationRequirementsApplication,
};
use std::sync::{Arc, OnceLock};
use test_log::test;
use tokio::sync::{Mutex, RwLock};

lazy_static! {
    static ref APP: Mutex<OnceLock<Arc<RwLock<GraduationRequirementsApplication>>>> =
        Mutex::new(OnceLock::new());
}

async fn get_app() -> Result<Arc<RwLock<GraduationRequirementsApplication>>, RusaintError> {
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
async fn student_info() {
    let lock = get_app().await.unwrap();
    let app = lock.read().await;
    let student_info = app.student_info().await.unwrap();
    println!("{:?}", student_info);
}

#[test(tokio::test)]
async fn graduation_requirements() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let graduation_requirements = app.requirements().await.unwrap();
    println!("{:?}", graduation_requirements);
}
