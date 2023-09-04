use rusaint::definitions::course_schedule::CourseSchedule;


#[tokio::test]
async fn course_schedule_load() {
    let mut app = CourseSchedule::new().await.unwrap();
    app.load_placeholder().await.unwrap();
}