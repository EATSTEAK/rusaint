use self::{
    body::{Body, BodyUpdate},
    client::Client,
};
use url::Url;

use super::{
    element::{define_elements, layout::Form},
    error::{ClientError, WebDynproError},
    event::Event,
};

/// 기본적인 WebDynpro 애플리케이션
pub struct BasicApplication {
    base_url: Url,
    name: String,
    client: Client,
    body: Body,
}

/// WebDynpro 애플리케이션의 기본 기능
pub trait Application {
    /// WebDynpro 애플리케이션의 이름을 반환합니다.
    fn name(&self) -> &str;

    /// WebDynpro 애플리케이션의 기본 URL을 반환합니다.
    fn base_url(&self) -> &Url;

    /// WebDynpro 애플리케이션의 페이지 문서를 반환합니다.
    fn body(&self) -> &Body;

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

impl Application for BasicApplication {
    fn name(&self) -> &str {
        &self.name
    }

    fn base_url(&self) -> &Url {
        &self.base_url
    }

    fn body(&self) -> &Body {
        &self.body
    }
}

impl<'a> BasicApplication {
    define_elements! {
        SSR_FORM: Form<'a> = "sap.client.SsrClient.form";
    }

    async fn with_client(
        base_url: Url,
        name: &str,
        mut client: Client,
    ) -> Result<Self, WebDynproError> {
        let body = { client.navigate(&base_url, name).await? };
        Ok(BasicApplication {
            base_url,
            name: name.to_owned(),
            client,
            body,
        })
    }

    /// WebDynpro 애플리케이션에 임의의 엘리먼트 이벤트를 보냅니다.
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
    pub async fn send_events(
        &mut self,
        events: impl IntoIterator<Item = Event>,
    ) -> Result<(), WebDynproError> {
        let body = self.body();
        let form_req = Self::SSR_FORM
            .from_body(body)?
            .request(false, "", "", false, false)
            .or(Err(ClientError::NoSuchForm(
                Self::SSR_FORM.id().to_string(),
            )))?;
        for event in events.into_iter() {
            if !event.is_enqueable() && event.is_submitable() {
                {
                    self.client.add_event(event);
                    self.client.add_event(form_req.to_owned());
                }
                let update = {
                    self.client
                        .send_event(&self.base_url, self.body.ssr_client())
                        .await?
                };
                self.mutate_body(update)?
            } else {
                self.client.add_event(event);
            }
        }
        Ok(())
    }

    fn mutate_body(&mut self, update: BodyUpdate) -> Result<(), WebDynproError> {
        Ok(self.body.apply(update)?)
    }
}

/// [`BasicApplication`]을 생성하는 빌더
pub struct BasicApplicationBuilder<'a> {
    base_url: &'a str,
    name: &'a str,
    client: Option<Client>,
}

impl<'a> BasicApplicationBuilder<'a> {
    /// 새로운 [`BasicApplicationBuilder`]를 만듭니다.
    pub fn new(base_url: &'a str, name: &'a str) -> BasicApplicationBuilder<'a> {
        BasicApplicationBuilder {
            base_url,
            name,
            client: None,
        }
    }

    /// 애플리케이션에 임의의 [`Client`]를 추가합니다.
    pub fn client(mut self, client: Client) -> BasicApplicationBuilder<'a> {
        self.client = Some(client);
        self
    }

    /// 새로운 [`BasicApplication`]을 생성합니다.
    pub async fn build(self) -> Result<BasicApplication, WebDynproError> {
        let client = match self.client {
            Some(client) => client,
            None => Client::new(),
        };
        let base_url = Url::parse(self.base_url)
            .or(Err(ClientError::InvalidBaseUrl(self.base_url.to_string())))?;
        Ok(BasicApplication::with_client(base_url, self.name, client).await?)
    }
}

pub(crate) struct SapSsrClient {
    action: String,
    charset: String,
    wd_secure_id: String,
    pub app_name: String,
    use_beacon: bool,
}

/// WebDynpro 요청 및 문서 처리를 담당하는 클라이언트
pub mod client;

/// WebDynpro의 페이지를 파싱, 업데이트하는 [`Body`] 구현
pub mod body;
