use self::body::{Body, BodyUpdate};
use crate::{
    utils::{default_header, DEFAULT_USER_AGENT},
    webdynpro::{
        error::ClientError,
        event::{event_queue::EventQueue, Event},
    },
};
use anyhow::Result;
use reqwest::{cookie::Jar, header::*, RequestBuilder};
use std::sync::Arc;
use url::Url;

pub struct Client {
    client: reqwest::Client,
    ssr_client: SapSsrClient,
    event_queue: EventQueue,
    pub(super) body: Body,
}

pub(super) struct SapSsrClient {
    action: String,
    charset: String,
    wd_secure_id: String,
    pub app_name: String,
    use_beacon: bool,
}

pub fn wd_xhr_header() -> HeaderMap {
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
    pub async fn new(base_url: &Url, app_name: &str) -> Result<Client> {
        let jar: Arc<Jar> = Arc::new(Jar::default());
        let client = reqwest::Client::builder()
            .cookie_provider(jar)
            .cookie_store(true)
            .user_agent(DEFAULT_USER_AGENT)
            .build()
            .unwrap();
        Self::with_client(client, base_url, app_name).await
    }

    pub async fn with_client(
        client: reqwest::Client,
        base_url: &Url,
        app_name: &str,
    ) -> Result<Client> {
        let raw_body = client
            .wd_navigate(base_url, app_name)
            .send()
            .await?
            .text()
            .await?;
        let body = Body::new(raw_body);
        let ssr_client = body.parse_sap_ssr_client()?;
        let wd_client = Client {
            client,
            ssr_client,
            event_queue: EventQueue::new(),
            body,
        };
        Ok(wd_client)
    }

    pub(crate) fn add_event(&mut self, event: Event) {
        self.event_queue.add(event)
    }

    pub(crate) async fn send_event(&mut self, base_url: &Url) -> Result<()> {
        let res = self.event_request(base_url).await?;
        self.mutate_body(res)
    }

    async fn event_request(&mut self, base_url: &Url) -> Result<String> {
        let res = self
            .client
            .wd_xhr(base_url, &self.ssr_client, &mut self.event_queue)?
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(ClientError::RequestFailed(res))?;
        }
        Ok(res.text().await?)
    }

    fn mutate_body(&mut self, response: String) -> Result<()> {
        let body = &mut self.body;
        let update = BodyUpdate::new(&response)?;
        Ok(body.apply(update)?)
    }
}

trait Requests {
    fn wd_navigate(&self, base_url: &Url, app_name: &str) -> RequestBuilder;

    fn wd_xhr(
        &self,
        base_url: &Url,
        ssr_client: &SapSsrClient,
        event_queue: &mut EventQueue,
    ) -> Result<RequestBuilder>;
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
    ) -> Result<RequestBuilder> {
        let mut url = "".to_owned();
        url.push_str(base_url.scheme());
        url.push_str("://");
        if let Some(host_str) = base_url.host_str() {
            url.push_str(host_str);
        } else {
            return Err(ClientError::InvalidBaseUrl)?;
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

pub mod body;

#[cfg(test)]
mod test {
    use url::Url;

    use crate::webdynpro::application::client::Client;
    #[tokio::test]
    async fn initial_load() {
        let client = Client::new(
            &Url::parse("https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/").unwrap(),
            "ZCMW2100",
        )
        .await
        .unwrap();
        assert_eq!(client.ssr_client.app_name, "ZCMW2100");
    }
}
