use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use url::Url;

use crate::{
    session::USaintSession,
    utils::DEFAULT_USER_AGENT,
    webdynpro::{
        application::{client::Client, BasicApplication},
        element::{
            define_elements,
            system::{
                client_inspector::ClientInspector,
                custom::{Custom, CustomClientInfo},
                loading_placeholder::LoadingPlaceholder,
            },
        },
        error::{ClientError, WebDynproError},
    },
};

const SSU_WEBDYNPRO_BASE_URL: &str = "https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/";
const INITIAL_CLIENT_DATA_WD01: &str = "ClientWidth:1920px;ClientHeight:1000px;ScreenWidth:1920px;ScreenHeight:1080px;ScreenOrientation:landscape;ThemedTableRowHeight:33px;ThemedFormLayoutRowHeight:32px;ThemedSvgLibUrls:{\"SAPGUI-icons\":\"https://ecc.ssu.ac.kr:8443/sap/public/bc/ur/nw5/themes/~cache-20210223121230/Base/baseLib/sap_fiori_3/svg/libs/SAPGUI-icons.svg\",\"SAPWeb-icons\":\"https://ecc.ssu.ac.kr:8443/sap/public/bc/ur/nw5/themes/~cache-20210223121230/Base/baseLib/sap_fiori_3/svg/libs/SAPWeb-icons.svg\"};ThemeTags:Fiori_3,Touch;ThemeID:sap_fiori_3;SapThemeID:sap_fiori_3;DeviceType:DESKTOP";
const INITIAL_CLIENT_DATA_WD02: &str = "ThemedTableRowHeight:25px";
/// u-saint에 접속하기 위한 기본 애플리케이션
pub struct USaintApplication(BasicApplication);

impl Deref for USaintApplication {
    type Target = BasicApplication;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for USaintApplication {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> USaintApplication {
    define_elements! {
        CLIENT_INSPECTOR_WD01: ClientInspector<'a> = "WD01",
        CLIENT_INSPECTOR_WD02: ClientInspector<'a> = "WD02",
        LOADING_PLACEHOLDER: LoadingPlaceholder<'a> = "_loadingPlaceholder_"
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
/// [`CourseGrades`](course_grades::CourseGrades) 애플리케이션 모듈
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
