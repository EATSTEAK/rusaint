use std::{marker, collections::HashMap, borrow::Cow};
use anyhow::Result;

use indexmap::IndexMap;
use regex::Regex;
use scraper::Selector;
use serde_json::{Map, Value};


use self::{button::Button, client_inspector::ClientInspector, combo_box::ComboBox, custom::Custom, form::Form, loading_placeholder::LoadingPlaceholder, tab_strip::{TabStrip, item::TabStripItem}, sap_table::SapTable, unknown::Unknown};

use super::{event::{ucf_parameters::UcfParameters, Event, EventBuilder}, error::{ElementError, BodyError}, application::client::body::Body};

pub mod button;
pub mod client_inspector;
pub mod combo_box;
pub mod custom;
pub mod form;
pub mod loading_placeholder;
pub mod tab_strip;
pub mod sap_table;
pub mod list_box;
pub mod unknown;

pub type EventParameterMap = HashMap<String, (UcfParameters, IndexMap<String, String>)>;

#[derive(Debug)]
pub enum Elements {
    Button(ElementDef<Button>),
    ClientInspector(ElementDef<ClientInspector>),
    ComboBox(ElementDef<ComboBox>),
    Custom(ElementDef<Custom>),
    Form(ElementDef<Form>),
    LoadingPlaceholder(ElementDef<LoadingPlaceholder>),
    TabStrip(ElementDef<TabStrip>),
    TabStripItem(ElementDef<TabStripItem>),
    SapTable(ElementDef<SapTable>),
    Unknown(ElementDef<Unknown>),
}

#[derive(Debug)]
pub struct ElementDef<T>
    where T: Element {
    id: Cow<'static, str>,
    _marker: marker::PhantomData<T>,
}

impl<T: Element> Clone for ElementDef<T> {
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), _marker: self._marker.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

