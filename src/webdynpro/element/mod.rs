use std::{marker, collections::HashMap, borrow::Cow};


use regex::Regex;
use scraper::{Selector, ElementRef};
use serde_json::{Map, Value};

use self::{action::{button::Button, link::Link}, layout::{button_row::ButtonRow, Container, FlowLayout, form::Form, grid_layout::{GridLayout, cell::GridLayoutCell}, tab_strip::{TabStrip, item::TabStripItem}, popup_window::PopupWindow, tray::Tray, scrollbar::Scrollbar, scroll_container::ScrollContainer}, system::{client_inspector::ClientInspector, custom::Custom, loading_placeholder::LoadingPlaceholder}, selection::{combo_box::ComboBox, list_box::{ListBoxPopup, ListBoxPopupFiltered, ListBoxPopupJson, ListBoxPopupJsonFiltered, ListBoxMultiple, ListBoxSingle, item::ListBoxItem, action_item::ListBoxActionItem}}, graphic::image::Image, text::{input_field::InputField, label::Label, text_view::TextView, caption::Caption}, complex::sap_table::SapTable};

use super::{event::{ucf_parameters::UcfParameters, Event, EventBuilder}, error::{ElementError, BodyError, WebDynproError}, application::client::body::Body};

pub mod action;
pub mod complex;
pub mod graphic;
pub mod layout;
pub mod selection;
pub mod system;
pub mod text;
pub mod unknown;

pub type EventParameterMap = HashMap<String, (UcfParameters, HashMap<String, String>)>;

