use super::{
    body::{Body, BodyUpdate},
    SapSsrClient,
};
use crate::{
    utils::{default_header, DEFAULT_USER_AGENT},
    webdynpro::{
        error::{ClientError, WebDynproError},
        event::{event_queue::EventQueue, Event},
    },
};
use reqwest::{cookie::Jar, header::*, RequestBuilder};
use std::sync::Arc;
use url::Url;

/// WebDynpro 애플리케이션의 웹 요청 및 페이지 문서 처리를 담당하는 클라이언트
pub struct Client {
    client: reqwest::Client,
    event_queue: EventQueue,
}

pub(super) fn wd_xhr_header() -> HeaderMap {
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

impl Client {
    
    /// 새로운 클라이언트를 생성합니다.
    pub fn new() -> Client {
        let jar: Arc<Jar> = Arc::new(Jar::default());
        let client = reqwest::Client::builder()
            .cookie_provider(jar)
            .cookie_store(true)
            .user_agent(DEFAULT_USER_AGENT)
            .build()
            .unwrap();
        Self::with_client(client)
    }

    /// 임의의 reqwest::Client 와 함께 클라이언트를 생성합니다.
    pub fn with_client(client: reqwest::Client) -> Client {
        Client {
            client,
            event_queue: EventQueue::new(),
        }
    }

    pub(crate) async fn navigate(
        &mut self,
        base_url: &Url,
        app_name: &str,
    ) -> Result<Body, ClientError> {
        let raw_body = self
            .client
            .wd_navigate(base_url, app_name)
            .send()
            .await?
            .text()
            .await?;
        Ok(Body::new(raw_body)?)
    }

    pub(crate) fn add_event(&mut self, event: Event) {
        self.event_queue.add(event)
    }

    pub(crate) async fn send_event(
        &mut self,
        base_url: &Url,
        ssr_client: &SapSsrClient,
    ) -> Result<BodyUpdate, WebDynproError> {
        let res = self.event_request(base_url, ssr_client).await?;
        Ok(BodyUpdate::new(&res)?)
    }

    async fn event_request(
        &mut self,
        base_url: &Url,
        ssr_client: &SapSsrClient,
    ) -> Result<String, ClientError> {
        let res = self
            .client
            .wd_xhr(base_url, ssr_client, &mut self.event_queue)?
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(ClientError::InvalidResponse(res))?;
        }
        Ok(res.text().await?)
    }
}

trait Requests {
    fn wd_navigate(&self, base_url: &Url, app_name: &str) -> RequestBuilder;

    fn wd_xhr(
        &self,
        base_url: &Url,
        ssr_client: &SapSsrClient,
        event_queue: &mut EventQueue,
    ) -> Result<RequestBuilder, ClientError>;
}

impl Requests for reqwest::Client {
    fn wd_navigate(&self, base_url: &Url, app_name: &str) -> RequestBuilder {
        let mut url = "".to_owned();
        url.push_str(base_url.as_str());
        if !url.ends_with('/') {
            url.push_str("/");
        }
        url.push_str(app_name);
        url.push_str("?sap-wd-stableids=X");
        self.get(url).headers(default_header())
    }

    fn wd_xhr(
        &self,
        base_url: &Url,
        ssr_client: &SapSsrClient,
        event_queue: &mut EventQueue,
    ) -> Result<RequestBuilder, ClientError> {
        let mut url = "".to_owned();
        url.push_str(base_url.scheme());
        url.push_str("://");
        if let Some(host_str) = base_url.host_str() {
            url.push_str(host_str);
        } else {
            return Err(ClientError::InvalidBaseUrl(base_url.to_string()))?;
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
            (
                "fesrUseBeacon",
                (if ssr_client.use_beacon {
                    "true"
                } else {
                    "false"
                }),
            ),
            ("SAPEVENTQUEUE", &serialized),
        ];
        Ok(self.post(url).headers(wd_xhr_header()).form(&params))
    }
}

#[cfg(test)]
mod test {
    use url::Url;

    use crate::webdynpro::application::client::Client;

    #[tokio::test]
    async fn initial_load() {
        let mut client = Client::new();
        let body = client
            .navigate(
                &Url::parse("https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/").unwrap(),
                "ZCMW2100",
            )
            .await
            .unwrap();
        assert_eq!(body.ssr_client().app_name, "ZCMW2100");
    }
}
