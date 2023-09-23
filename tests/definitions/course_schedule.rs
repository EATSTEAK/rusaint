use rusaint::definitions::course_schedule::CourseSchedule;
use scraper::Selector;

#[tokio::test]
async fn initial_load() {
    let mut app = CourseSchedule::new().await.unwrap();
    app.load_placeholder().await.unwrap();
}

#[tokio::test]
async fn edu_data() {
    let mut app = CourseSchedule::new().await.unwrap();
    let table = app.read_edu_raw().await;
    if let Ok(table) = table {
        println!("{:?}", table.table());
    } else {
        eprintln!("{:?}", table.unwrap_err());
    }

    assert!(false);
}
