use self::client::{body::Body, Client};
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
}

impl<'a> BasicApplication {
    define_elements! {
        SSR_FORM: Form<'a> = "sap.client.SsrClient.form"
    }

    /// WebDynpro 애플리케이션이 제공되는 Base URL과 애플리케이션 이름을 제공하여 새로운 애플리케이션을 생성합니다.
    /// ### 예시
    /// ```
    /// # tokio_test::block_on(async {
    /// BasicApplication::new("https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP", "ZCMW2100").await.unwrap();
    /// # })
    /// ```
    pub async fn new(base_url_str: &str, name: &str) -> Result<Self, WebDynproError> {
        let base_url = Url::parse(base_url_str)
            .or(Err(ClientError::InvalidBaseUrl(base_url_str.to_string())))?;
        let client = Client::new(&base_url, name).await?;
        Ok(Self::with_client(base_url, name, client)?)
    }

    /// 임의의 WebDynpro [`Client`]와 함께 애플리케이션을 생성합니다.
    /// ### 예시
    /// ```
    /// # tokio_test::block_on(async {
    /// # use self::client::Client;
    /// # use url::Url;
    /// let url = Url::parse("https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP").unwrap();
    /// let client = Client::new(url, "ZCMW2100").await.unwrap();
    /// BasicApplication::with_client(url, "ZCMW2100", client).await.unwrap();
    /// # })
    /// ```
    pub fn with_client(base_url: Url, name: &str, client: Client) -> Result<Self, WebDynproError> {
        Ok(BasicApplication {
            base_url,
            name: name.to_owned(),
            client,
        })
    }

    pub(crate) fn client_url(&self) -> String {
        let mut url = "".to_owned();
        url.push_str(&self.base_url.as_str());
        if !url.ends_with('/') {
            url.push_str("/");
        }
        url.push_str(&self.name);
        url.push_str("?sap-wd-stableids=X#");
        url
    }

    /// WebDynpro 애플리케이션에 임의의 엘리먼트 이벤트를 보냅니다.
    ///
    /// > | **주의** |
    /// > `send_events()` 함수는 [`Body`]의 변경 가능한 레퍼런스를 가져오므로 [`Body`]의 참조가 남아있을 경우 작동하지 않습니다(엘리먼트 등).
    /// > 엘리먼트의 이벤트를 만드려면 엘리먼트가 `send_events()`함수를 호출 할 때 살아있지 않도록 생명주기를 관리하십시오.
    /// ### 예시
    /// ```
    /// # tokio_test::block_on(async {
    /// # use std::sync::Arc;
    /// # use rusaint::application::USaintApplication;
    /// # use rusaint::webdynpro::element::{ElementDef, selection::combo_box::ComboBox};
    /// const PERIOD_YEAR: ElementDef<'_, ComboBox<'_>> = ElementDef::new("ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERYR");
    /// # let app = USaintApplication::new("ZCMW2100").await.unwrap();
    /// let select_event = {
    ///     // body를 참조하는 변수를 격리
    ///     let elem = PERIOD_YEAR.from_body(app.body());
    ///     elem.select("2022").unwrap()
    /// };
    /// // app: BasicApplication
    /// app.send_events(vec![select_event]).await.unwrap();
    /// # })
    pub async fn send_events(&mut self, events: Vec<Event>) -> Result<(), WebDynproError> {
        let form_req = Self::SSR_FORM
            .from_body(&self.client.body)?
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
                {
                    self.client.send_event(&self.base_url).await?;
                }
            } else {
                self.client.add_event(event);
            }
        }
        Ok(())
    }

    /// WebDynpro 애플리케이션의 페이지 문서 구조체를 반환합니다.
    pub fn body(&self) -> &Body {
        &self.client.body
    }
}

/// WebDynpro 요청 및 문서 처리를 담당하는 클라이언트 모듈
pub mod client;
