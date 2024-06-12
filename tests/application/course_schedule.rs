use rusaint::{
    application::{
        course_schedule::{model::LectureCategory, CourseScheduleApplication},
        USaintClientBuilder,
    },
    model::SemesterType,
    ApplicationError, RusaintError,
};
use serial_test::serial;

use crate::get_session;

#[tokio::test]
#[serial]
async fn find_major() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::major("IT대학", "글로벌미디어학부", None);
    let lectures = app
        .find_lectures(2023, SemesterType::Two, category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
    assert!(true);
}

#[tokio::test]
#[serial]
async fn find_required_elective() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::required_elective("Academic and Professional English 1");
    let lectures = app
        .find_lectures(2023, SemesterType::Two, category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
    assert!(true);
}

#[tokio::test]
#[serial]
async fn find_optional_elective() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::optional_elective("[‘23이후]과학·기술");
    let lectures = app
        .find_lectures(2023, SemesterType::Two, category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
    assert!(true);
}

#[tokio::test]
#[serial]
async fn find_chapel() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::chapel("CHAPEL");
    let lectures = app
        .find_lectures(2023, SemesterType::Two, category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
    assert!(true);
}

#[tokio::test]
#[serial]
async fn find_education() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::education();
    let lectures = app
        .find_lectures(2023, SemesterType::Two, category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
    assert!(true);
}

#[tokio::test]
#[serial]
async fn find_graduated() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::graduated("정보과학대학원", "전체 학과");
    let lectures = app
        .find_lectures(2023, SemesterType::Two, category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
    assert!(true);
}

#[tokio::test]
#[serial]
async fn find_connected_major() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::connected_major("융합창업연계");
    let lectures = app
        .find_lectures(2023, SemesterType::Two, category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
    assert!(true);
}

#[tokio::test]
#[serial]
async fn find_united_major() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::united_major("빅데이터융합");
    let lectures = app
        .find_lectures(2023, SemesterType::Two, category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
    assert!(true);
}

#[tokio::test]
#[serial]
async fn find_recognized_other_major() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::recognized_other_major("IT대학", "글로벌미디어학부", None);
    let lectures = app
        .find_lectures(2023, SemesterType::Two, category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
    assert!(true);
}

#[tokio::test]
#[serial]
async fn find_cyber() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::cyber();
    let lectures = app
        .find_lectures(2023, SemesterType::Two, category)
        .await
        .unwrap();
    for lecture in lectures {
        println!("{:?}", lecture);
    }
    assert!(true);
}

#[tokio::test]
#[serial]
async fn find_nothing() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::find_by_lecture("내가A+받는강의");
    let Some(err) = app
        .find_lectures(2023, SemesterType::Two, category)
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
