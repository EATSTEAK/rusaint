use std::{marker, collections::HashMap, borrow::Cow};
use anyhow::Result;

use indexmap::IndexMap;
use regex::Regex;
use scraper::{Selector, ElementRef};
use serde_json::{Map, Value};


use crate::webdynpro::element::text_view::TextView;

use self::{button::Button, client_inspector::ClientInspector, combo_box::ComboBox, custom::Custom, form::Form, loading_placeholder::LoadingPlaceholder, tab_strip::{TabStrip, item::TabStripItem}, sap_table::SapTable, caption::Caption, link::Link, button_row::ButtonRow, list_box::{ListBoxMultiple, ListBoxPopup, ListBoxPopupFiltered, ListBoxPopupJson, ListBoxPopupJsonFiltered, ListBoxSingle, item::ListBoxItem, action_item::ListBoxActionItem}, layout::{FlowLayout, scroll_container::ScrollContainer, grid_layout::{GridLayout, cell::GridLayoutCell}, Container}, image::Image, input_field::InputField, label::Label, tray::Tray, scrollbar::Scrollbar, popup_window::PopupWindow};

use super::{event::{ucf_parameters::UcfParameters, Event, EventBuilder}, error::{ElementError, BodyError}, application::client::body::Body};

pub mod button;
pub mod button_row;
pub mod caption;
pub mod client_inspector;
pub mod combo_box;
pub mod custom;
pub mod form;
pub mod image;
pub mod input_field;
pub mod label;
pub mod layout;
pub mod link;
pub mod loading_placeholder;
pub mod popup_window;
pub mod tab_strip;
pub mod tray;
pub mod sap_table;
pub mod scrollbar;
pub mod list_box;
pub mod text_view;
pub mod unknown;

pub type EventParameterMap = HashMap<String, (UcfParameters, IndexMap<String, String>)>;

macro_rules! define_element_base {
    {$name:ident<$controlid:literal, $element_name:literal> {
        $($sfield:ident : $stype:ty),* $(,)?
    },
    $lsdata:ident {
        $($field:ident: $ftype:ty => $encoded:literal),* $(,)?
    }} => {
        #[derive(Debug)]
        #[allow(unused)]
        pub struct $name<'a> {
            id: std::borrow::Cow<'static, str>,
            element_ref: scraper::ElementRef<'a>,
            lsdata: std::cell::OnceCell<Option<$lsdata>>,
            $($sfield: $stype, )*
        }
        impl<'a> $crate::webdynpro::element::Element<'a> for $name<'a> {
            const CONTROL_ID: &'static str = $controlid;

            const ELEMENT_NAME: &'static str = $element_name;

            type ElementLSData = $lsdata;

            fn lsdata(&self) -> Option<&Self::ElementLSData> {
                self.lsdata
                    .get_or_init(|| {
                        let lsdata_obj = Self::lsdata_elem(self.element_ref).ok()?;
                        serde_json::from_value::<Self::ElementLSData>(lsdata_obj).ok()
                    })
                    .as_ref()
            }

            fn from_elem(
                elem_def: $crate::webdynpro::element::ElementDef<'a, Self>,
                element: scraper::ElementRef<'a>,
            ) -> anyhow::Result<Self> {
                Ok(Self::new(elem_def.id.to_owned(), element))
            }

            fn id(&self) -> &str {
                &self.id
            }

            fn element_ref(&self) -> &scraper::ElementRef<'a> {
                &self.element_ref
            }

            fn wrap(self) -> $crate::webdynpro::element::ElementWrapper<'a> {
                $crate::webdynpro::element::ElementWrapper::$name(self)
            }

            fn children(&self) -> Vec<$crate::webdynpro::element::ElementWrapper<'a>> {
                Self::children_elem(self.element_ref().clone())
            }
        }

        #[derive(getset::Getters, serde::Deserialize, Debug, Default)]
        #[allow(unused)]
        #[get = "pub"]
        pub struct $lsdata {
            $(
                #[serde(rename = $encoded)]
                $field: Option<$ftype>,
            )*
        }
    };
}

