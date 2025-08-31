use std::sync::Arc;

use url::Url;
use wdpe::{
    body::Body,
    command::{
        WebDynproCommandExecutor as _,
        element::system::{
            ClientInspectorNotifyEventCommand, CustomClientInfoEventCommand,
            LoadingPlaceholderLoadEventCommand,
        },
    },
    define_elements,
    element::{
        parser::ElementParser,
        system::{ClientInspector, Custom, CustomClientInfo, LoadingPlaceholder},
    },
    error::WebDynproError,
    event::{Event, event_queue::EnqueueEventResult},
    requests::WebDynproRequests as _,
    state::{EventProcessResult, WebDynproState},
};

use crate::{RusaintError, USaintSession, utils::DEFAULT_USER_AGENT};

const SSU_WEBDYNPRO_BASE_URL: &str = "https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/";
const INITIAL_CLIENT_DATA_WD01: &str = "ClientWidth:1920px;ClientHeight:1000px;ScreenWidth:1920px;ScreenHeight:1080px;ScreenOrientation:landscape;ThemedTableRowHeight:33px;ThemedFormLayoutRowHeight:32px;ThemedSvgLibUrls:{\"SAPGUI-icons\":\"https://ecc.ssu.ac.kr:8443/sap/public/bc/ur/nw5/themes/~cache-20210223121230/Base/baseLib/sap_fiori_3/svg/libs/SAPGUI-icons.svg\",\"SAPWeb-icons\":\"https://ecc.ssu.ac.kr:8443/sap/public/bc/ur/nw5/themes/~cache-20210223121230/Base/baseLib/sap_fiori_3/svg/libs/SAPWeb-icons.svg\"};ThemeTags:Fiori_3,Touch;ThemeID:sap_fiori_3;SapThemeID:sap_fiori_3;DeviceType:DESKTOP";
const INITIAL_CLIENT_DATA_WD02: &str = "ThemedTableRowHeight:25px";
/// u-saint에 접속하기 위한 기본 클라이언트
#[derive(Debug)]
pub struct USaintClient {
    state: WebDynproState,
    client: reqwest::Client,
}

impl<'a> USaintClient {
    define_elements! {
        CLIENT_INSPECTOR_WD01: ClientInspector<'a> = "WD01";
        CLIENT_INSPECTOR_WD02: ClientInspector<'a> = "WD02";
        LOADING_PLACEHOLDER: LoadingPlaceholder<'a> = "_loadingPlaceholder_";
    }

    const CUSTOM: Custom = Custom::new(std::borrow::Cow::Borrowed("WD01"));

    async fn new(
        state: WebDynproState,
        client: reqwest::Client,
    ) -> Result<USaintClient, WebDynproError> {
        let mut client = USaintClient { state, client };
        client.load_placeholder().await?;
        Ok(client)
    }

    /// WebDynpro 애플리케이션의 이름을 반환합니다.
    pub fn name(&self) -> &str {
        self.state.name()
    }

    /// WebDynpro 애플리케이션의 기본 URL을 반환합니다.
    pub fn base_url(&self) -> &Url {
        self.state.base_url()
    }

    /// WebDynpro 애플리케이션의 페이지 문서를 반환합니다.
    pub fn body(&self) -> &Body {
        self.state.body()
    }

    /// 실제로 요청하는 애플리케이션의 URL을 반환합니다.
    pub fn client_url(&self) -> String {
        self.state.client_url()
    }

    /// 이벤트를 처리합니다. [`process_event()`](WebDynproClient::process_event)를 참조하세요.
    pub async fn process_event(
        &mut self,
        force_send: bool,
        event: Event,
    ) -> Result<EventProcessResult, WebDynproError> {
        let enqueue_result = self.state.add_event(event).await;

        if (matches!(enqueue_result, EnqueueEventResult::ShouldProcess)) || force_send {
            let serialized_events = self.state.serialize_and_clear_with_form_event().await?;
            let update = {
                self.client
                    .send_events(
                        self.state.base_url(),
                        self.state.body().ssr_client(),
                        &serialized_events,
                    )
                    .await?
            };
            self.state.mutate_body(update)?;
            Ok(EventProcessResult::Sent)
        } else {
            Ok(EventProcessResult::Enqueued)
        }
    }

    async fn load_placeholder(&mut self) -> Result<(), WebDynproError> {
        let parser = ElementParser::new(self.body());
        let notify_wd01 = parser.read(ClientInspectorNotifyEventCommand::new(
            Self::CLIENT_INSPECTOR_WD01,
            INITIAL_CLIENT_DATA_WD01,
        ))?;
        let notify_wd02 = parser.read(ClientInspectorNotifyEventCommand::new(
            Self::CLIENT_INSPECTOR_WD02,
            INITIAL_CLIENT_DATA_WD02,
        ))?;
        let load = parser.read(LoadingPlaceholderLoadEventCommand::new(
            Self::LOADING_PLACEHOLDER,
        ))?;
        let custom = parser.read(CustomClientInfoEventCommand::new(
            Self::CUSTOM,
            CustomClientInfo {
                client_url: self.client_url(),
                document_domain: "ssu.ac.kr".to_owned(),
                ..CustomClientInfo::default()
            },
        ))?;
        self.process_event(false, notify_wd01).await?;
        self.process_event(false, notify_wd02).await?;
        self.process_event(false, load).await?;
        self.process_event(false, custom).await?;
        Ok(())
    }
}

/// U-Saint 애플리케이션이 구현하는 트레이트
pub trait USaintApplication: Sized {
    /// U-Saint WebDynpro 애플리케이션 이름
    const APP_NAME: &'static str;

    /// U-Saint 클라이언트를 애플리케이션으로 변환합니다.
    fn from_client(client: USaintClient) -> Result<Self, RusaintError>;
}

/// 새로운 [`USaintClient`]를 생성하는 빌더
pub struct USaintClientBuilder {
    session: Option<Arc<USaintSession>>,
}

impl USaintClientBuilder {
    /// 새로운 빌더를 만듭니다.
    pub fn new() -> USaintClientBuilder {
        USaintClientBuilder { session: None }
    }

    /// 빌더에 [`USaintSession`]을 추가합니다.
    pub fn session(mut self, session: Arc<USaintSession>) -> USaintClientBuilder {
        self.session = Some(session);
        self
    }

    /// 애플리케이션 이름과 함께 [`USaintClient`]을 생성합니다.
    pub async fn build(self, name: &str) -> Result<USaintClient, WebDynproError> {
        let base_url = Url::parse(SSU_WEBDYNPRO_BASE_URL).unwrap();
        let client = if let Some(session) = self.session {
            reqwest::Client::builder()
                .cookie_provider(session)
                .user_agent(DEFAULT_USER_AGENT)
                .build()
                .unwrap()
        } else {
            reqwest::Client::builder()
                .user_agent(DEFAULT_USER_AGENT)
                .build()
                .unwrap()
        };
        let body = client.navigate(&base_url, name).await?;
        let state = WebDynproState::new(base_url, name.to_string(), body);
        USaintClient::new(state, client).await
    }

    /// 특정 [`USaintApplication`]을 만듭니다.
    pub async fn build_into<T: USaintApplication>(self) -> Result<T, RusaintError> {
        let name = T::APP_NAME;
        let client = self.build(name).await?;
        T::from_client(client)
    }
}

impl Default for USaintClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
