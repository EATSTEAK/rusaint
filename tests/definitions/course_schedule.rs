use rusaint::definitions::course_schedule::CourseSchedule;

#[tokio::test]
async fn initial_load() {
    let mut app = CourseSchedule::new().await.unwrap();
    app.load_placeholder().await.unwrap();
}

#[tokio::test]
async fn edu_data() {
    let mut app = CourseSchedule::new().await.unwrap();
    app.load_placeholder().await.unwrap();
    app.load_edu().await.unwrap();
    let table = app.read_edu_raw().unwrap();
    println!("{:?}", table.table());
    assert!(false);
}
