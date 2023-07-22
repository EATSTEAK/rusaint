use std::ops::{Deref, DerefMut};
use super::SapSsrClient;

pub(crate) struct WDBodyUpdate<'a>(tl::VDom<'a>, String);

impl<'a> Deref for WDBodyUpdate<'a> {
    type Target = tl::VDom<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for WDBodyUpdate<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub(crate) struct WDBody<'a>(tl::VDom<'a>, String);

impl<'a> Deref for WDBody<'a> {
    type Target = tl::VDom<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for WDBody<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> WDBody<'a> {

    pub fn new(body: String) -> Result<WDBody<'static>, tl::ParseError> {
        Ok(WDBody(tl::parse(&body, tl::ParserOptions::default())?, body))
    }

    pub fn parse_sap_ssr_client(&self) -> SapSsrClient {
        SapSsrClient{ action: todo!(), charset: todo!(), wd_secure_id: todo!(), app_name: todo!(), use_beacon: todo!() }
    }

    pub fn apply(&mut self, update: WDBodyUpdate) {
        todo!("Implement WebDynpro dom update logic")
    }
}