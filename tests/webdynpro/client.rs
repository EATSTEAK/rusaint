use rusaint::webdynpro::application::client::WDClient;
use url::Url;
#[tokio::test]
async fn initial_load() {
    let client = WDClient::new(&Url::parse("https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/").unwrap(), "ZCMW2100").await.unwrap();
    assert_eq!(client.ssr_client.app_name, "ZCMW2100");
}
