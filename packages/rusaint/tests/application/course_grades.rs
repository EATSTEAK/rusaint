use crate::{TARGET_SEMESTER, TARGET_YEAR, get_session};
use lazy_static::lazy_static;
use rusaint::RusaintError;
use rusaint::application::{
    USaintClientBuilder,
    course_grades::{CourseGradesApplication, model::CourseType},
};
use std::sync::{Arc, OnceLock};
use test_log::test;
use tokio::sync::{Mutex, RwLock};

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

#[test(tokio::test)]
async fn recorded_summary() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let recorded_summary = app.recorded_summary(CourseType::Bachelor).await.unwrap();
    println!("Recorded: {:?}", recorded_summary);
    let certificated_summary = app
        .certificated_summary(CourseType::Bachelor)
        .await
        .unwrap();
    println!("Certificated: {:?}", certificated_summary);
}

#[test(tokio::test)]
async fn certificated_summary() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let certificated_summary = app
        .certificated_summary(CourseType::Bachelor)
        .await
        .unwrap();
    println!("Certificated: {:?}", certificated_summary);
}

#[test(tokio::test)]
async fn semesters() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let semesters = app.semesters(CourseType::Bachelor).await.unwrap();
    println!("{:?}", semesters);
    assert!(!semesters.is_empty());
}

#[test(tokio::test)]
async fn classes_with_detail() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let details = app
        .classes(CourseType::Bachelor, *TARGET_YEAR, *TARGET_SEMESTER, true)
        .await
        .unwrap();
    println!("{:?}", details);
    assert!(!details.is_empty());
    println!("Try to obtain class's detail");
    let detail_code = details.iter().find(|grade| grade.detail().is_some());
    if detail_code.is_none() {
        println!("No class found with detail");
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
    println!("{:?}", detail);
    assert!(!detail.is_empty());
}
