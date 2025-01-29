use crate::get_session;
use rusaint::application::scholarships::ScholarshipsApplication;
use rusaint::application::USaintClientBuilder;

#[tokio::test]
async fn scholarships() {
    let session = get_session().await.unwrap().clone();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<ScholarshipsApplication>()
        .await
        .unwrap();
    let info = app.scholarships().await.unwrap();
    println!("{:?}", info);
}
