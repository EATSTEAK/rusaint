use rusaint::application::USaintApplication;

#[tokio::test]
async fn basic_usaint_load() {
    let mut app = USaintApplication::new("ZCMW2100").await.unwrap();
    app.load_placeholder().await.unwrap();
}
