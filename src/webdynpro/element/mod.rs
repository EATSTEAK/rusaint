use std::{marker, collections::HashMap};

use indexmap::IndexMap;
use regex::Regex;
use scraper::{Selector, Html};
use serde_json::{Map, Value};

use super::{event::{ucf_parameters::UcfParameters, Event, EventBuilder}, error::{ElementError, BodyError}};

pub mod button;
pub mod client_inspector;
pub mod combo_box;
pub mod custom;
pub mod form;
pub mod loading_placeholder;
pub mod tab_strip;
pub mod sap_table;
mod list_box;

pub type EventParameterMap = HashMap<String, (UcfParameters, IndexMap<String, String>)>;

pub struct ElementDef<'a, T>
    where T: Element {
    id: &'a str,
    _marker: marker::PhantomData<T>,
}

impl<'a, T> ElementDef<'a, T>
where T: Element
{
    pub const fn new(id: &'a str) -> ElementDef<'a, T> {
        ElementDef {
            id, _marker: std::marker::PhantomData
        }
    }

    pub fn selector(&self) -> Result<Selector, ElementError> {
        println!("{}", &self.id);
        Selector::parse(format!(r#"[id="{}""#, self.id).as_str())
        .or(Err(ElementError::InvalidId))
    }
}

pub trait Element {
    const CONTROL_ID: &'static str;
    const ELEMENT_NAME: &'static str;
    type ElementLSData;

    // TODO: Do multiple replacements without owning
    fn normalize_lsjson(lsjson: &str) -> String {
        let quote_key = Regex::new(r"([{,])(\w+):").unwrap();
        let quote_to_double = Regex::new(r"([^\\])'([\s\S]*?)'").unwrap();
        let convert_escape_to_rust = Regex::new(r"\\x([a-f0-9]{2})").unwrap();
        let quoted = quote_key.replace_all(lsjson, r#"$1"$2":"#).into_owned();
        let double_quoted = quote_to_double.replace_all(&quoted, r#"$1"$2""#).into_owned();
        let ret = convert_escape_to_rust.replace_all(&double_quoted, r"\u00$1").into_owned();
        ret
    }

    fn lsdata_elem(selector: &Selector, document: &Html) -> Result<Value, ElementError> {
        let select = document.select(&selector).next().ok_or(ElementError::InvalidId)?;
        let raw_data = select.value().attr("lsdata").ok_or(ElementError::InvalidLSData)?;
        let normalized = Self::normalize_lsjson(raw_data);
        return Ok(serde_json::from_str(&normalized)?);
    }

    fn lsevents_elem(selector: &Selector, document: &Html) -> Result<EventParameterMap, BodyError> {
        let select = document.select(&selector).next().ok_or(BodyError::Invalid)?;
        let raw_data = select.value().attr("lsevents").ok_or(BodyError::Invalid)?;
        let normalized = Self::normalize_lsjson(raw_data);
        let json: Map<String, Value> = serde_json::from_str::<Map<String, Value>>(&normalized).or(Err(BodyError::Invalid))?.to_owned();
        Ok(json.into_iter().flat_map(|(key, value)| -> Result<(String, (UcfParameters, IndexMap<String, String>)), BodyError> {
                    let mut parameters = value.as_array().ok_or(BodyError::Invalid)?.to_owned().into_iter();
                    let raw_ucf = parameters.next().ok_or(BodyError::Invalid)?;
                    let ucf: UcfParameters = serde_json::from_value(raw_ucf).or(Err(BodyError::Invalid))?;
                    let mut custom = parameters.next().ok_or(BodyError::Invalid)?.as_object().ok_or(BodyError::Invalid)?.to_owned();
                    let custom_map = custom.iter_mut().map(|(key, value)| { 
                        (key.to_owned(), value.to_string())
                    }).collect::<IndexMap<String, String>>();
                    Ok((key, (ucf, custom_map)))
                }).collect::<EventParameterMap>())
    }

    fn lsdata(&self) -> Option<&Self::ElementLSData>;

    fn lsevents(&self) -> Option<&EventParameterMap>;

    fn event_parameter(&self, event: &str) -> Result<&(UcfParameters, IndexMap<String, String>), ElementError> {
        if let Some(lsevents) = self.lsevents() {
            lsevents.get(event).ok_or(ElementError::NoSuchEvent)
        } else {
            Err(ElementError::NoSuchEvent)
        }
    }

    fn fire_event(&self, event: &str, parameters: IndexMap<String, String>) -> Result<Event, ElementError> {
        let (ucf_params, custom_params) = self.event_parameter(event)?;
        Ok(EventBuilder::default()
            .control(Self::ELEMENT_NAME.to_owned())
            .event(event.to_owned())
            .parameters(parameters)
            .ucf_parameters(ucf_params.to_owned())
            .custom_parameters(custom_params.to_owned())
            .build()
            .unwrap())
    }
}
