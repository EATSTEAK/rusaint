use rusaint::definitions::BasicUSaintApplication;


#[tokio::test]
async fn basic_usaint_load() {
    let mut app = BasicUSaintApplication::new("ZCMW2100").await.unwrap();
    app.load_placeholder().await.unwrap();

}