macro_rules! define_element_base {
    {$name:ident<$controlid:literal, $element_name:literal> {
        $($sfield:ident : $stype:ty),* $(,)?
    },
    $lsdata:ident {
        $($field:ident: $ftype:ty => $encoded:literal),* $(,)?
    }} => {
        #[derive(custom_debug_derive::Debug)]
        #[allow(unused)]
        pub struct $name<'a> {
            id: std::borrow::Cow<'static, str>,
            #[debug(skip)]
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
            ) -> Result<Self, $crate::webdynpro::error::WebDynproError> {
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
            pub fn dyn_elem(element: scraper::ElementRef<'a>) -> Result<ElementWrapper, WebDynproError> {
                let value = element.value();
                let id = value.id().ok_or(BodyError::NoSuchAttribute("id".to_owned()))?.to_owned();
                #[allow(unreachable_patterns)]
                match element.value().attr("ct") {
                    $( Some(<$type>::CONTROL_ID) => Ok($crate::webdynpro::element::ElementDef::<$type>::new_dynamic(id).from_elem(element)?.wrap()), )*
                    _ => Ok($crate::webdynpro::element::ElementDef::<$crate::webdynpro::element::unknown::Unknown>::new_dynamic(id).from_elem(element)?.wrap())
                }
            }
        }

        $(
            impl<'a> std::convert::TryInto<$type> for ElementWrapper<'a> {
                type Error = $crate::webdynpro::error::BodyError;
    
                fn try_into(self) -> Result<$type, Self::Error> {
                    match self {
                        ElementWrapper::$enum(res) => Ok(res),
                        _ => Err(Self::Error::InvalidElement)
                    }
                }
            }
        )+
        
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

    pub fn selector(&self) -> Result<Selector, WebDynproError> {
        Ok(std::result::Result::or(Selector::parse(format!(r#"[id="{}"]"#, &self.id).as_str()), Err(BodyError::InvalidSelector))?)
    }

    pub fn from_body(self, body: &'a Body) -> Result<T, WebDynproError> {
        T::from_body(self, body)
    }

    pub fn from_elem(self, element: scraper::ElementRef<'a>) -> Result<T, WebDynproError> {
        T::from_elem(self, element)
    }
}

#[macro_export]
macro_rules! define_elements {
    ($($name:ident : $eltype:tt<$lt:lifetime> = $id:literal),+ $(,)?) => {
        $(const $name: $crate::webdynpro::element::ElementDef<$lt, $eltype<$lt>> = $crate::webdynpro::element::ElementDef::new($id);)*
    }
}

pub use define_elements;

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

    fn lsdata_elem(element: scraper::ElementRef) -> Result<Value, WebDynproError> {
        let raw_data = element.value().attr("lsdata").ok_or(ElementError::InvalidLSData(element.value().id().unwrap().to_string()))?;
        let normalized = normalize_lsjson(raw_data);
        return Ok(serde_json::from_str(&normalized).or(Err(ElementError::InvalidLSData(element.value().id().unwrap().to_string())))?);
    }

    fn from_body(elem_def: ElementDef<'a, Self>, body: &'a Body) -> Result<Self, WebDynproError> {
        let selector = &elem_def.selector().or(Err(BodyError::InvalidSelector))?;
        let element = body
            .document()
            .select(selector)
            .next()
            .ok_or(ElementError::InvalidId(elem_def.id().to_owned()))?;
        Self::from_elem(elem_def, element)
    }

    fn from_elem(elem_def: ElementDef<'a, Self>, element: scraper::ElementRef<'a>) -> Result<Self, WebDynproError>;

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

    unsafe fn fire_event_unchecked(event: String, parameters: HashMap<String, String>, ucf_params: UcfParameters, custom_params: HashMap<String, String>) -> Event {
        EventBuilder::default()
        .control(Self::ELEMENT_NAME.to_owned())
        .event(event)
        .parameters(parameters)
        .ucf_parameters(ucf_params)
        .custom_parameters(custom_params)
        .build()
        .unwrap()
    }

    fn lsevents_elem(element: scraper::ElementRef) -> Result<EventParameterMap, WebDynproError> {
        let raw_data = element.value().attr("lsevents").ok_or(BodyError::Invalid)?;
        let normalized = normalize_lsjson(raw_data);
        let json: Map<String, Value> = serde_json::from_str::<Map<String, Value>>(&normalized).or(Err(BodyError::Invalid))?.to_owned();
        Ok(json.into_iter().flat_map(|(key, value)| -> Result<(String, (UcfParameters, HashMap<String, String>)), BodyError> {
                    let mut parameters = value.as_array().ok_or(BodyError::Invalid)?.to_owned().into_iter();
                    let raw_ucf = parameters.next().ok_or(BodyError::Invalid)?;
                    let ucf: UcfParameters = serde_json::from_value(raw_ucf).or(Err(BodyError::Invalid))?;
                    let mut custom = parameters.next().ok_or(BodyError::Invalid)?.as_object().ok_or(BodyError::Invalid)?.to_owned();
                    let custom_map = custom.iter_mut().map(|(key, value)| { 
                        (key.to_owned(), value.to_string())
                    }).collect::<HashMap<String, String>>();
                    Ok((key, (ucf, custom_map)))
                }).collect::<EventParameterMap>())
    }

    fn event_parameter(&self, event: &str) -> Result<&(UcfParameters, HashMap<String, String>), ElementError> {
        if let Some(lsevents) = self.lsevents() {
            lsevents.get(event).ok_or(ElementError::NoSuchEvent { element: self.id().to_string(), event: event.to_string() })
        } else {
            Err(ElementError::NoSuchEvent { element: self.id().to_string(), event: event.to_string() })
        }
    }

    fn fire_event(&self, event: String, parameters: HashMap<String, String>) -> Result<Event, WebDynproError> {
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

    pub fn selector(&self) -> Result<Selector, WebDynproError> {
        Selector::parse(format!(r#"[id="{}"] [id="{}"]"#, self.parent.id, self.id).as_str())
        .or(Err(ElementError::InvalidId(format!("{}, {}", self.parent.id, self.id)))?)
    }
    pub fn from_body(self, body: &'a Body) -> Result<T, WebDynproError> {
        T::from_body(self, body)
    }

    pub fn from_elem(self, element: scraper::ElementRef<'a>) -> Result<T, WebDynproError> {
        T::from_elem(self, element)
    }
}

pub trait SubElement<'a>: Sized {
    const SUBCONTROL_ID: &'static str;
    const ELEMENT_NAME: &'static str;
    type SubElementLSData;

    fn lsdata_elem(element: scraper::ElementRef) -> Result<Value, WebDynproError> {
        let raw_data = element.value().attr("lsdata").ok_or(ElementError::InvalidLSData(element.value().id().unwrap().to_string()))?;
        let normalized = normalize_lsjson(raw_data);
        return Ok(serde_json::from_str(&normalized).or(Err(ElementError::InvalidLSData(element.value().id().unwrap().to_string())))?);
    }

    fn from_body<Parent: Element<'a>>(
        elem_def: SubElementDef<'a, Parent, Self>,
        body: &'a Body,
    ) -> Result<Self, WebDynproError> {
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
    ) -> Result<Self, WebDynproError>;

    fn lsdata(&self) -> Option<&Self::SubElementLSData>;

    fn id(&self) -> &str;

    fn element_ref(&self) -> &ElementRef<'a>;
}
