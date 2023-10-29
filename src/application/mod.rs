use std::sync::Arc;
use url::Url;

use crate::{
    session::USaintSession,
    utils::DEFAULT_USER_AGENT,
    webdynpro::{
        application::{
            body::Body, client::Client, Application, BasicApplication, BasicApplicationBuilder,
        },
        element::{
            define_elements,
            system::{ClientInspector, Custom, CustomClientInfo, LoadingPlaceholder},
        },
        error::WebDynproError,
        event::Event,
    },
};

/// 새로운 u-saint 애플리케이션을 만듭니다.
///
/// ### 예시
/// ```no_run
/// # use std::sync::Arc;
/// # use rusaint::define_usaint_application;
/// define_usaint_application!(pub struct ExampleApplication);
///
/// impl<'a> ExampleApplication {
///     const APP_NAME: &str = "ZCMW1001n";
///
///     // 엘리먼트를 정의하는 매크로
///     define_elements! {
///         // 담당자문의 정보에 해당하는 캡션의 ID 정의
///         CAPTION: Caption<'a> = "ZCMW_DEVINFO_RE.ID_D080C16F227F4D68751326DC40BB6BE0:MAIN.CAPTION"
///     }
///
///     pub async fn new(session: Arc<USaintSession>) -> Result<ExampleApplication, WebDynproError> {
///         Ok(ExampleApplication(
///             USaintApplication::with_session(Self::APP_NAME, session).await?,
///        ))
///     }
/// }
///
/// async fn test() -> Result<(), dyn Error> {
///     let session = Arc::new(USaintSession::with_password("20212345", "password").await?);
///     let app = ExampleApplication::new(session).await?;
///     let caption = ExampleApplication::CAPTION.from_body(app.body())?;
///     // Some("담당자문의 정보");
///     println!("{:?}", caption.text());
///     
/// }
/// ```
#[macro_export]
macro_rules! define_usaint_application {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident<$app_name:literal>
    ) => {
        $(#[$attr])*
        $vis struct $name($crate::application::USaintApplication);

        impl $crate::webdynpro::application::Application for $name {
            fn name(&self) -> &str {
                self.0.name()
            }

            fn base_url(&self) -> &url::Url {
                self.0.base_url()
            }

            fn body(&self) -> &$crate::webdynpro::application::body::Body {
                self.0.body()
            }
        }

        impl From<USaintApplication> for $name {
            fn from(value: USaintApplication) -> Self {
                $name(value)
            }
        }

        impl $crate::application::PredefinedUSaintApplication for $name {
            const APP_NAME: &'static str = $app_name;
        }

        impl $name {
            #[allow(unused)]
            async fn send_events(
                &mut self,
                events: impl IntoIterator<Item = $crate::webdynpro::event::Event>,
            ) -> Result<(), $crate::webdynpro::error::WebDynproError> {
                self.0.send_events(events).await
            }
        }
    };
}

pub use define_usaint_application;

const SSU_WEBDYNPRO_BASE_URL: &str = "https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/";
const INITIAL_CLIENT_DATA_WD01: &str = "ClientWidth:1920px;ClientHeight:1000px;ScreenWidth:1920px;ScreenHeight:1080px;ScreenOrientation:landscape;ThemedTableRowHeight:33px;ThemedFormLayoutRowHeight:32px;ThemedSvgLibUrls:{\"SAPGUI-icons\":\"https://ecc.ssu.ac.kr:8443/sap/public/bc/ur/nw5/themes/~cache-20210223121230/Base/baseLib/sap_fiori_3/svg/libs/SAPGUI-icons.svg\",\"SAPWeb-icons\":\"https://ecc.ssu.ac.kr:8443/sap/public/bc/ur/nw5/themes/~cache-20210223121230/Base/baseLib/sap_fiori_3/svg/libs/SAPWeb-icons.svg\"};ThemeTags:Fiori_3,Touch;ThemeID:sap_fiori_3;SapThemeID:sap_fiori_3;DeviceType:DESKTOP";
const INITIAL_CLIENT_DATA_WD02: &str = "ThemedTableRowHeight:25px";
/// u-saint에 접속하기 위한 기본 애플리케이션
pub struct USaintApplication(BasicApplication);

