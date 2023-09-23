use std::ops::{Deref, DerefMut};

use crate::webdynpro::{
    application::BasicApplication,
    element::{
        client_inspector::ClientInspector,
        custom::{Custom, CustomClientInfo},
        loading_placeholder::LoadingPlaceholder,
        ElementDef,
    },
    error::ClientError,
};

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

impl BasicUSaintApplication {
    pub const CLIENT_INSPECTOR_WD01: ElementDef<ClientInspector> = ElementDef::new("WD01");
    pub const CUSTOM: Custom = Custom::new(std::borrow::Cow::Borrowed("WD01"));
    pub const CLIENT_INSPECTOR_WD02: ElementDef<ClientInspector> = ElementDef::new("WD02");
    pub const LOADING_PLACEHOLDER: ElementDef<LoadingPlaceholder> =
        ElementDef::new("_loadingPlaceholder_");

    pub async fn new(app_name: &str) -> Result<BasicUSaintApplication, ClientError> {
        Ok(BasicUSaintApplication(
            BasicApplication::new(SSU_WEBDYNPRO_BASE_URL, app_name).await?,
        ))
    }

    pub async fn load_placeholder(&mut self) -> Result<(), ClientError> {
        let body = self.body();
        let wd01 = Self::CLIENT_INSPECTOR_WD01.from_body(body)?;
        let wd02 = Self::CLIENT_INSPECTOR_WD02.from_body(body)?;
        let load_ph = Self::LOADING_PLACEHOLDER.from_body(body)?;
        let client_infos = Self::CUSTOM.client_infos(CustomClientInfo {
            client_url: self.client_url(),
            document_domain: "ssu.ac.kr".to_owned(),
            ..CustomClientInfo::default()
        });
        self.send_events(vec![
            wd01.notify(INITIAL_CLIENT_DATA_WD01)?,
            wd02.notify(INITIAL_CLIENT_DATA_WD02)?,
            load_ph.load()?,
            client_infos,
        ])
        .await
    }
}

pub mod course_schedule;
pub mod student_information;