macro_rules! define_element_interactable {
    {$name:ident<$controlid:literal, $element_name:literal> {
        $($sfield:ident : $stype:ty),* $(,)?
    },
    $lsdata:ident {
        $($field:ident: $ftype:ty => $encoded:literal),* $(,)?
    }} => {
        $crate::webdynpro::element::define_element_base!{
            $name<$controlid, $element_name> {
                lsevents: std::cell::OnceCell<Option<$crate::webdynpro::element::EventParameterMap>>,
                $($sfield : $stype, )*
            },
            $lsdata {
                $($field: $ftype => $encoded, )*
            }
        }

        impl<'a> $crate::webdynpro::element::Interactable<'a> for $name<'a> {
            fn lsevents(&self) -> Option<&$crate::webdynpro::element::EventParameterMap> {
                self.lsevents
                    .get_or_init(|| Self::lsevents_elem(self.element_ref).ok())
                    .as_ref()
            }
        }
    }
}

pub(crate) use define_element_base;
pub(crate) use define_element_interactable;

macro_rules! register_elements {
    [$( $enum:ident : $type: ty ),+ $(,)?] => {
        pub enum ElementWrapper<'a> {
            $( $enum($type), )*
            Unknown($crate::webdynpro::element::unknown::Unknown<'a>)
        }

        impl<'a> std::fmt::Debug for ElementWrapper<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $( ElementWrapper::$enum(elem) => {
                        f.debug_struct(stringify!($enum))
                            .field("id", &elem.id().to_string())
                            .finish()
                    },)+
                    ElementWrapper::Unknown(elem) => {
                        f.debug_struct("Unknown")
                            .field("ct", &elem.ct().to_owned())
                            .field("id", &elem.id().to_string())
                            .finish()
                    },
                }
            }
        }

        impl<'a> ElementWrapper<'a> {
            pub fn dyn_elem(element: scraper::ElementRef<'a>) -> Result<ElementWrapper> {
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
    Container: Container<'a>,
    Custom: Custom,
    FlowLayout: FlowLayout<'a>,
    Form: Form<'a>,
    GridLayout: GridLayout<'a>,
    GridLayoutCell: GridLayoutCell<'a>,
    Image: Image<'a>,
    InputField: InputField<'a>,
    Label: Label<'a>,
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
    PopupWindow: PopupWindow<'a>,
    TabStrip: TabStrip<'a>,
    TabStripItem: TabStripItem<'a>,
    Tray: Tray<'a>,
    SapTable: SapTable<'a>,
    Scrollbar: Scrollbar<'a>,
    ScrollContainer: ScrollContainer<'a>,
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

    fn lsdata_elem(element: scraper::ElementRef) -> Result<Value> {
        let raw_data = element.value().attr("lsdata").ok_or(ElementError::InvalidLSData)?;
        let normalized = normalize_lsjson(raw_data);
        return Ok(serde_json::from_str(&normalized)?);
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

    fn children_elem(root: scraper::ElementRef<'a>) -> Vec<ElementWrapper<'a>> {
        let mut next_refs = vec![root];
        let mut cts: Vec<ElementRef<'_>> = vec![];
        while let Some(elem) = next_refs.pop() {
            for child in elem.children() {
                if let scraper::Node::Element(child_elem) = child.value() {
                    let child_elem_ref = scraper::ElementRef::wrap(child).unwrap();
                    if child_elem.attr("ct").is_some() {
                        cts.push(child_elem_ref);
                    } else {
                        next_refs.push(child_elem_ref);
                    }

                }
            }
        }
        cts.into_iter().rev().filter_map(|eref| ElementWrapper::dyn_elem(eref).ok()).collect()
    }

    fn children(&self) -> Vec<ElementWrapper<'a>>;

    fn lsdata(&self) -> Option<&Self::ElementLSData>;

    fn id(&self) -> &str;

    fn element_ref(&self) -> &ElementRef<'a>;

    fn wrap(self) -> ElementWrapper<'a>;
}

pub trait Interactable<'a>: Element<'a> {

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

    fn lsevents(&self) -> Option<&EventParameterMap>;
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
