use anyhow::Result;
use reqwest::{
    cookie::Jar,
    header::{HeaderValue, COOKIE, HOST},
};
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use url::Url;

use crate::{
    utils::{default_header, DEFAULT_USER_AGENT},
    webdynpro::{
        application::{client::Client, BasicApplication},
        element::{
            client_inspector::ClientInspector,
            custom::{Custom, CustomClientInfo},
            element_ref,
            loading_placeholder::LoadingPlaceholder,
        },
        error::ClientError,
    },
};

const SSU_USAINT_SSO_URL: &str = "https://saint.ssu.ac.kr/webSSO/sso.jsp";
const SSU_WEBDYNPRO_BASE_URL: &str = "https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/";
const INITIAL_CLIENT_DATA_WD01: &str = "ClientWidth:1920px;ClientHeight:1000px;ScreenWidth:1920px;ScreenHeight:1080px;ScreenOrientation:landscape;ThemedTableRowHeight:33px;ThemedFormLayoutRowHeight:32px;ThemedSvgLibUrls:{\"SAPGUI-icons\":\"https://ecc.ssu.ac.kr:8443/sap/public/bc/ur/nw5/themes/~cache-20210223121230/Base/baseLib/sap_fiori_3/svg/libs/SAPGUI-icons.svg\",\"SAPWeb-icons\":\"https://ecc.ssu.ac.kr:8443/sap/public/bc/ur/nw5/themes/~cache-20210223121230/Base/baseLib/sap_fiori_3/svg/libs/SAPWeb-icons.svg\"};ThemeTags:Fiori_3,Touch;ThemeID:sap_fiori_3;SapThemeID:sap_fiori_3;DeviceType:DESKTOP";
const INITIAL_CLIENT_DATA_WD02: &str = "ThemedTableRowHeight:25px";
pub struct BasicUSaintApplication(BasicApplication);

impl Deref for BasicUSaintApplication {
    type Target = BasicApplication;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for BasicUSaintApplication {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> BasicUSaintApplication {
    element_ref! {
        CLIENT_INSPECTOR_WD01:ClientInspector<'a> = "WD01",
        CLIENT_INSPECTOR_WD02:ClientInspector<'a> = "WD02",
        LOADING_PLACEHOLDER: LoadingPlaceholder<'a> = "_loadingPlaceholder_"
    }

    pub const CUSTOM: Custom = Custom::new(std::borrow::Cow::Borrowed("WD01"));

    pub async fn new(app_name: &str) -> Result<BasicUSaintApplication> {
        Ok(BasicUSaintApplication(
            BasicApplication::new(SSU_WEBDYNPRO_BASE_URL, app_name).await?,
        ))
    }

    async fn client_with_session(sso_idno: &str, sso_token: &str) -> Result<reqwest::Client> {
        let jar: Arc<Jar> = Arc::new(Jar::default());
        let client = reqwest::Client::builder()
            .cookie_provider(jar)
            .cookie_store(true)
            .user_agent(DEFAULT_USER_AGENT)
            .build()
            .unwrap();
        // Manually include WAF cookies because of bug in reqwest::cookie::Jar
        let portal = client
            .get("https://saint.ssu.ac.kr/irj/portal")
            .headers(default_header())
            .header(HOST, "saint.ssu.ac.kr".parse::<HeaderValue>().unwrap())
            .send()
            .await?;
        let waf = portal
            .cookies()
            .find(|cookie| cookie.name() == "WAF")
            .ok_or(ClientError::NoCookie)?;
        let waf_cookie_str = format!("WAF={}; domain=saint.ssu.ac.kr; path=/;", waf.value());
        let token_cookie_str = format!("sToken={}; domain=.ssu.ac.kr; path=/; secure", sso_token);
        let req = client
            .get(format!(
                "{}?sToken={}&sIdno={}",
                SSU_USAINT_SSO_URL, sso_token, sso_idno
            ))
            .query(&[("sToken", sso_token), ("sIdno", sso_idno)])
            .headers(default_header())
            .header(COOKIE, waf_cookie_str.parse::<HeaderValue>().unwrap())
            .header(COOKIE, token_cookie_str.parse::<HeaderValue>().unwrap())
            .header(HOST, "saint.ssu.ac.kr".parse::<HeaderValue>().unwrap())
            .build()?;
        let res = client.execute(req).await?;
        if res.cookies().any(|cookie| cookie.name() == "MYSAPSSO2") {
            Ok(client)
        } else {
            Err(ClientError::RequestFailed(res))?
        }
    }

    pub async fn with_auth(
        app_name: &str,
        id: &str,
        token: &str,
    ) -> Result<BasicUSaintApplication> {
        let base_url = Url::parse(SSU_WEBDYNPRO_BASE_URL)?;
        let r_client = Self::client_with_session(id, token).await?;
        let client = Client::with_client(r_client, &base_url, app_name).await?;
        Ok(BasicUSaintApplication(BasicApplication::with_client(
            base_url, app_name, client,
        )?))
    }

    pub async fn load_placeholder(&mut self) -> Result<()> {
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

pub mod course_schedule;
pub mod student_information;

#[cfg(test)]
mod test {
    use crate::{definitions::BasicUSaintApplication, utils::obtain_ssu_sso_token};
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_sso_login() {
        dotenv().ok();
        let id = std::env::var("SSO_ID").unwrap();
        let password = std::env::var("SSO_PASSWORD").unwrap();
        let token = obtain_ssu_sso_token(&id, &password).await.unwrap();
        println!("Got token: {}", &token);
        BasicUSaintApplication::with_auth("ZCMW1001n", &id, &token)
            .await
            .unwrap();
    }
}
