use crate::webdynpro::event::ucf_parameters::UcfParameters;
use crate::webdynpro::event::{EventBuilder, EventBuilderError};
use crate::{
    utils::{DEFAULT_USER_AGENT, default_header},
    webdynpro::{
        error::{ClientError, WebDynproError},
        event::{Event, event_queue::EventQueue},
    },
};
use body::{Body, BodyUpdate};
use reqwest::{RequestBuilder, cookie::Jar, header::*};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use url::Url;

/// WebDynpro 애플리케이션의 웹 요청 및 페이지 문서 처리를 담당하는 클라이언트
#[derive(Debug)]
pub struct WebDynproClient {
    base_url: Url,
    name: String,
    body: Body,
    client: reqwest::Client,
    event_queue: Mutex<EventQueue>,
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

impl WebDynproClient {
    /// WebDynpro 애플리케이션의 이름을 반환합니다.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// WebDynpro 애플리케이션의 기본 URL을 반환합니다.
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// WebDynpro 애플리케이션의 페이지 문서를 반환합니다.
    pub fn body(&self) -> &Body {
        &self.body
    }

    /// 실제로 요청하는 애플리케이션의 URL을 반환합니다.
    pub fn client_url(&self) -> String {
        let mut url = "".to_owned();
        url.push_str(self.base_url().as_str());
        if !url.ends_with('/') {
            url.push('/');
        }
        url.push_str(self.name());
        url.push_str("?sap-wd-stableids=X#");
        url
    }

    /// 새로운 클라이언트를 생성합니다.
    async fn new(base_url: Url, name: &str) -> Result<WebDynproClient, ClientError> {
        let jar: Arc<Jar> = Arc::new(Jar::default());
        let client = reqwest::Client::builder()
            .cookie_provider(jar)
            .cookie_store(true)
            .user_agent(DEFAULT_USER_AGENT)
            .build()?;
        Self::with_client(base_url, name, client).await
    }

    /// 임의의 reqwest::Client 와 함께 클라이언트를 생성합니다.
    async fn with_client(
        base_url: Url,
        name: &str,
        client: reqwest::Client,
    ) -> Result<WebDynproClient, ClientError> {
        let raw_body = client
            .wd_navigate(&base_url, name)
            .send()
            .await?
            .text()
            .await?;
        Ok(WebDynproClient {
            base_url,
            name: name.to_owned(),
            body: Body::new(raw_body)?,
            client,
            event_queue: Mutex::new(EventQueue::new()),
        })
    }

    #[allow(dead_code)]
    /// 특정 WebDynpro 애플리케이션으로 탐색합니다.
    pub(crate) async fn navigate(&mut self, base_url: &Url, name: &str) -> Result<(), ClientError> {
        let raw_body = self
            .client
            .wd_navigate(base_url, name)
            .send()
            .await?
            .text()
            .await?;
        self.body = Body::new(raw_body)?;
        Ok(())
    }

    /// 이벤트 유형에 따라 이벤트 큐를 큐에 추가하거나 서버에 전송합니다.
    pub async fn process_event(
        &mut self,
        force_send: bool,
        event: Event,
    ) -> Result<EventProcessResult, WebDynproError> {
        let form_req = create_form_request_event(false, "", "", false, false).or(Err(
            ClientError::NoSuchForm("sap.client.SsrClient.form".to_string()),
        ))?;
        if (!event.is_enqueable() && event.is_submitable()) || force_send {
            {
                self.add_event(event);
                self.add_event(form_req.to_owned());
            }
            let update = { self.send_events().await? };
            self.mutate_body(update)?;
            Ok(EventProcessResult::Sent)
        } else {
            self.add_event(event);
            Ok(EventProcessResult::Enqueued)
        }
    }

    /// 이벤트를 이벤트 큐에 추가합니다.
    fn add_event(&mut self, event: Event) {
        self.event_queue.try_lock().unwrap().add(event)
    }

    /// 이벤트 큐 내의 이벤트를 전송하고 그 결과를 `BodyUpdate`로 반환합니다.
    async fn send_events(&mut self) -> Result<BodyUpdate, WebDynproError> {
        let res = self.event_request().await?;
        Ok(BodyUpdate::new(&res)?)
    }

    /// 이벤트 큐 내부 내용을 서버에 전송하고 응답을 받습니다.
    async fn event_request(&mut self) -> Result<String, ClientError> {
        let mut event_queue = self.event_queue.lock().await;
        let serialized_events = event_queue.serialize_and_clear();
        let res = self
            .client
            .wd_xhr(&self.base_url, self.body.ssr_client(), &serialized_events)?
            .send()
            .await?;
        if !res.status().is_success() {
            log::warn!(res:?, serialized_events:%; "event request failed: {}", &serialized_events);
            return Err(ClientError::InvalidResponse(res))?;
        }
        Ok(res.text().await?)
    }

