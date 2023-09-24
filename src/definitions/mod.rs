use anyhow::Result;
use std::ops::{Deref, DerefMut};

use crate::webdynpro::{
    application::BasicApplication,
    element::{
        client_inspector::ClientInspector,
        custom::{Custom, CustomClientInfo},
        element_ref,
        loading_placeholder::LoadingPlaceholder,
    },
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