impl<T> ElementDef<T>
where T: Element
{

    pub const fn new(id: &'static str) -> ElementDef<T> {
        ElementDef {
            id: Cow::Borrowed(id),
            _marker: std::marker::PhantomData
        }
    }

    pub fn new_dynamic(id: String) -> ElementDef<T> {
        ElementDef {
            id: id.into(), _marker: std::marker::PhantomData
        }
    }

    pub fn selector(&self) -> Result<Selector> {
        Ok(std::result::Result::or(Selector::parse(format!(r#"[id="{}"]"#, &self.id).as_str()), Err(BodyError::InvalidSelector))?)
    }

    pub fn from_body(self, body: &'_ Body) -> Result<T> {
        T::from_body(self, body)
    }

    pub fn from_elem(self, element: scraper::ElementRef) -> Result<T> {
        T::from_elem(self, element)
    }
}

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

macro_rules! match_elem {
    ($id: expr, $element: expr, $( $type: ty ),+ $(,)?) => {
        match $element.value().attr("ct") {
            $( Some(<$type>::CONTROL_ID) => Ok($crate::webdynpro::element::ElementDef::<$type>::new_dynamic($id).wrap()), )*
            _ => Ok($crate::webdynpro::element::ElementDef::<$crate::webdynpro::element::unknown::Unknown>::new_dynamic($id).wrap())
        }
    };
}

fn dyn_elem(element: scraper::ElementRef) -> Result<Elements> {
        let value = element.value();
        let id = value.id().ok_or(ElementError::InvalidId)?.to_owned();
        match_elem!(id, element, 
            Button,
            ClientInspector,
            ComboBox,
            Form,
            LoadingPlaceholder,
            TabStrip, 
            TabStripItem,
            SapTable
        )
}

pub trait Element: Sized {
    const CONTROL_ID: &'static str;
    const ELEMENT_NAME: &'static str;
    type ElementLSData;

    fn from_body(elem_def: ElementDef<Self>, body: &'_ Body) -> Result<Self> {
        let selector = &elem_def.selector().or(Err(BodyError::InvalidSelector))?;
        let element = body
            .document()
            .select(selector)
            .next()
            .ok_or(ElementError::InvalidId)?;
        Self::from_elem(elem_def, element)
    }

    fn from_elem(elem_def: ElementDef<Self>, element: scraper::ElementRef) -> Result<Self>;

    fn lsdata_elem(element: scraper::ElementRef) -> Result<Value> {
        let raw_data = element.value().attr("lsdata").ok_or(ElementError::InvalidLSData)?;
        let normalized = normalize_lsjson(raw_data);
        return Ok(serde_json::from_str(&normalized)?);
    }

    fn lsevents_elem(element: scraper::ElementRef) -> Result<EventParameterMap> {
        let raw_data = element.value().attr("lsevents").ok_or(BodyError::Invalid)?;
        let normalized = normalize_lsjson(raw_data);
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

    unsafe fn fire_event_unchecked(event: &str, parameters: IndexMap<String, String>, ucf_params: UcfParameters, custom_params: IndexMap<String, String>) -> Event {
        EventBuilder::default()
        .control(Self::ELEMENT_NAME.to_owned())
        .event(event.to_owned())
        .parameters(parameters)
        .ucf_parameters(ucf_params)
        .custom_parameters(custom_params)
        .build()
        .unwrap()
    }

    fn fire_event(&self, event: &str, parameters: IndexMap<String, String>) -> Result<Event> {
        let (ucf_params, custom_params) = self.event_parameter(event)?;
        Ok(unsafe { Self::fire_event_unchecked(event, parameters, ucf_params.to_owned(), custom_params.to_owned()) })
    }
}


#[derive(Debug)]
pub struct SubElementDef<Parent, T>
    where Parent: Element, T: SubElement {
        id: Cow<'static, str>,
        parent: ElementDef<Parent>,
        _marker: std::marker::PhantomData<T>
}

impl<Parent: Element, T: SubElement> Clone for SubElementDef<Parent, T> {
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), parent: self.parent.clone(), _marker: self._marker.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

impl<Parent, T> SubElementDef<Parent, T>
where Parent: Element, T: SubElement
{

    pub const fn new(parent: ElementDef<Parent>, id: &'static str) -> SubElementDef<Parent, T> {
        SubElementDef {
            id: Cow::Borrowed(id),
            parent,
            _marker: std::marker::PhantomData
        }
    }

    pub fn new_dynamic(parent: ElementDef<Parent>, id: String) -> SubElementDef<Parent, T> {
        SubElementDef {
            id: id.into(), parent, _marker: std::marker::PhantomData
        }
    }

    pub fn selector(&self) -> Result<Selector> {
        Selector::parse(format!(r#"[id="{}"] [id="{}"]"#, self.parent.id, self.id).as_str())
        .or(Err(ElementError::InvalidId)?)
    }
    pub fn from_body(self, body: &'_ Body) -> Result<T> {
        T::from_body(self, body)
    }

    pub fn from_elem(self, element: scraper::ElementRef) -> Result<T> {
        T::from_elem(self, element)
    }
}

pub trait SubElement: Sized {
    const SUBCONTROL_ID: &'static str;
    const ELEMENT_NAME: &'static str;
    type SubElementLSData;

    fn lsdata_elem(element: scraper::ElementRef) -> Result<Value> {
        let raw_data = element.value().attr("lsdata").ok_or(ElementError::InvalidLSData)?;
        let normalized = normalize_lsjson(raw_data);
        return Ok(serde_json::from_str(&normalized)?);
    }

    fn lsdata(&self) -> Option<&Self::SubElementLSData>;

    fn from_body<Parent: Element>(
        elem_def: SubElementDef<Parent, Self>,
        body: &'_ Body,
    ) -> Result<Self> {
        let selector = &elem_def.selector().or(Err(BodyError::InvalidSelector))?;
        let element = body
            .document()
            .select(selector)
            .next()
            .ok_or(ElementError::InvalidId)?;
        Self::from_elem(elem_def, element)
    }

    fn from_elem<Parent: Element>(
        elem_def: SubElementDef<Parent, Self>,
        element: scraper::ElementRef
    ) -> Result<Self>;
}
