use std::{collections::HashMap, hash::{DefaultHasher, Hash, Hasher}};


use regex_lite::Regex;
use scraper::ElementRef;
use selection::CheckBox;
use serde_json::{Map, Value};

use self::{action::{Button, Link}, complex::SapTable, definition::ElementDefinition, graphic::Image, layout::{grid_layout::cell::GridLayoutCell, tab_strip::item::TabStripItem, ButtonRow, Container, FlowLayout, Form, GridLayout, PopupWindow, ScrollContainer, Scrollbar, TabStrip, Tray}, selection::{list_box::{item::{ListBoxActionItem, ListBoxItem}, ListBoxMultiple, ListBoxPopup, ListBoxPopupFiltered, ListBoxPopupJson, ListBoxPopupJsonFiltered, ListBoxSingle}, ComboBox}, system::{ClientInspector, Custom, LoadingPlaceholder}, text::{Caption, InputField, Label, TextView}};

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
        $(#[$def_outer:meta])*
        $def_name:ident,
        $(#[$lsdata_outer:meta])*
        $lsdata:ident {
            $(
                $(#[$lsdata_inner:meta])*
                $field:ident: $ftype:ty => $encoded:literal
            ),* $(,)?
        }
    } => {

        $(#[$def_outer])*
        #[derive(Clone, Debug)]
        pub struct $def_name {
            id: std::borrow::Cow<'static, str>,
            node_id: Option<crate::webdynpro::element::definition::ElementNodeId>
        }

        impl $def_name {
            /// 엘리먼트 정의를 생성합니다. 이 함수를 직접 실행하기보다는 [`define_elements`](crate::webdynpro::element::define_elements)매크로 사용을 추천합니다.
            pub const fn new(id: &'static str) -> Self {
                Self {
                    id: std::borrow::Cow::Borrowed(id),
                    node_id: None
                }
            }
        }

        impl<'body> $crate::webdynpro::element::definition::ElementDefinition<'body> for $def_name {
            type Element = $name<'body>;
            
            fn new_dynamic(id: String) -> Self {
                Self {
                    id: id.into(),
                    node_id: None
                }
            }

            fn from_element_ref(element_ref: scraper::ElementRef<'_>) -> Result<Self, $crate::webdynpro::error::WebDynproError> {
                let id = element_ref.value().id().ok_or($crate::webdynpro::error::BodyError::InvalidElement)?;
                Ok(Self {
                    id: id.to_string().into(),
                    node_id: None
                })
            }

            fn with_node_id(id: String, body_hash: u64, node_id: ego_tree::NodeId) -> Self {
                Self {
                    id: id.into(),
                    node_id: Some($crate::webdynpro::element::definition::ElementNodeId::new(body_hash, node_id))
                }
            }

            fn id(&self) -> &str {
                &self.id
            }

            fn id_cow(&self) -> Cow<'static, str> {
                self.id.clone()
            }

            fn node_id(&self) -> Option<&$crate::webdynpro::element::definition::ElementNodeId> {
                (&self.node_id).as_ref()
            }
        }

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

            type Def = $def_name;

            fn lsdata(&self) -> &Self::ElementLSData {
                self.lsdata
                    .get_or_init(|| {
                        let Ok(lsdata_obj) = Self::lsdata_element(self.element_ref).or_else(|e| { eprintln!("{:?}", e); Err(e) }) else {
                            return $lsdata::default();
                        };
                        serde_json::from_value::<Self::ElementLSData>(lsdata_obj).or_else(|e| { eprintln!("{:?}", e); Err(e) }).ok().unwrap_or($lsdata::default())
                    })
            }

            fn from_element(
                element_def: &impl $crate::webdynpro::element::definition::ElementDefinition<'a>,
                element: scraper::ElementRef<'a>,
            ) -> Result<Self, $crate::webdynpro::error::WebDynproError> {
                Ok(Self::new($crate::webdynpro::element::definition::ElementDefinition::id_cow(element_def), element))
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
                Self::children_element(self.element_ref().clone())
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
        $(#[$def_outer:meta])*
        $def_name:ident,
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
            $(#[$def_outer])*
            $def_name,
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
                    .get_or_init(|| Self::lsevents_element(self.element_ref).ok())
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
            pub fn dyn_element(element: scraper::ElementRef<'a>) -> Result<ElementWrapper, WebDynproError> {
                let value = element.value();
                let id = value.id().ok_or(BodyError::NoSuchAttribute("id".to_owned()))?.to_owned();
                #[allow(unreachable_patterns)]
                match element.value().attr("ct") {
                    $( Some(<$type>::CONTROL_ID) => Ok(<$type as $crate::webdynpro::element::Element<'a>>::Def::new_dynamic(id).from_element(element)?.wrap()), )*
                    _ => Ok(<$crate::webdynpro::element::unknown::Unknown as $crate::webdynpro::element::Element<'a>>::Def::new_dynamic(id).from_element(element)?.wrap())
                }
            }

            /// 엘리먼트의 id를 반환합니다.
            pub fn id(&self) -> &str {
                match self {
                    $( ElementWrapper::$enum(element) => <$type as $crate::webdynpro::element::Element<'a>>::id(element), )*
                    ElementWrapper::Unknown(element) => <$crate::webdynpro::element::unknown::Unknown<'a> as $crate::webdynpro::element::Element<'a>>::id(element),
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

        /// 다양한 [`Element`]를 대상으로 하는 [`ElementDefinition`]를 공통의 타입으로 취급할 수 있게 하는 Wrapper
        #[allow(missing_docs)]
        #[derive(Clone, Debug)]
        pub enum ElementDefWrapper<'a> {
            $( $enum(<$type as $crate::webdynpro::element::Element<'a>>::Def), )*
            Unknown(<$crate::webdynpro::element::unknown::Unknown<'a> as $crate::webdynpro::element::Element<'a>>::Def)
        }

        impl<'a> ElementDefWrapper<'a> {
        	/// 분류를 알 수 없는 엘리먼트의 `scraper::ElementRef`로 [`ElementDefWrapper`]를 반환합니다.
            pub fn dyn_elem_def(element: scraper::ElementRef<'a>) -> Result<ElementDefWrapper<'a>, WebDynproError> {
                let value = element.value();
                let id = value.id().ok_or(BodyError::NoSuchAttribute("id".to_owned()))?.to_owned();
                #[allow(unreachable_patterns)]
                match element.value().attr("ct") {
                    $( Some(<$type>::CONTROL_ID) => Ok(ElementDefWrapper::$enum(<$type as $crate::webdynpro::element::Element<'a>>::Def::new_dynamic(id))), )*
                    _ => Ok(ElementDefWrapper::Unknown(<$crate::webdynpro::element::unknown::Unknown<'a> as $crate::webdynpro::element::Element<'a>>::Def::new_dynamic(id)))
                }
            }

            /// 엘리먼트의 id를 반환합니다.
            pub fn id(&self) -> &str {
                match self {
                    $( ElementDefWrapper::$enum(element_def) => <$type as $crate::webdynpro::element::Element<'a>>::Def::id(element_def), )*
                    ElementDefWrapper::Unknown(element_def) => <$crate::webdynpro::element::unknown::Unknown<'a> as $crate::webdynpro::element::Element<'a>>::Def::id(element_def),
                }
            }

            /// 엘리먼트의 [`scraper::Selector`]를 반환합니다.
            pub fn selector(&self) -> Result<scraper::Selector, WebDynproError> {
                match self {
                    $( ElementDefWrapper::$enum(element_def) => <$type as $crate::webdynpro::element::Element<'a>>::Def::selector(element_def), )*
                    ElementDefWrapper::Unknown(element_def) => <$crate::webdynpro::element::unknown::Unknown<'a> as $crate::webdynpro::element::Element<'a>>::Def::selector(element_def),
                }
            }

            /// [`Body`](crate::webdynpro::client::body::Body)에서 [`ElementWrapper`]를 반환합니다.
            pub fn from_body(&self, body: &'a $crate::webdynpro::client::body::Body) -> Result<$crate::webdynpro::element::ElementWrapper<'a>, WebDynproError> {
                let selector = &self.selector()?;
                let element = body
                    .document()
                    .select(selector)
                    .next()
                    .ok_or(ElementError::InvalidId(self.id().to_owned()))?;
                $crate::webdynpro::element::ElementWrapper::dyn_element(element)
            }
        }
        
    };
}

register_elements![
    Button: Button<'a>,
    ButtonRow: ButtonRow<'a>,
    CheckBox: CheckBox<'a>,
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
            $v const $name: <$eltype<$lt> as $crate::webdynpro::element::Element<$lt>>::Def = <$eltype<$lt> as $crate::webdynpro::element::Element<$lt>>::Def::new($id);
        )*
    };
    ($(
        $(#[$attr:meta])*
        $name:ident : $eltype:tt<$lt:lifetime> = $id:literal
    ;)+) => {
        $(
            $(#[$attr])*
            const $name: <$eltype<$lt> as $crate::webdynpro::element::Element<$lt>>::Def = <$eltype<$lt> as $crate::webdynpro::element::Element<$lt>>::Def::new($id);
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

    /// 엘리먼트의 정의
    type Def: ElementDefinition<'a>;
	
    /// 엘리먼트의 JSON 객체 형태의 LSData를 반환합니다.
    fn lsdata_element(element: scraper::ElementRef) -> Result<Value, WebDynproError> {
        let raw_data = element.value().attr("lsdata").ok_or(ElementError::InvalidLSData(element.value().id().unwrap().to_string()))?;
        let normalized = normalize_lsjson(raw_data);
        return Ok(serde_json::from_str(&normalized).or(Err(ElementError::InvalidLSData(element.value().id().unwrap().to_string())))?);
    }

	/// 엘리먼트 정의와 [`Body`]에서 엘리먼트를 가져옵니다.
    fn from_body(elem_def: &impl ElementDefinition<'a>, body: &'a Body) -> Result<Self, WebDynproError> {
        if let Some(node_id) = elem_def.node_id() {
            let mut hasher = DefaultHasher::new();
            body.hash(&mut hasher);
            let body_hash = hasher.finish();
            if body_hash == node_id.body_hash() {
                if let Some(elem) = body.document().tree.get(node_id.node_id()).and_then(|node| ElementRef::wrap(node)) {
                    return Self::from_element(elem_def, elem)
                }
            }
        }
        let selector = &elem_def.selector()?;
        let element = body
            .document()
            .select(selector)
            .next()
            .ok_or(ElementError::InvalidId(elem_def.id().to_owned()))?;
        Self::from_element(elem_def, element)
    }

	/// 엘리먼트 정의와 [`scraper::ElementRef`]에서 엘리먼트를 가져옵니다.
    fn from_element(elem_def: &impl ElementDefinition<'a>, element: scraper::ElementRef<'a>) -> Result<Self, WebDynproError>;

	/// 엘리먼트의 자식 엘리먼트를 가져옵니다.
    fn children_element(root: scraper::ElementRef<'a>) -> Vec<ElementWrapper<'a>> {
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
        cts.into_iter().rev().filter_map(|eref| ElementWrapper::dyn_element(eref).ok()).collect()
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
    fn lsevents_element(element: scraper::ElementRef) -> Result<EventParameterMap, WebDynproError> {
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

impl<'a> ElementWrapper<'a> {
    /// 주어진 엘리먼트를 텍스트 형태로 변환하려고 시도합니다.
    pub fn textise(&self) -> Result<String, WebDynproError> {
        match self {
            ElementWrapper::TextView(tv) => Ok(tv.text().to_string()),
            ElementWrapper::Caption(cp) => Ok(cp.text().to_string()),
            ElementWrapper::CheckBox(c) => Ok(format!("{}", c.checked())),
            _ => Err(WebDynproError::Element(ElementError::InvalidContent { element: self.id().to_string(), content: "This element is cannot be textised.".to_string() }))
        }
    }
}

/// [`SubElement`](crate::webdynpro::element::sub::SubElement) 트레이트 모듈
pub mod sub;
mod parser;
