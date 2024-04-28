use std::{collections::HashMap, hash::{DefaultHasher, Hash, Hasher}};


use regex::Regex;
use scraper::ElementRef;
use serde_json::{Map, Value};

use self::{action::{Button, Link}, complex::SapTable, definition::{sub::SubElementDef, ElementDef}, graphic::Image, layout::{grid_layout::cell::GridLayoutCell, tab_strip::item::TabStripItem, ButtonRow, Container, FlowLayout, Form, GridLayout, PopupWindow, ScrollContainer, Scrollbar, TabStrip, Tray}, selection::{list_box::{item::{ListBoxActionItem, ListBoxItem}, ListBoxMultiple, ListBoxPopup, ListBoxPopupFiltered, ListBoxPopupJson, ListBoxPopupJsonFiltered, ListBoxSingle}, ComboBox}, system::{ClientInspector, Custom, LoadingPlaceholder}, text::{Caption, InputField, Label, TextView}};

use super::{event::{ucf_parameters::UcfParameters, Event, EventBuilder}, error::{ElementError, BodyError, WebDynproError}, client::body::Body};

/// 엘리먼트의 정의를 다루는 모듈
pub mod definition;

/// 버튼 등 기본적인 액션에 이용되는 엘리먼트
pub mod action;
/// 복잡한 데이터를 표현하는 엘리먼트
pub mod complex;
/// 이미지 등 그래픽 데이터를 처리하는 엘리먼트
pub mod graphic;
/// 레이아웃을 정의하는 엘리먼트
pub mod layout;
/// 사용자 선택을 수행하는 엘리먼트
pub mod selection;
/// 시스템에서 사용하는 엘리먼트
pub mod system;
/// 텍스트를 표현하는 엘리먼트
pub mod text;
/// rusaint에서 구현되지 않는 엘리먼트
pub mod unknown;

/// 엘리먼트에서 사용되는 프로퍼티
pub mod property;

/// 엘리먼트에서 발생시킬 수 있는 이벤트의 기본 파라메터
pub type EventParameterMap = HashMap<String, (UcfParameters, HashMap<String, String>)>;

