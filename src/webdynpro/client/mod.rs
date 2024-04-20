use crate::{
    define_elements,
    utils::{default_header, DEFAULT_USER_AGENT},
    webdynpro::{
        element::layout::Form,
        error::{ClientError, WebDynproError},
        event::{event_queue::EventQueue, Event},
    },
};
use body::{Body, BodyUpdate};
use reqwest::{cookie::Jar, header::*, RequestBuilder};
use std::sync::Arc;
use url::Url;

use super::command::WebDynproCommand;

/// WebDynpro 애플리케이션의 웹 요청 및 페이지 문서 처리를 담당하는 클라이언트
pub struct WebDynproClient {
    base_url: Url,
    name: String,
    body: Body,
    client: reqwest::Client,
    event_queue: EventQueue,
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

impl<'a> WebDynproClient {
    define_elements! {
        SSR_FORM: Form<'a> = "sap.client.SsrClient.form";
    }

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
        url.push_str(&self.base_url().as_str());
        if !url.ends_with('/') {
            url.push_str("/");
        }
        url.push_str(&self.name());
        url.push_str("?sap-wd-stableids=X#");
        url
    }

    /// 새로운 클라이언트를 생성합니다.
    pub async fn new(base_url: Url, name: &str) -> Result<WebDynproClient, ClientError> {
        let jar: Arc<Jar> = Arc::new(Jar::default());
        let client = reqwest::Client::builder()
            .cookie_provider(jar)
            .cookie_store(true)
            .user_agent(DEFAULT_USER_AGENT)
            .build()
            .unwrap();
        Self::with_client(base_url, name, client).await
    }

    /// 임의의 reqwest::Client 와 함께 클라이언트를 생성합니다.
    pub async fn with_client(
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
            event_queue: EventQueue::new(),
        })
    }

    /// WebDynpro 클라이언트에 명령을 전송합니다.
    pub async fn send<T: WebDynproCommand>(
        &mut self,
        command: T,
    ) -> Result<T::Result, WebDynproError> {
        command.dispatch(self).await
    }

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
    pub(crate) async fn process_event(
        &mut self,
        force_send: bool,
        event: Event,
    ) -> Result<EventProcessResult, WebDynproError> {
        let form_req = Self::SSR_FORM
            .from_body(self.body())?
            .request(false, "", "", false, false)
            .or(Err(ClientError::NoSuchForm(
                Self::SSR_FORM.id().to_string(),
            )))?;
        if !event.is_enqueable() && event.is_submitable() {
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
        self.event_queue.add(event)
    }

    /// 이벤트 큐 내의 이벤트를 전송하고 그 결과를 `BodyUpdate`로 반환합니다.
    async fn send_events(&mut self) -> Result<BodyUpdate, WebDynproError> {
        let res = self.event_request().await?;
        Ok(BodyUpdate::new(&res)?)
    }

    /// 이벤트 큐 내부 내용을 서버에 전송하고 응답을 받습니다.
    async fn event_request(&mut self) -> Result<String, ClientError> {
        let res = self
            .client
            .wd_xhr(
                &self.base_url,
                self.body.ssr_client(),
                &mut self.event_queue,
            )?
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(ClientError::InvalidResponse(res))?;
        }
        Ok(res.text().await?)
    }

    fn mutate_body(&mut self, update: BodyUpdate) -> Result<(), WebDynproError> {
        Ok(self.body.apply(update)?)
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

pub(crate) struct SapSsrClient {
    action: String,
    charset: String,
    wd_secure_id: String,
    pub app_name: String,
    use_beacon: bool,
}

pub enum EventProcessResult {
    Enqueued,
    Sent
}

pub mod body;

#[cfg(test)]
mod test {
    use url::Url;

    use crate::webdynpro::client::WebDynproClient;

    // #[tokio::test]
    // async fn initial_load() {
    //     let mut client = BasicWDClient::new("https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/", "ZCMW2100");
    //     let body = client
    //         .navigate(
    //             &Url::parse("https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/").unwrap(),
    //             "ZCMW2100",
    //         )
    //         .await
    //         .unwrap();
    //     assert_eq!(body.ssr_client().app_name, "ZCMW2100");
    // }
}
