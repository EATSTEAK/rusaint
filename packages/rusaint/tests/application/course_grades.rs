use crate::{TARGET_SEMESTER, TARGET_YEAR, get_session};
use lazy_static::lazy_static;
use rusaint::RusaintError;
use rusaint::application::course_grades::{CourseGradesApplication, model::CourseType};
use rusaint::client::USaintClientBuilder;
use std::sync::{Arc, OnceLock};
use tokio::sync::{Mutex, RwLock};
use tracing_test::traced_test;

lazy_static! {
    static ref APP: Mutex<OnceLock<Arc<RwLock<CourseGradesApplication>>>> =
        Mutex::new(OnceLock::new());
}

async fn get_app() -> Result<Arc<RwLock<CourseGradesApplication>>, RusaintError> {
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
async fn recorded_summary() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let recorded_summary = app.recorded_summary(CourseType::Bachelor).await.unwrap();
    tracing::info!("Recorded: {:?}", recorded_summary);
    let certificated_summary = app
        .certificated_summary(CourseType::Bachelor)
        .await
        .unwrap();
    tracing::info!("Certificated: {:?}", certificated_summary);
}

#[tokio::test]
#[traced_test]
async fn certificated_summary() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let certificated_summary = app
        .certificated_summary(CourseType::Bachelor)
        .await
        .unwrap();
    tracing::info!("Certificated: {:?}", certificated_summary);
}

#[tokio::test]
#[traced_test]
async fn semesters() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let semesters = app.semesters(CourseType::Bachelor).await.unwrap();
    tracing::info!("{:?}", semesters);
    assert!(!semesters.is_empty());
}

#[tokio::test]
#[traced_test]
async fn classes_with_detail() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let details = app
        .classes(CourseType::Bachelor, *TARGET_YEAR, *TARGET_SEMESTER, true)
        .await
        .unwrap();
    tracing::info!("{:?}", details);
    assert!(!details.is_empty());
    tracing::info!("Try to obtain class's detail");
    let detail_code = details.iter().find(|grade| grade.detail().is_some());
    if detail_code.is_none() {
        tracing::info!("No class found with detail");
        return;
    }
    let detail_code = detail_code.unwrap();
    let detail = app
        .class_detail(
            CourseType::Bachelor,
            *TARGET_YEAR,
            *TARGET_SEMESTER,
            detail_code.code(),
        )
        .await
        .unwrap();
    tracing::info!("{:?}", detail);
    assert!(!detail.is_empty());
}
