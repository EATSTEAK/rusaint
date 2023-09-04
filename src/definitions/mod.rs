use std::ops::{Deref, DerefMut};

use crate::webdynpro::{application::{BasicWDApplication, client::WDClientError}, component::{client_inspector::ClientInspector, loading_placeholder::LoadingPlaceholder, form::Form, custom::{Custom, CustomClientInfo}}};

const SSU_WEBDYNPRO_BASE_URL: &str = "https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/";
const INITIAL_CLIENT_DATA_WD01: &str = "ClientWidth:1920px;ClientHeight:1000px;ScreenWidth:1920px;ScreenHeight:1080px;ScreenOrientation:landscape;ThemedTableRowHeight:33px;ThemedFormLayoutRowHeight:32px;ThemedSvgLibUrls:{\"SAPGUI-icons\":\"https://ecc.ssu.ac.kr:8443/sap/public/bc/ur/nw5/themes/~cache-20210223121230/Base/baseLib/sap_fiori_3/svg/libs/SAPGUI-icons.svg\",\"SAPWeb-icons\":\"https://ecc.ssu.ac.kr:8443/sap/public/bc/ur/nw5/themes/~cache-20210223121230/Base/baseLib/sap_fiori_3/svg/libs/SAPWeb-icons.svg\"};ThemeTags:Fiori_3,Touch;ThemeID:sap_fiori_3;SapThemeID:sap_fiori_3;DeviceType:DESKTOP";
const INITIAL_CLIENT_DATA_WD02: &str = "ThemedTableRowHeight:25px";
pub struct BasicUSaintApplication(BasicWDApplication);

impl Deref for BasicUSaintApplication {
    type Target = BasicWDApplication;
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
    pub const CLIENT_INSPECTOR_WD01: ClientInspector<'_> = ClientInspector::new("WD01");
    pub const CUSTOM: Custom<'_> = Custom::new("WD01");
    pub const CLIENT_INSPECTOR_WD02: ClientInspector<'_> = ClientInspector::new("WD02");
    pub const LOADING_PLACEHOLDER: LoadingPlaceholder<'_> = LoadingPlaceholder::new("_loadingPlaceholder_");
    pub const SSR_FORM: Form<'_> = Form::new("sap.client.SsrClient.form");

    pub async fn new(app_name: &str) -> Result<BasicUSaintApplication, WDClientError> {
        Ok(BasicUSaintApplication(BasicWDApplication::new(SSU_WEBDYNPRO_BASE_URL, app_name).await?))
    }

    pub async fn load_placeholder(&mut self) -> Result<(), WDClientError> {
        let notify_wd01 = Self::CLIENT_INSPECTOR_WD01.notify(
            INITIAL_CLIENT_DATA_WD01
        );
        let notify_wd02 = Self::CLIENT_INSPECTOR_WD02.notify(
            INITIAL_CLIENT_DATA_WD02
        );
        let load = Self::LOADING_PLACEHOLDER.load();
        let client_infos = Self::CUSTOM.client_infos(CustomClientInfo {
            client_url: self.client_url(),
            document_domain: "ssu.ac.kr".to_owned(),
            ..CustomClientInfo::default()
        });
        let form_request = Self::SSR_FORM.request(false, "", "", false, false);
        self.event_queue().extend([notify_wd01, notify_wd02, load, form_request, client_infos]);
        self.send_event().await
    }
}

pub mod course_schedule;
pub mod student_information;