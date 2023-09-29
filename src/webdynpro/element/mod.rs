use std::{marker, collections::HashMap, borrow::Cow};
use anyhow::Result;

use indexmap::IndexMap;
use regex::Regex;
use scraper::{Selector, ElementRef};
use serde_json::{Map, Value};


use crate::webdynpro::element::text_view::TextView;

use self::{button::Button, client_inspector::ClientInspector, combo_box::ComboBox, custom::Custom, form::Form, loading_placeholder::LoadingPlaceholder, tab_strip::{TabStrip, item::TabStripItem}, sap_table::SapTable, caption::Caption, link::Link, button_row::ButtonRow, list_box::{ListBoxMultiple, ListBoxPopup, ListBoxPopupFiltered, ListBoxPopupJson, ListBoxPopupJsonFiltered, ListBoxSingle, item::ListBoxItem, action_item::ListBoxActionItem}};

use super::{event::{ucf_parameters::UcfParameters, Event, EventBuilder}, error::{ElementError, BodyError}, application::client::body::Body};

pub mod button;
pub mod button_row;
pub mod caption;
pub mod client_inspector;
pub mod combo_box;
pub mod custom;
pub mod form;
pub mod link;
pub mod loading_placeholder;
pub mod tab_strip;
pub mod sap_table;
pub mod list_box;
pub mod text_view;
pub mod unknown;

pub type EventParameterMap = HashMap<String, (UcfParameters, IndexMap<String, String>)>;

macro_rules! register_elements {
    [$( $enum:ident : $type: ty ),+ $(,)?] => {
        pub enum Elements<'a> {
            $( $enum($type), )*
            Unknown($crate::webdynpro::element::unknown::Unknown<'a>)
        }

        impl<'a> std::fmt::Debug for Elements<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $( Elements::$enum(elem) => {
                        f.debug_struct(stringify!($enum))
                            .field("id", &elem.id().to_string())
                            .finish()
                    },)+
                    Elements::Unknown(elem) => {
                        f.debug_struct("Unknown")
                            .field("ct", &elem.ct().to_owned())
                            .field("id", &elem.id().to_string())
                            .finish()
                    },
                }
            }
        }

        impl<'a> Elements<'a> {
            pub fn dyn_elem(element: scraper::ElementRef<'a>) -> Result<Elements> {
                let value = element.value();
                let id = value.id().ok_or(ElementError::NoSuchAttribute("id".to_owned()))?.to_owned();
                #[allow(unreachable_patterns)]
                match element.value().attr("ct") {
                    $( Some(<$type>::CONTROL_ID) => Ok($crate::webdynpro::element::ElementDef::<$type>::new_dynamic(id).from_elem(element)?.wrap()), )*
                    _ => Ok($crate::webdynpro::element::ElementDef::<$crate::webdynpro::element::unknown::Unknown>::new_dynamic(id).from_elem(element)?.wrap())
                }
            }
        }
        
    };
}

register_elements![
    Button: Button<'a>,
    ButtonRow: ButtonRow<'a>,
    ClientInspector: ClientInspector<'a>,
    ComboBox: ComboBox<'a>,
    Custom: Custom,
    Form: Form<'a>,
    Link: Link<'a>,
    ListBoxPopup: ListBoxPopup<'a>,
    ListBoxPopupFiltered: ListBoxPopupFiltered<'a>,
    ListBoxPopupJson: ListBoxPopupJson<'a>,
    ListBoxPopupJsonFiltered: ListBoxPopupJsonFiltered<'a>,
    ListBoxMultiple: ListBoxMultiple<'a>,
    ListBoxSingle: ListBoxSingle<'a>,
    ListBoxItem: ListBoxItem<'a>,
    ListBoxActionItem: ListBoxActionItem<'a>,
    LoadingPlaceholder: LoadingPlaceholder<'a>,
    TabStrip: TabStrip<'a>,
    TabStripItem: TabStripItem<'a>,
    SapTable: SapTable<'a>,
    TextView: TextView<'a>,
    Caption: Caption<'a>,
];

#[derive(Debug)]
pub struct ElementDef<'a, T>
    where T: Element<'a> {
    id: Cow<'static, str>,
    _marker: marker::PhantomData<&'a T>,
}

impl<'a, T: Element<'a>> Clone for ElementDef<'a, T> {
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), _marker: self._marker.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