impl From<BasicApplication> for USaintApplication {
    fn from(value: BasicApplication) -> Self {
        USaintApplication(value)
    }
}

impl Application for USaintApplication {
    fn name(&self) -> &str {
        self.0.name()
    }

    fn base_url(&self) -> &Url {
        self.0.base_url()
    }

    fn body(&self) -> &Body {
        self.0.body()
    }
}

impl<'a> USaintApplication {
    define_elements! {
        CLIENT_INSPECTOR_WD01: ClientInspector<'a> = "WD01";
        CLIENT_INSPECTOR_WD02: ClientInspector<'a> = "WD02";
        LOADING_PLACEHOLDER: LoadingPlaceholder<'a> = "_loadingPlaceholder_";
    }

    const CUSTOM: Custom = Custom::new(std::borrow::Cow::Borrowed("WD01"));

    fn new(app: BasicApplication) -> USaintApplication {
        USaintApplication(app)
    }

    /// 이벤트를 서버에 전송합니다. [`send_events()`](crate::webdynpro::application::BasicApplication::send_events)를 참조하세요.
    pub async fn send_events(
        &mut self,
        events: impl IntoIterator<Item = Event>,
    ) -> Result<(), WebDynproError> {
        self.0.send_events(events).await
    }

    async fn load_placeholder(&mut self) -> Result<(), WebDynproError> {
        let events = {
            let body = self.body();
            let wd01 = Self::CLIENT_INSPECTOR_WD01.from_body(body)?;
            let wd02 = Self::CLIENT_INSPECTOR_WD02.from_body(body)?;
            let load_ph = Self::LOADING_PLACEHOLDER.from_body(body)?;
            let client_infos = Self::CUSTOM.client_infos(CustomClientInfo {
                client_url: self.client_url(),
                document_domain: "ssu.ac.kr".to_owned(),
                ..CustomClientInfo::default()
            });
            vec![
                wd01.notify(INITIAL_CLIENT_DATA_WD01)?,
                wd02.notify(INITIAL_CLIENT_DATA_WD02)?,
                load_ph.load()?,
                client_infos,
            ]
        };
        self.send_events(events).await
    }
}

pub trait PredefinedUSaintApplication: From<USaintApplication> {
    const APP_NAME: &'static str;
}

pub struct USaintApplicationBuilder {
    session: Option<Arc<USaintSession>>,
}

impl USaintApplicationBuilder {
    pub fn new() -> USaintApplicationBuilder {
        USaintApplicationBuilder { session: None }
    }

    pub fn session(mut self, session: Arc<USaintSession>) -> USaintApplicationBuilder {
        self.session = Some(session);
        self
    }

    pub async fn build_into<T: PredefinedUSaintApplication>(self) -> Result<T, WebDynproError> {
        let name = T::APP_NAME;
        Ok(self.build(name).await?.into())
    }

    pub async fn build(self, name: &str) -> Result<USaintApplication, WebDynproError> {
        let mut builder = BasicApplicationBuilder::new(SSU_WEBDYNPRO_BASE_URL, name);
        if let Some(session) = self.session {
            let r_client = reqwest::Client::builder()
                .cookie_provider(session)
                .user_agent(DEFAULT_USER_AGENT)
                .build()
                .unwrap();
            let client = Client::with_client(r_client);
            builder = builder.client(client);
        }
        let base_app = builder.build().await?;
        let mut app = USaintApplication::new(base_app);
        app.load_placeholder().await?;
        Ok(app)
    }
}
/// 학생 성적 조회: [`CourseGrades`](course_grades::CourseGrades)
pub mod course_grades;
mod course_schedule;
mod student_information;
