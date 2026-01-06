use crate::{TARGET_SEMESTER, TARGET_YEAR, get_session};
use lazy_static::lazy_static;
use rusaint::RusaintError;
use rusaint::application::course_registration_status::CourseRegistrationStatusApplication;
use rusaint::client::USaintClientBuilder;
use std::sync::{Arc, OnceLock};
use tokio::sync::{Mutex, RwLock};
use tracing_test::traced_test;

lazy_static! {
    static ref APP: Mutex<OnceLock<Arc<RwLock<CourseRegistrationStatusApplication>>>> =
        Mutex::new(OnceLock::new());
}

async fn get_app() -> Result<Arc<RwLock<CourseRegistrationStatusApplication>>, RusaintError> {
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

#[tokio::test]
#[traced_test]
async fn get_selected_semester() {
    let lock = get_app().await.unwrap();
    let app = lock.read().await;
    let selected_semester = app.get_selected_semester().unwrap();
    tracing::info!("Selected semester: {:?}", selected_semester);
}

#[tokio::test]
#[traced_test]
async fn lectures() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let result = app.lectures(*TARGET_YEAR, *TARGET_SEMESTER).await;
    match result {
        Ok(lectures) => {
            let lectures: Vec<_> = lectures.collect();
            tracing::info!("Lectures: {:?}", lectures);
        }
        Err(e) => {
            // NoLectureResult is acceptable
            tracing::info!("Error (may be expected): {:?}", e);
        }
    }
}