impl<'a, T> ElementDef<'a, T>
where T: Element<'a>
{

    pub const fn new(id: &'static str) -> ElementDef<'a, T> {
        ElementDef {
            id: Cow::Borrowed(id),
            _marker: std::marker::PhantomData
        }
    }

    pub fn new_dynamic(id: String) -> ElementDef<'a, T> {
        ElementDef {
            id: id.into(), _marker: std::marker::PhantomData
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn selector(&self) -> Result<Selector> {
        Ok(std::result::Result::or(Selector::parse(format!(r#"[id="{}"]"#, &self.id).as_str()), Err(BodyError::InvalidSelector))?)
    }

    pub fn from_body(self, body: &'a Body) -> Result<T> {
        T::from_body(self, body)
    }

    pub fn from_elem(self, element: scraper::ElementRef<'a>) -> Result<T> {
        T::from_elem(self, element)
    }
}

#[macro_export]
macro_rules! element_ref {
    ($($name:ident : $eltype:tt<$lt:lifetime> = $id:literal),+ $(,)?) => {
        $(const $name: $crate::webdynpro::element::ElementDef<$lt, $eltype<$lt>> = $crate::webdynpro::element::ElementDef::new($id);)*
    }
}

pub use element_ref;

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

pub trait Element<'a>: Sized {
    const CONTROL_ID: &'static str;
    const ELEMENT_NAME: &'static str;
    type ElementLSData;

    unsafe fn fire_event_unchecked(event: String, parameters: IndexMap<String, String>, ucf_params: UcfParameters, custom_params: IndexMap<String, String>) -> Event {
        EventBuilder::default()
        .control(Self::ELEMENT_NAME.to_owned())
        .event(event)
        .parameters(parameters)
        .ucf_parameters(ucf_params)
        .custom_parameters(custom_params)
        .build()
        .unwrap()
    }

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

    fn from_body(elem_def: ElementDef<'a, Self>, body: &'a Body) -> Result<Self> {
        let selector = &elem_def.selector().or(Err(BodyError::InvalidSelector))?;
        let element = body
            .document()
            .select(selector)
            .next()
            .ok_or(ElementError::InvalidId(elem_def.id().to_owned()))?;
        Self::from_elem(elem_def, element)
    }

    fn from_elem(elem_def: ElementDef<'a, Self>, element: scraper::ElementRef<'a>) -> Result<Self>;

    fn event_parameter(&self, event: &str) -> Result<&(UcfParameters, IndexMap<String, String>), ElementError> {
        if let Some(lsevents) = self.lsevents() {
            lsevents.get(event).ok_or(ElementError::NoSuchEvent)
        } else {
            Err(ElementError::NoSuchEvent)
        }
    }

    fn fire_event(&self, event: String, parameters: IndexMap<String, String>) -> Result<Event> {
        let (ucf_params, custom_params) = self.event_parameter(&event)?;
        Ok(unsafe { Self::fire_event_unchecked(event, parameters, ucf_params.to_owned(), custom_params.to_owned()) })
    }

    fn lsdata(&self) -> Option<&Self::ElementLSData>;

    fn lsevents(&self) -> Option<&EventParameterMap>;

    fn id(&self) -> &str;

    fn element_ref(&self) -> &ElementRef<'a>;

    fn wrap(self) -> Elements<'a>;

    
}


#[derive(Debug)]
pub struct SubElementDef<'a, Parent, T>
    where Parent: Element<'a>, T: SubElement<'a> {
        id: Cow<'static, str>,
        parent: ElementDef<'a, Parent>,
        _marker: std::marker::PhantomData<&'a T>
}

impl<'a, Parent: Element<'a>, T: SubElement<'a>> Clone for SubElementDef<'a, Parent, T> {
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), parent: self.parent.clone(), _marker: self._marker.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

impl<'a, Parent, T> SubElementDef<'a, Parent, T>
where Parent: Element<'a>, T: SubElement<'a>
{

    pub const fn new(parent: ElementDef<'a, Parent>, id: &'static str) -> SubElementDef<'a, Parent, T> {
        SubElementDef {
            id: Cow::Borrowed(id),
            parent,
            _marker: std::marker::PhantomData
        }
    }

    pub fn new_dynamic(parent: ElementDef<'a, Parent>, id: String) -> SubElementDef<'a, Parent, T> {
        SubElementDef {
            id: id.into(), parent, _marker: std::marker::PhantomData
        }
    }

    pub fn selector(&self) -> Result<Selector> {
        Selector::parse(format!(r#"[id="{}"] [id="{}"]"#, self.parent.id, self.id).as_str())
        .or(Err(ElementError::InvalidId(format!("{}, {}", self.parent.id, self.id)))?)
    }
    pub fn from_body(self, body: &'a Body) -> Result<T> {
        T::from_body(self, body)
    }

    pub fn from_elem(self, element: scraper::ElementRef<'a>) -> Result<T> {
        T::from_elem(self, element)
    }
}

pub trait SubElement<'a>: Sized {
    const SUBCONTROL_ID: &'static str;
    const ELEMENT_NAME: &'static str;
    type SubElementLSData;

    fn lsdata_elem(element: scraper::ElementRef) -> Result<Value> {
        let raw_data = element.value().attr("lsdata").ok_or(ElementError::InvalidLSData)?;
        let normalized = normalize_lsjson(raw_data);
        return Ok(serde_json::from_str(&normalized)?);
    }

    fn from_body<Parent: Element<'a>>(
        elem_def: SubElementDef<'a, Parent, Self>,
        body: &'a Body,
    ) -> Result<Self> {
        let selector = &elem_def.selector().or(Err(BodyError::InvalidSelector))?;
        let element = body
            .document()
            .select(selector)
            .next()
            .ok_or(ElementError::InvalidId((&elem_def.id).clone().into_owned()))?;
        Self::from_elem(elem_def, element)
    }

    fn from_elem<Parent: Element<'a>>(
        elem_def: SubElementDef<'a, Parent, Self>,
        element: scraper::ElementRef<'a>
    ) -> Result<Self>;

    fn lsdata(&self) -> Option<&Self::SubElementLSData>;

    fn id(&self) -> &str;

    fn element_ref(&self) -> &ElementRef<'a>;
}
