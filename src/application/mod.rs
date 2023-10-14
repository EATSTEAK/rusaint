use std::sync::Arc;
use url::Url;

use crate::{
    session::USaintSession,
    utils::DEFAULT_USER_AGENT,
    webdynpro::{
        application::{
            client::{body::Body, Client},
            Application, BasicApplication,
        },
        element::{
            define_elements,
            system::{ClientInspector, Custom, CustomClientInfo, LoadingPlaceholder},
        },
        error::{ClientError, WebDynproError},
        event::Event,
    },
};

/// 새로운 u-saint 애플리케이션을 만듭니다.
///
/// ### 예시
/// ```no_run
/// # use std::sync::Arc;
/// # use rusaint::define_usaint_application;
/// define_usaint_application(pub struct ExampleApplication);
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
        $vis:vis struct $name:ident
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

            fn body(&self) -> &$crate::webdynpro::application::client::body::Body {
                self.0.body()
            }
        }

        impl $name {
            #[allow(unused)]
            async fn send_events(
                &mut self,
                events: impl IntoIterator<Item = $crate::webdynpro::event::Event>,
            ) -> Result<(), WebDynproError> {
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

    /// 새로운 u-saint 애플리케이션을 만듭니다. 이렇게 만들어진 애플리케이션은 익명 세션을 갖습니다.
    /// ### 예시
    /// ```
    /// // 학기시간표 애플리케이션(로그인 없이 접근 가능)
    /// let app = USaintApplication::new("ZCMW2100").await.unwrap();
    /// ```
    /// ```should_panic
    /// // 학적정보 애플리케이션(세션이 없으므로 접속 불가)
    /// let app = USaintApplication::new("ZCMW1001n").await.unwrap();
    /// ```
    /// 로그인된 세션이 필요한 애플리케이션에선 이용해서는 안됩니다.
    pub async fn new(app_name: &str) -> Result<USaintApplication, WebDynproError> {
        let mut app =
            USaintApplication(BasicApplication::new(SSU_WEBDYNPRO_BASE_URL, app_name).await?);
        app.load_placeholder().await?;
        Ok(app)
    }

    /// 세션이 포함된 u-saint 애플리케이션을 만듭니다.
    /// ### 예시
    /// ```no_run
    /// # use std::sync::Arc;
    /// # use rusaint::USaintSession;
    /// // 사용자 학번, 비밀번호로부터 세션 생성
    /// let session = Arc::new(USaintSession::with_password("20212345", "password!").await.unwrap());
    /// // 애플리케이션 생성(로그인 되었으므로 접속 가능)
    /// let app = USaintApplication::with_session("ZCMW1001n", session).await.unwrap();
    /// ```
    pub async fn with_session(
        app_name: &str,
        session: Arc<USaintSession>,
    ) -> Result<USaintApplication, WebDynproError> {
        let base_url = Url::parse(SSU_WEBDYNPRO_BASE_URL).or(Err(ClientError::InvalidBaseUrl(
            SSU_WEBDYNPRO_BASE_URL.to_string(),
        )))?;
        let r_client = reqwest::Client::builder()
            .cookie_provider(session)
            .user_agent(DEFAULT_USER_AGENT)
            .build()
            .unwrap();
        let client = Client::with_client(r_client, &base_url, app_name).await?;
        let mut app = USaintApplication(BasicApplication::with_client(base_url, app_name, client)?);
        app.load_placeholder().await?;
        Ok(app)
    }
    /// 이벤트를 서버에 전송합니다. [`send_events`] 를 참조하세요.
    ///
    /// [`send_events`] : webdynpro::application::BasicApplication::send_events
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
/// 학생 성적 조회: [`CourseGrades`](course_grades::CourseGrades)
pub mod course_grades;
mod course_schedule;
mod student_information;

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use crate::{application::USaintApplication, session::USaintSession};
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_sso_login() {
        dotenv().ok();
        let id = std::env::var("SSO_ID").unwrap();
        let password = std::env::var("SSO_PASSWORD").unwrap();
        let session = Arc::new(USaintSession::with_password(&id, &password).await.unwrap());
        USaintApplication::with_session("ZCMW1001n", session)
            .await
            .unwrap();
    }
}
