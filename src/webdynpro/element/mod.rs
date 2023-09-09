use scraper::Selector;
use serde_json::{Map, Value};

use super::{application::client::body::{WDBody, WDBodyError}, event::ucf_parameters::UcfParameters};

pub mod button;
pub mod client_inspector;
pub mod combo_box;
pub mod custom;
pub mod form;
pub mod loading_placeholder;
pub mod tab_strip;
pub mod sap_table;

trait Element<'a> {
    const CONTROL_ID: &'static str;
}

struct ElementParser<'a, T> where T: Element<'a> {
    component: &'a T,
    id: &'a str,
    body: &'a WDBody
}

impl<'a, T> ElementParser<'a, T>
    where T: Element<'a> {
    fn raw_lsdata(&'a self) -> Result<String, WDBodyError> {
        let selector = Selector::parse(&format!(r#"[id="{}""#, self.id)).unwrap();
        let document = self.body.document();
        let select = document.select(&selector).next().ok_or(WDBodyError::Invalid)?;
        let raw_data = select.value().attr("lsdata").ok_or(WDBodyError::Invalid)?;
        return Ok(raw_data.to_owned());
    }

    fn lsevent(&'a self, event: &str) -> Result<(UcfParameters, Map<String, Value>), WDBodyError> {
        let selector = Selector::parse(&format!(r#"[id="{}""#, self.id)).unwrap();
        let document = self.body.document();
        let select = document.select(&selector).next().ok_or(WDBodyError::Invalid)?;
        let raw_data = select.value().attr("lsevents").ok_or(WDBodyError::Invalid)?;
        let json: Value = serde_json::from_str(raw_data).or(Err(WDBodyError::Invalid))?;
        let event: &Value = &json[event];
        let mut parameters = event.as_array().ok_or(WDBodyError::Invalid)?.to_owned().into_iter();
        let raw_ucf = parameters.next().ok_or(WDBodyError::Invalid)?;
        let ucf: UcfParameters = serde_json::from_value(raw_ucf).or(Err(WDBodyError::Invalid))?;
        let custom = parameters.next().ok_or(WDBodyError::Invalid)?.as_object().ok_or(WDBodyError::Invalid)?.to_owned();
        Ok((ucf, custom.to_owned()))
    }
}

trait Parseable<'a>: Element<'a>
    where Self: Sized {

    fn parser(&'a self, body: &'a WDBody) -> ElementParser<'a, Self>;
}
