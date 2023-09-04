use std::sync::Arc;
use reqwest::{header::*, RequestBuilder, cookie::Jar};
use thiserror::Error;
use url::Url;
use crate::webdynpro::event::event_queue::WDEventQueue;
use self::body::{WDBody, WDBodyUpdate, WDBodyUpdateError, WDBodyError};

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36";


#[derive(Error, Debug)]
pub enum WDClientError {
    #[error("Failed to request from web")]
    Request(#[from] reqwest::Error),
    #[error("Failed to parse HTML body")]
    Parse(#[from] WDBodyError),
    #[error("Failed to parse base url")]
    BaseUrlParse(#[from] url::ParseError),
    #[error("Given base url is not valid")]
    InvalidBaseUrl,
    #[error("Server's update response is invalid")]
    InvalidUpdate(#[from] WDBodyUpdateError),
    #[error("No form found in desired application")]
    NoForm
}

pub struct WDClient {
    client: reqwest::Client,
    pub ssr_client: SapSsrClient,
    pub event_queue: WDEventQueue,
    pub(super) body: WDBody,
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
    headers.insert(ACCEPT_LANGUAGE, "ko,en;q=0.9,en-US;q=0.8".parse().unwrap());
    headers.insert(CONNECTION, "keep-alive".parse().unwrap());
    headers.insert("Sec-Fetch-Dest", "empty".parse().unwrap());
    headers.insert("Sec-Fetch-Mode", "empty".parse().unwrap());
    headers.insert("Sec-Fetch-Site", "empty".parse().unwrap());
    headers.insert("Sec-GPC", "empty".parse().unwrap());
    headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
    headers.insert("X-XHR-Logon", "Accept".parse().unwrap());
    headers
}


impl WDClient {

    pub async fn new(base_url: &Url, app_name: &str) -> Result<WDClient, WDClientError> {
        let jar: Arc<Jar> = Arc::new(Jar::default());
        let client = reqwest::Client::builder()
            .cookie_provider(jar)
            .cookie_store(true)
            .user_agent(USER_AGENT)
            .build()
            .unwrap();
        let raw_body = client.wd_navigate(base_url, app_name).send().await?.text().await?;
        let body = WDBody::new(raw_body);
        let ssr_client = body.parse_sap_ssr_client()?;
        let wd_client = WDClient {
            client,
            body,
            ssr_client,
            event_queue: WDEventQueue::new(),
        };
        Ok(wd_client)
    }

    pub async fn with_client(client: reqwest::Client, base_url: &Url, app_name: &str) -> Result<WDClient, WDClientError> {
        let raw_body = client.wd_navigate(base_url, app_name).send().await?.text().await?;
        let body = WDBody::new(raw_body);
        let ssr_client = body.parse_sap_ssr_client()?;
        let wd_client = WDClient {
            client,
            ssr_client,
            event_queue: WDEventQueue::new(),
            body,
        };
        Ok(wd_client)
    }

    pub async fn send_event(&mut self, base_url: &Url) -> Result<(), WDClientError> {
        let res = self.event_request(base_url).await?;
        self.mutate_body(res)
    }

    async fn event_request(&mut self, base_url: &Url) -> Result<String, WDClientError> {
        let res = self.client.wd_xhr(base_url, &self.ssr_client, &mut self.event_queue)?.send().await?;
        Ok(res.text().await?)
    }

    fn mutate_body(&mut self, response: String) -> Result<(), WDClientError> {
        let body = &mut self.body;
        let update = WDBodyUpdate::new(&response)?;
        body.apply(update);
        Ok(())
    }
}

trait WDRequests {
    fn wd_navigate(&self, base_url: &Url, app_name: &str) -> RequestBuilder;

    fn wd_xhr(&self, base_url: &Url, ssr_client: &SapSsrClient, event_queue: &mut WDEventQueue) -> Result<RequestBuilder, WDClientError>;
}

impl WDRequests for reqwest::Client {
    fn wd_navigate(&self, base_url: &Url, app_name: &str) -> RequestBuilder {
        let mut url = "".to_owned();
        url.push_str(base_url.as_str());
        if !url.ends_with('/') { url.push_str("/"); }
        url.push_str(app_name);
        url.push_str("?sap-wd-stableids=X");
        self.get(url)
            .headers(default_header())
    }

    fn wd_xhr(&self, base_url: &Url, ssr_client: &SapSsrClient, event_queue: &mut WDEventQueue) -> Result<RequestBuilder, WDClientError> {
        let mut url = "".to_owned();
        url.push_str(base_url.scheme());
        url.push_str("://");
        if let Some(host_str) = base_url.host_str() {
            url.push_str(host_str);
        } else {
            return Err(WDClientError::InvalidBaseUrl);
        }
        if let Some(port) = base_url.port() {
            url.push_str(":");
            url.push_str(port.to_string().as_str());
        }
        url.push_str(ssr_client.action.as_str());
        let serialized = event_queue.serialize_and_clear();
        let params = [
            ("sap-charset", ssr_client.charset.as_str()), 
            ("sap-wd-secure-id", ssr_client.wd_secure_id.as_str()),
            ("fesrAppName", ssr_client.app_name.as_str()),
            ("fesrUseBeacon", (if ssr_client.use_beacon { "true" } else { "false" })),
            ("SAPEVENTQUEUE", &serialized),
            ];
        Ok(self.post(url)
            .headers(wd_xhr_header())
            .form(&params))
    }
}

pub mod body;