macro_rules! define_lsdata {
    {   $(#[$lsdata_outer:meta])*
        $lsdata:ident {
            $(
                $(#[$lsdata_inner:meta])*
                $field:ident: $ftype:ty => $encoded:literal
            ),* $(,)?
        }
    } => {
        $(#[$lsdata_outer])*
        #[derive(Clone, serde::Deserialize, Debug, Default)]
        #[allow(unused)]
        pub struct $lsdata {
            $(
                $(#[$lsdata_inner])*
                #[serde(rename = $encoded)]
                $field: Option<$ftype>,
            )*
        }

        #[allow(missing_docs)]
        impl $lsdata {
            $(
                pub fn $field(&self) -> Option<&$ftype> {
                    (&self.$field).as_ref()
                }
            )*
        }
    }
}

macro_rules! define_element_base {
    {   $(#[$outer:meta])*
        $name:ident<$controlid:literal, $element_name:literal> {
            $($sfield:ident : $stype:ty),* $(,)?
        },
        $(#[$lsdata_outer:meta])*
        $lsdata:ident {
            $(
                $(#[$lsdata_inner:meta])*
                $field:ident: $ftype:ty => $encoded:literal
            ),* $(,)?
        }
    } => {
        $(#[$outer])*
        #[derive(custom_debug_derive::Debug)]
        #[allow(unused)]
        pub struct $name<'a> {
            id: std::borrow::Cow<'static, str>,
            #[debug(skip)]
            element_ref: scraper::ElementRef<'a>,
            lsdata: std::cell::OnceCell<$lsdata>,
            $($sfield: $stype, )*
        }

        impl<'a> $crate::webdynpro::element::Element<'a> for $name<'a> {
            const CONTROL_ID: &'static str = $controlid;

            const ELEMENT_NAME: &'static str = $element_name;

            type ElementLSData = $lsdata;

            fn lsdata(&self) -> &Self::ElementLSData {
                self.lsdata
                    .get_or_init(|| {
                        let Ok(lsdata_obj) = Self::lsdata_elem(self.element_ref) else {
                            return $lsdata::default();
                        };
                        serde_json::from_value::<Self::ElementLSData>(lsdata_obj).ok().unwrap_or($lsdata::default())
                    })
            }

            fn from_elem(
                elem_def: &$crate::webdynpro::element::definition::ElementDef<'a, Self>,
                element: scraper::ElementRef<'a>,
            ) -> Result<Self, $crate::webdynpro::error::WebDynproError> {
                Ok(Self::new(elem_def.id_cow(), element))
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
        
        $crate::webdynpro::element::define_lsdata! {
            $(#[$lsdata_outer])*
            $lsdata {
                $(
                    $(#[$lsdata_inner])*
                    $field : $ftype => $encoded,
                )+
            }
        }
    };
}

macro_rules! define_element_interactable {
    {
        $(#[$outer:meta])*
        $name:ident<$controlid:literal, $element_name:literal> {
            $($sfield:ident : $stype:ty),* $(,)?
        },
        $(#[$lsdata_outer:meta])*
        $lsdata:ident {
            $(
                $(#[$lsdata_inner:meta])*
                $field:ident: $ftype:ty => $encoded:literal
            ),* $(,)?
        }
    } => {
        $crate::webdynpro::element::define_element_base!{
            $(#[$outer])*
            $name<$controlid, $element_name> {
                lsevents: std::cell::OnceCell<Option<$crate::webdynpro::element::EventParameterMap>>,
                $($sfield : $stype, )*
            },
            $(#[$lsdata_outer])*
            $lsdata {
                $(
                    $(#[$lsdata_inner])*
                    $field: $ftype => $encoded, 
                )*
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

pub(crate) use define_lsdata;
pub(crate) use define_element_base;
pub(crate) use define_element_interactable;

macro_rules! register_elements {
    [$( $enum:ident : $type: ty ),+ $(,)?] => {
    	/// 도큐먼트에서 파싱한 [`Element`]를 공통의 타입으로 취급할 수 있게 하는 Wrapper
        #[allow(missing_docs)]
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
        	/// 분류를 알 수 없는 엘리먼트의 `scraper::ElementRef`로 [`ElementWrapper`]를 반환합니다.
            pub fn dyn_elem(element: scraper::ElementRef<'a>) -> Result<ElementWrapper, WebDynproError> {
                let value = element.value();
                let id = value.id().ok_or(BodyError::NoSuchAttribute("id".to_owned()))?.to_owned();
                #[allow(unreachable_patterns)]
                match element.value().attr("ct") {
                    $( Some(<$type>::CONTROL_ID) => Ok($crate::webdynpro::element::definition::ElementDef::<$type>::new_dynamic(id).from_elem(element)?.wrap()), )*
                    _ => Ok($crate::webdynpro::element::definition::ElementDef::<$crate::webdynpro::element::unknown::Unknown>::new_dynamic(id).from_elem(element)?.wrap())
                }
            }
        }

        $(
            impl<'a> std::convert::TryFrom<ElementWrapper<'a>> for $type {
                type Error = $crate::webdynpro::error::BodyError;
    
                fn try_from(wrapper: ElementWrapper<'a>) -> Result<$type, Self::Error> {
                    match wrapper {
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

/// 애플리케이션에서 쉽게 엘리먼트를 미리 정의할 수 있는 매크로
/// ### 예시
/// ```ignore
/// # use rusaint::define_elements;
/// # use rusaint::application::USaintApplication;
/// # use rusaint::webdynpro::element::{action::Button, selection::ComboBox};
/// impl<'a> USaintApplication {
///     define_elements!{
///         // const TEST_BUTTON: ElementDef<'a, Button<'a>> = ElementDef::new("TEST.BUTTON1"); 과 같음
///         TEST_BUTTON: Button<'a> = "TEST.BUTTON1";
///         // const TEST_COMBOBOX: ElementDef<'a, ComboBox<'a>> = ElementDef::new("TEST.COMBOBOX1"); 과 같음
///         TEST_COMBOBOX: ComboBox<'a> = "TEST.COMBOBOX1";
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_elements {
    ($(
        $(#[$attr:meta])*
        $v:vis $name:ident : $eltype:tt<$lt:lifetime> = $id:literal
    ;)+) => {
        $(
            $(#[$attr])*
            $v const $name: $crate::webdynpro::element::definition::ElementDef<$lt, $eltype<$lt>> = $crate::webdynpro::element::definition::ElementDef::new($id);
        )*
    };
    ($(
        $(#[$attr:meta])*
        $name:ident : $eltype:tt<$lt:lifetime> = $id:literal
    ;)+) => {
        $(
            $(#[$attr])*
            const $name: $crate::webdynpro::element::definition::ElementDef<$lt, $eltype<$lt>> = $crate::webdynpro::element::definition::ElementDef::new($id);
        )*
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

/// 엘리먼트의 기본 동작
pub trait Element<'a>: Sized {
	/// WebDynpro 상에서 사용하는 엘리먼트의 Id
    const CONTROL_ID: &'static str;
    /// WebDynpro 상에서 사용하는 엘리먼트의 이름
    const ELEMENT_NAME: &'static str;
/// 엘리먼트의 LSData
    type ElementLSData;
	
    /// 엘리먼트의 JSON 객체 형태의 LSData를 반환합니다.
    fn lsdata_elem(element: scraper::ElementRef) -> Result<Value, WebDynproError> {
        let raw_data = element.value().attr("lsdata").ok_or(ElementError::InvalidLSData(element.value().id().unwrap().to_string()))?;
        let normalized = normalize_lsjson(raw_data);
        return Ok(serde_json::from_str(&normalized).or(Err(ElementError::InvalidLSData(element.value().id().unwrap().to_string())))?);
    }

	/// 엘리먼트 정의와 [`Body`]에서 엘리먼트를 가져옵니다.
    fn from_body(elem_def: &ElementDef<'a, Self>, body: &'a Body) -> Result<Self, WebDynproError> {
        if let Some(node_id) = elem_def.node_id() {
            let mut hasher = DefaultHasher::new();
            body.hash(&mut hasher);
            let body_hash = hasher.finish();
            if body_hash == node_id.body_hash() {
                if let Some(elem) = body.document().tree.get(node_id.node_id()).and_then(|node| ElementRef::wrap(node)) {
                    return Self::from_elem(elem_def, elem)
                }
            }
        }
        let selector = &elem_def.selector().or(Err(BodyError::InvalidSelector))?;
        let element = body
            .document()
            .select(selector)
            .next()
            .ok_or(ElementError::InvalidId(elem_def.id().to_owned()))?;
        Self::from_elem(elem_def, element)
    }

	/// 엘리먼트 정의와 [`scraper::ElementRef`]에서 엘리먼트를 가져옵니다.
    fn from_elem(elem_def: &ElementDef<'a, Self>, element: scraper::ElementRef<'a>) -> Result<Self, WebDynproError>;

	/// 엘리먼트의 자식 엘리먼트를 가져옵니다.
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

	/// 엘리먼트의 자식 엘리먼트를 가져옵니다.
    fn children(&self) -> Vec<ElementWrapper<'a>>;

/// 엘리먼트의 LSData를 가져옵니다.
    fn lsdata(&self) -> &Self::ElementLSData;

	/// 엘리먼트의 Id를 가져옵니다.
    fn id(&self) -> &str;
	
    /// 엘리먼트의 [`scraper::ElementRef`]를 가져옵니다.
    fn element_ref(&self) -> &ElementRef<'a>;

	/// 엘리먼트를 [`ElementWrapper`]로 감쌉니다.
    fn wrap(self) -> ElementWrapper<'a>;
}

/// 이벤트를 통해 상호작용 할 수 있는 [`Element`]의 기본 동작
pub trait Interactable<'a>: Element<'a> {

	/// 엘리먼트가 이벤트를 발생시킬 수 있는가와 관계 없이 이벤트를 발생시킵니다.
    /// > | **주의** | 엘리먼트가 이벤트를 발생시킬 수 있는지 여부를 확인하지 않으므로 예상치 않은 오류가 발생할 수 있습니다.
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

	/// 엘리먼트가 발생시킬 수 있는 이벤트와 파라메터를 가져옵니다.
    fn lsevents_elem(element: scraper::ElementRef) -> Result<EventParameterMap, WebDynproError> {
        let raw_data = element.value().attr("lsevents").ok_or(BodyError::Invalid("Cannot find lsevents from element".to_string()))?;
        let normalized = normalize_lsjson(raw_data);
        let json: Map<String, Value> = serde_json::from_str::<Map<String, Value>>(&normalized).or(Err(BodyError::Invalid("Cannot deserialize lsevents field".to_string())))?.to_owned();
        Ok(json.into_iter().flat_map(|(key, value)| -> Result<(String, (UcfParameters, HashMap<String, String>)), BodyError> {
                    let mut parameters = value.as_array().ok_or(BodyError::Invalid("Cannot deserialize lsevents field".to_string()))?.to_owned().into_iter();
                    let raw_ucf = parameters.next().ok_or(BodyError::Invalid("Cannot deserialize lsevents field".to_string()))?;
                    let ucf: UcfParameters = serde_json::from_value(raw_ucf).or(Err(BodyError::Invalid("Cannot deserialize lsevents field".to_string())))?;
                    let mut custom = parameters.next().ok_or(BodyError::Invalid("Cannot deserialize lsevents field".to_string()))?.as_object().ok_or(BodyError::Invalid("Cannot deserialize lsevents field".to_string()))?.to_owned();
                    let custom_map = custom.iter_mut().map(|(key, value)| { 
                        (key.to_owned(), value.to_string())
                    }).collect::<HashMap<String, String>>();
                    Ok((key, (ucf, custom_map)))
                }).collect::<EventParameterMap>())
    }

	/// 엘리먼트의 주어진 이벤트에 대한 파라메터들을 가져옵니다.
    fn event_parameter(&self, event: &str) -> Result<&(UcfParameters, HashMap<String, String>), ElementError> {
        if let Some(lsevents) = self.lsevents() {
            lsevents.get(event).ok_or(ElementError::NoSuchEvent { element: self.id().to_string(), event: event.to_string() })
        } else {
            Err(ElementError::NoSuchEvent { element: self.id().to_string(), event: event.to_string() })
        }
    }

	/// 엘리먼트의 주어진 이벤트를 발생시킵니다.
    fn fire_event(&self, event: String, parameters: HashMap<String, String>) -> Result<Event, WebDynproError> {
        let (ucf_params, custom_params) = self.event_parameter(&event)?;
        Ok(unsafe { Self::fire_event_unchecked(event, parameters, ucf_params.to_owned(), custom_params.to_owned()) })
    }

	/// 주어진 엘리먼트의 이벤트 데이터를 반환합니다.
    fn lsevents(&self) -> Option<&EventParameterMap>;
}

/// 서브 엘리먼트의 기능
pub trait SubElement<'a>: Sized {
	/// WebDynpro 내부에서 사용하는 서브 엘리먼트의 Id
    const SUBCONTROL_ID: &'static str;
    /// WebDynpro 내부에서 사용하는 서브 엘리먼트의 이름
    const ELEMENT_NAME: &'static str;
    /// 서브 엘리먼트의 LSData
    type SubElementLSData;

	/// 서브 엘리먼트의 LSData를 JSON 객체 형태로 반환합니다.
    fn lsdata_elem(element: scraper::ElementRef) -> Result<Value, WebDynproError> {
        let raw_data = element.value().attr("lsdata").ok_or(ElementError::InvalidLSData(element.value().id().unwrap().to_string()))?;
        let normalized = normalize_lsjson(raw_data);
        return Ok(serde_json::from_str(&normalized).or(Err(ElementError::InvalidLSData(element.value().id().unwrap().to_string())))?);
    }

	/// 서브 엘리먼트의 정의와 [`Body`]로부터 서브 엘리먼트를 가져옵니다.
    fn from_body<Parent: Element<'a>>(
        elem_def: SubElementDef<'a, Parent, Self>,
        body: &'a Body,
    ) -> Result<Self, WebDynproError> {
        let selector = &elem_def.selector().or(Err(BodyError::InvalidSelector))?;
        let element = body
            .document()
            .select(selector)
            .next()
            .ok_or(ElementError::InvalidId(elem_def.id().to_owned()))?;
        Self::from_elem(elem_def, element)
    }

	/// 서브 엘리먼트 정의와[] `scraper::ElementRef`]로부터 서브 엘리먼트를 가져옵니다.
    fn from_elem<Parent: Element<'a>>(
        elem_def: SubElementDef<'a, Parent, Self>,
        element: scraper::ElementRef<'a>
    ) -> Result<Self, WebDynproError>;

	/// 서브 엘리먼트의 LSData를 가져옵니다.
    fn lsdata(&self) -> &Self::SubElementLSData;

	/// 서브 엘리먼트의 Id를 가져옵니다.
    fn id(&self) -> &str;

	/// 서브 엘리먼트의 [`scraper::ElementRef`]를 가져옵니다.
    fn element_ref(&self) -> &ElementRef<'a>;
}
