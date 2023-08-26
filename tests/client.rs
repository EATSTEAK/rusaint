use rusaint::webdynpro::application::client::WDClient;
#[tokio::test]
async fn initial_load() {
    let client = WDClient::new("https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/", "ZCMW2100").await.unwrap();
    assert_eq!(client.ssr_client.app_name, "ZCMW2100");
}

async fn first_xhr() {
    let client = WDClient::new("https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/", "ZCMW2100").await.unwrap();
    assert_eq!(false, true);
}
