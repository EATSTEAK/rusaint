use anyhow::Result;
use reqwest::cookie::Jar;
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use url::Url;

use crate::webdynpro::{
    application::{
        client::{default_header, Client, USER_AGENT},
        BasicApplication,
    },
    element::{
        client_inspector::ClientInspector,
        custom::{Custom, CustomClientInfo},
        element_ref,
        loading_placeholder::LoadingPlaceholder,
    },
    error::ClientError,
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
        jar.add_cookie_str(
            format!("sToken={}; domain=.ssu.ac.kr; path=/; secure", sso_token).as_str(),
            &Url::parse("https://smartid.ssu.ac.kr")?,
        );
        let client = reqwest::Client::builder()
            .cookie_provider(jar)
            .cookie_store(true)
            .user_agent(USER_AGENT)
            .build()
            .unwrap();
        let res = client
            .get(format!(
                "{}?sToken={}&sIdno={}",
                SSU_USAINT_SSO_URL, sso_token, sso_idno
            ))
            .headers(default_header())
            .send()
            .await?;
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
