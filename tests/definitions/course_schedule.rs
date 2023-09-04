use rusaint::definitions::course_schedule::{CourseSchedule, PeriodType};
use scraper::Selector;


#[tokio::test]
async fn course_schedule_load() {
    let mut app = CourseSchedule::new().await.unwrap();
    app.load_placeholder().await.unwrap();
    app.load_edu().await.unwrap();
    let table_selector = Selector::parse(r#"[id="SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.ROOTUIELEMENTCONTAINER"]"#).unwrap();
    let doc = app.body().document();
    let selection = doc.select(&table_selector);
    for table in selection {
        println!("{}", table.html());
    }
}