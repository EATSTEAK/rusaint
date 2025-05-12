use crate::get_session;
use lazy_static::lazy_static;
use rusaint::{
    ApplicationError, RusaintError,
    application::{
        USaintClientBuilder,
        course_schedule::{CourseScheduleApplication, model::LectureCategory},
    },
    model::SemesterType,
};
use std::sync::{Arc, OnceLock};
use test_log::test;
use tokio::sync::{Mutex, RwLock};

lazy_static! {
    static ref APP: Mutex<OnceLock<Arc<RwLock<CourseScheduleApplication>>>> =
        Mutex::new(OnceLock::new());
}

async fn get_app() -> Result<Arc<RwLock<CourseScheduleApplication>>, RusaintError> {
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
async fn find_major() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let category = LectureCategory::major("IT대학", "글로벌미디어학부", None);
    let lectures = app
        .find_lectures(2023, SemesterType::Two, &category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
}

#[test(tokio::test)]
async fn find_required_elective() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let category = LectureCategory::required_elective("Academic and Professional English 1");
    let lectures = app
        .find_lectures(2023, SemesterType::Two, &category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
}

#[test(tokio::test)]
async fn find_optional_elective() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let category = LectureCategory::optional_elective("[‘23이후]과학·기술");
    let lectures = app
        .find_lectures(2023, SemesterType::Two, &category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
}

#[test(tokio::test)]
async fn find_chapel() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let category = LectureCategory::chapel("CHAPEL");
    let lectures = app
        .find_lectures(2023, SemesterType::Two, &category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
}

#[test(tokio::test)]
async fn find_education() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let category = LectureCategory::education();
    let lectures = app
        .find_lectures(2023, SemesterType::Two, &category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
}

#[test(tokio::test)]
async fn find_graduated() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let category = LectureCategory::graduated("정보과학대학원", "전체 학과");
    let lectures = app
        .find_lectures(2023, SemesterType::Two, &category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
}

#[test(tokio::test)]
async fn find_connected_major() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let category = LectureCategory::connected_major("융합창업연계");
    let lectures = app
        .find_lectures(2023, SemesterType::Two, &category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
}

#[test(tokio::test)]
async fn find_united_major() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let category = LectureCategory::united_major("빅데이터융합");
    let lectures = app
        .find_lectures(2023, SemesterType::Two, &category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
}

#[test(tokio::test)]
async fn find_recognized_other_major() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let category = LectureCategory::recognized_other_major("IT대학", "글로벌미디어학부", None);
    let lectures = app
        .find_lectures(2023, SemesterType::Two, &category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
}

#[test(tokio::test)]
async fn find_cyber() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let category = LectureCategory::cyber();
    let lectures = app
        .find_lectures(2023, SemesterType::Two, &category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
}

#[test(tokio::test)]
async fn find_nothing() {
    let lock = get_app().await.unwrap();
    let mut app = lock.write().await;
    let category = LectureCategory::find_by_lecture("내가A+받는강의");
    let Some(err) = app
        .find_lectures(2023, SemesterType::Two, &category)
        .await
        .err()
    else {
        panic!("this lecture query should return error");
    };
    assert!(matches!(
        err,
        RusaintError::ApplicationError(ApplicationError::NoLectureResult)
    ));
}
