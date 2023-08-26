use std::sync::Arc;
use reqwest::{header::*, RequestBuilder, cookie::Jar};
use thiserror::Error;
use crate::webdynpro::event::event_queue::WDEventQueue;
use self::body::WDBody;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36";


#[derive(Error, Debug)]
pub enum WDClientError {
    #[error("Failed to request from web")]
    Request(#[from] reqwest::Error),
    #[error("Failed to parse HTML body")]
    Parse(#[from] tl::ParseError),
    #[error("No form found in desired application")]
    NoForm
}

pub struct WDClient {
    client: reqwest::Client,
    pub ssr_client: SapSsrClient,
    raw_body: String,
}

pub struct SapSsrClient {
    action: String,
    charset: String,
    wd_secure_id: String,
    pub app_name: String,
    use_beacon: bool
}

fn default_header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "gzip, deflate, br".parse().unwrap());
    headers.insert(ACCEPT_LANGUAGE, "ko,en;q=0.9,en-US;q=0.8".parse().unwrap());
    headers.insert(CACHE_CONTROL, "max-age=0".parse().unwrap());
    headers.insert(CONNECTION, "keep-alive".parse().unwrap());
    headers
}

fn wd_xhr_header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "*/*".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "gzip, deflate, br".parse().unwrap());
    headers.insert(CONNECTION, "keep-alive".parse().unwrap());
    headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
    headers.insert("X-XHR-Logon", "Accept".parse().unwrap());
    headers
}


impl WDClient {

    pub async fn new(base_url: &str, app_name: &str) -> Result<WDClient, WDClientError> {
        let jar: Arc<Jar> = Arc::new(Jar::default());
        let client = reqwest::Client::builder()
            .cookie_provider(jar)
            .cookie_store(true)
            .user_agent(USER_AGENT)
            .build()
            .unwrap();
        let raw_body = client.wd_navigate(base_url, app_name).send().await?.text().await?;
        let ssr_client = WDBody::new(&raw_body)?.parse_sap_ssr_client().ok_or(WDClientError::NoForm)?;
        let wd_client = WDClient {
            client,
            raw_body,
            ssr_client,
        };
        Ok(wd_client)
    }

    pub async fn with_client(client: reqwest::Client, base_url: &str, app_name: &str) -> Result<WDClient, WDClientError> {
        let raw_body = client.wd_navigate(base_url, app_name).send().await?.text().await?;
        let ssr_client = WDBody::new(&raw_body)?.parse_sap_ssr_client().ok_or(WDClientError::NoForm)?;
        let wd_client = WDClient {
            client,
            ssr_client,
            raw_body,
        };
        Ok(wd_client)
    }

    pub async fn event_request(&self) {

    }

    pub fn body(&self) -> Result<WDBody, WDClientError> {
        Ok(WDBody::new(&self.raw_body)?)
    }
}

trait WDRequests {
    fn wd_navigate(&self, base_url: &str, app_name: &str) -> RequestBuilder;

    fn wd_xhr(&self, base_url: &str, ssr_client: &SapSsrClient, event_queue: &mut WDEventQueue) -> RequestBuilder;
}

impl WDRequests for reqwest::Client {
    fn wd_navigate(&self, base_url: &str, app_name: &str) -> RequestBuilder {
        let mut url = "".to_owned();
        url.push_str(base_url);
        if !url.ends_with('/') { url.push_str("/"); }
        url.push_str(app_name);
        url.push_str("?sap-wd-stableids=X");
        println!("{}", url);
        self.get(url)
            .headers(default_header())
    }

    fn wd_xhr(&self, base_url: &str, ssr_client: &SapSsrClient, event_queue: &mut WDEventQueue) -> RequestBuilder {
        let mut url = "".to_owned();
        url.push_str(base_url);
        if !url.ends_with('/') { url.push_str("/"); }
        url.push_str(ssr_client.action.as_str());
        let serialized = event_queue.serialize_and_clear();
        let params = [
            ("sap-charset", ssr_client.charset.as_str()), 
            ("sap-wd-secure-id", ssr_client.wd_secure_id.as_str()),
            ("fesrAppName", ssr_client.app_name.as_str()),
            ("fesrUseBeacon", (if ssr_client.use_beacon { "true" } else { "false" })),
            ("SAPEVENTQUEUE", &serialized),
            ];
        self.post(url)
            .headers(wd_xhr_header())
            .form(&params)
    }
}

pub mod body;