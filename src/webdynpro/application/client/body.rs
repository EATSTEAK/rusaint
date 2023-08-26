use std::{ops::{Deref, DerefMut}, collections::HashMap};

use super::SapSsrClient;

pub struct WDBodyUpdate<'a>(tl::VDom<'a>);

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

pub struct WDBody<'a>(tl::VDom<'a>);

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

    pub fn new(body: &'a str) -> Result<WDBody<'a>, tl::ParseError> {
        Ok(WDBody(tl::parse(body, tl::ParserOptions::default())?))
    }

    pub fn parse_sap_ssr_client(&self) -> Option<SapSsrClient>  {
        let parser = &self.parser();
        let client_form = &self.get_element_by_id("sap.client.SsrClient.form")
            .expect("Failed to find element")
            .get(parser)
            .unwrap()
            .as_tag()
            .expect("Not a valid tag");
        let mut data = HashMap::<String, String>::new();
        data.insert("action".to_owned(), 
        client_form.attributes().get("action")
            .flatten()
            .expect("Attribute not found or malformed")
            .as_utf8_str().to_string()
        );
        let children_iter = client_form.children().all(parser).iter();
        children_iter.for_each(|item| {
            let item_tag = item.as_tag().expect("Not a valid tag");
            let attributes = item_tag.attributes();
            let id = attributes.id().expect("id Attribute not found or malformed").as_utf8_str().to_string();
            let value = attributes.get("value").flatten().expect("value Attribute not found or malformed").as_utf8_str().to_string();
            data.insert(id, value);
        });
        Some(SapSsrClient {
            action: data.get("action")?.to_owned(),
            charset: data.get("sap-charset")?.to_owned(),
            wd_secure_id: data.get("sap-wd-secure-id")?.to_owned(),
            app_name: data.get("fesrAppName")?.to_owned(),
            use_beacon: (data.get("fesrUseBeacon")?.to_owned().as_str() == "true")
        })
    }

    pub fn apply(&mut self, update: WDBodyUpdate) {
        todo!("Implement WebDynpro dom update logic")
    }
}