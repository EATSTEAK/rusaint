use crate::{
    define_elements,
    utils::{default_header, DEFAULT_USER_AGENT},
    webdynpro::{
        error::{ClientError, WebDynproError},
        event::{event_queue::EventQueue, Event},
        element::layout::Form,
    },
};
use body::{Body, BodyUpdate};
use reqwest::{cookie::Jar, header::*, RequestBuilder};
use std::sync::Arc;
use url::Url;

/// WebDynpro 애플리케이션의 웹 요청 및 페이지 문서 처리를 담당하는 클라이언트
pub struct BasicWDClient {
    base_url: Url,
    name: String,
    body: Option<Body>,
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

/// WebDynpro 애플리케이션의 기본 기능
pub trait WebDynproClient {
    /// WebDynpro 애플리케이션의 이름을 반환합니다.
    fn name(&self) -> &str;

    /// WebDynpro 애플리케이션의 기본 URL을 반환합니다.
    fn base_url(&self) -> &Url;

    /// WebDynpro 애플리케이션의 페이지 문서를 반환합니다.
    fn body(&self) -> Option<&Body>;

    /// 실제로 요청하는 애플리케이션의 URL을 반환합니다.
    fn client_url(&self) -> String {
        let mut url = "".to_owned();
        url.push_str(&self.base_url().as_str());
        if !url.ends_with('/') {
            url.push_str("/");
        }
        url.push_str(&self.name());
        url.push_str("?sap-wd-stableids=X#");
        url
    }
}

impl WebDynproClient for BasicWDClient {
    fn name(&self) -> &str {
        &self.name
    }

    fn base_url(&self) -> &Url {
        &self.base_url
    }

    fn body(&self) -> Option<&Body> {
        (&self.body).as_ref()
    }
}

impl<'a> BasicWDClient {
    define_elements! {
        SSR_FORM: Form<'a> = "sap.client.SsrClient.form";
    }

    /// 새로운 클라이언트를 생성합니다.
    pub fn new(base_url: Url, name: &str) -> BasicWDClient {
        let jar: Arc<Jar> = Arc::new(Jar::default());
        let client = reqwest::Client::builder()
            .cookie_provider(jar)
            .cookie_store(true)
            .user_agent(DEFAULT_USER_AGENT)
            .build()
            .unwrap();
        Self::with_client(base_url, name, client)
    }

    /// 임의의 reqwest::Client 와 함께 클라이언트를 생성합니다.
    pub fn with_client(base_url: Url, name: &str, client: reqwest::Client) -> BasicWDClient {
        BasicWDClient {
            base_url,
            name: name.to_owned(),
            body: None,
            client,
            event_queue: EventQueue::new(),
        }
    }

    pub(crate) async fn navigate(
        &mut self,
        base_url: &Url,
        app_name: &str,
    ) -> Result<(), ClientError> {
        let raw_body = self
            .client
            .wd_navigate(base_url, app_name)
            .send()
            .await?
            .text()
            .await?;
        self.body = Some(Body::new(raw_body)?);
        Ok(())
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

    /// WebDynpro 클라이언트에 임의의 엘리먼트 이벤트를 보냅니다.
    ///
    /// > | **주의** |
    /// > `send_events()` 함수는 [`Body`]의 변경 가능한 레퍼런스를 가져오므로 [`Body`]의 참조가 남아있을 경우 작동하지 않습니다(엘리먼트 등).
    /// > 엘리먼트의 이벤트를 만드려면 엘리먼트가 `send_events()`함수를 호출 할 때 살아있지 않도록 생명주기를 관리하십시오.
    /// ### 예시
    /// ```ignore
    /// # tokio_test::block_on(async {
    /// # use std::sync::Arc;
    /// # use rusaint::application::USaintApplicationBuilder;
    /// # use rusaint::webdynpro::element::{ElementDef, selection::ComboBox};
    /// const PERIOD_YEAR: ElementDef<'_, ComboBox<'_>> = ElementDef::new("ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERYR");
    /// # let app = USaintApplicationBuilder::new().build("ZCMW2100").await.unwrap();
    /// let select_event = {
    ///     // body를 참조하는 변수를 격리
    ///     let elem = PERIOD_YEAR.from_body(app.body()).unwrap();
    ///     elem.select("2022").unwrap()
    /// };
    /// // app: BasicApplication
    /// app.send_events(vec![select_event]).await.unwrap();
    /// # })
    pub(crate) async fn send_events(
        &mut self,
        events: impl IntoIterator<Item = Event>,
    ) -> Result<(), WebDynproError> {
        if self.body().is_none() {
          self.navigate(&self.base_url, &self.name).await?;
        }
        let body = self.body().unwrap();
        let form_req = Self::SSR_FORM
            .from_body(body)?
            .request(false, "", "", false, false)
            .or(Err(ClientError::NoSuchForm(
                Self::SSR_FORM.id().to_string(),
            )))?;
        for event in events.into_iter() {
            if !event.is_enqueable() && event.is_submitable() {
                {
                    self.add_event(event);
                    self.add_event(form_req.to_owned());
                }
                let update = {
                    self.send_event(&self.base_url, self.body.ssr_client())
                        .await?
                };
                self.mutate_body(update)?
            } else {
                self.add_event(event);
            }
        }
        Ok(())
    }

    fn mutate_body(&mut self, update: BodyUpdate) -> Result<(), WebDynproError> {
        Ok(self.body.unwrap().apply(update)?)
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

pub mod body;

#[cfg(test)]
mod test {
    use url::Url;

    use crate::webdynpro::client::BasicWDClient;

    #[tokio::test]
    async fn initial_load() {
        let mut client = BasicWDClient::new("https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/", "ZCMW2100");
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