    fn mutate_body(&mut self, update: BodyUpdate) -> Result<(), WebDynproError> {
        Ok(self.body.apply(update)?)
    }
}

/// [`WebDynproClient`]을 생성하는 빌더
pub struct WebDynproClientBuilder<'a> {
    base_url: &'a str,
    name: &'a str,
    client: Option<reqwest::Client>,
}

impl<'a> WebDynproClientBuilder<'a> {
    /// 새로운 [`WebDynproClientBuilder`]를 만듭니다.
    pub fn new(base_url: &'a str, name: &'a str) -> WebDynproClientBuilder<'a> {
        WebDynproClientBuilder {
            base_url,
            name,
            client: None,
        }
    }

    /// 애플리케이션에 임의의 [`reqwest::Client`]를 추가합니다.
    pub fn client(mut self, client: reqwest::Client) -> WebDynproClientBuilder<'a> {
        self.client = Some(client);
        self
    }

    /// 새로운 [`WebDynproClient`]을 생성합니다.
    pub async fn build(self) -> Result<WebDynproClient, WebDynproError> {
        let base_url = Url::parse(self.base_url)
            .or(Err(ClientError::InvalidBaseUrl(self.base_url.to_string())))?;
        match self.client {
            Some(client) => Ok(WebDynproClient::with_client(base_url, self.name, client).await?),
            None => Ok(WebDynproClient::new(base_url, self.name).await?),
        }
    }
}

fn create_form_request_event(
    is_async: bool,
    focus_info: &str,
    hash: &str,
    dom_changed: bool,
    is_dirty: bool,
) -> Result<Event, EventBuilderError> {
    let mut form_parameters: HashMap<String, String> = HashMap::new();
    form_parameters.insert("Id".to_string(), "sap.client.SsrClient.form".to_string());
    form_parameters.insert("Async".to_string(), is_async.to_string());
    form_parameters.insert("FocusInfo".to_string(), focus_info.to_string());
    form_parameters.insert("Hash".to_string(), hash.to_string());
    form_parameters.insert("DomChanged".to_string(), dom_changed.to_string());
    form_parameters.insert("IsDirty".to_string(), is_dirty.to_string());
    EventBuilder::default()
        .control("Form".to_string())
        .event("Request".to_string())
        .parameters(form_parameters)
        .ucf_parameters(UcfParameters::default())
        .custom_parameters(HashMap::new())
        .build()
}

trait Requests {
    fn wd_navigate(&self, base_url: &Url, app_name: &str) -> RequestBuilder;

    fn wd_xhr(
        &self,
        base_url: &Url,
        ssr_client: &SapSsrClient,
        event_queue: &str,
    ) -> Result<RequestBuilder, ClientError>;
}

impl Requests for reqwest::Client {
    fn wd_navigate(&self, base_url: &Url, app_name: &str) -> RequestBuilder {
        let mut url = "".to_owned();
        url.push_str(base_url.as_str());
        if !url.ends_with('/') {
            url.push('/');
        }
        url.push_str(app_name);
        url.push_str("?sap-wd-stableids=X");
        self.get(url).headers(default_header())
    }

    fn wd_xhr(
        &self,
        base_url: &Url,
        ssr_client: &SapSsrClient,
        event_queue: &str,
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
            url.push(':');
            url.push_str(port.to_string().as_str());
        }
        url.push_str(ssr_client.action.as_str());
        let params = [
            ("sap-charset", ssr_client.charset.as_str()),
            ("sap-wd-secure-id", ssr_client.wd_secure_id.as_str()),
            ("fesrAppName", ssr_client.app_name.as_str()),
            (
                "fesrUseBeacon",
                if ssr_client.use_beacon {
                    "true"
                } else {
                    "false"
                },
            ),
            ("SAPEVENTQUEUE", event_queue),
        ];
        Ok(self.post(url).headers(wd_xhr_header()).form(&params))
    }
}

#[derive(Debug)]
pub(crate) struct SapSsrClient {
    action: String,
    charset: String,
    wd_secure_id: String,
    pub app_name: String,
    use_beacon: bool,
}

/// 전달받은 이벤트가 어떻게 처리되었는지 표현합니다.
pub enum EventProcessResult {
    /// 전달받은 이벤트가 큐에 추가되었을 경우
    Enqueued,
    /// 전달받은 이벤트가 큐에 추가된 후 서버에 전송되었을 경우
    Sent,
}

/// WebDynpro의 페이지를 파싱, 업데이트하는 [`Body`] 구현
pub mod body;

#[cfg(test)]
mod test {
    use url::Url;

    use crate::webdynpro::client::WebDynproClientBuilder;

    #[tokio::test]
    async fn initial_load() {
        let mut client =
            WebDynproClientBuilder::new("https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/", "ZCMW2100")
                .build()
                .await
                .unwrap();
        client
            .navigate(
                &Url::parse("https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/").unwrap(),
                "ZCMW2100",
            )
            .await
            .unwrap();
        assert_eq!(client.body.ssr_client().app_name, "ZCMW2100");
    }
}
