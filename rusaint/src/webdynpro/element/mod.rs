use std::collections::HashMap;

use regex_lite::Regex;
use selection::CheckBox;
use serde_json::{Map, Value};

use self::{
    action::{Button, Link},
    complex::SapTable,
    definition::ElementDefinition,
    graphic::Image,
    layout::{
        grid_layout::cell::GridLayoutCell, tab_strip::item::TabStripItem, ButtonRow, Container,
        FlowLayout, Form, GridLayout, PopupWindow, ScrollContainer, Scrollbar, TabStrip, Tray,
    },
    selection::{
        list_box::{
            item::{ListBoxActionItem, ListBoxItem},
            ListBoxMultiple, ListBoxPopup, ListBoxPopupFiltered, ListBoxPopupJson,
            ListBoxPopupJsonFiltered, ListBoxSingle,
        },
        ComboBox,
    },
    system::{ClientInspector, Custom, LoadingPlaceholder},
    text::{Caption, InputField, Label, TextView},
};

use super::{
    error::{BodyError, ElementError, WebDynproError},
    event::{ucf_parameters::UcfParameters, Event, EventBuilder},
};

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

macro_rules! register_elements {
    [$( $enum:ident : $type: ty ),+ $(,)?] => {
    	/// 도큐먼트에서 파싱한 [`Element`]를 공통의 타입으로 취급할 수 있게 하는 Wrapper
        #[allow(missing_docs)]
        pub enum ElementWrapper<'body> {
            $( $enum($type), )*
            Unknown($crate::webdynpro::element::unknown::Unknown<'body>)
        }

        impl<'body> std::fmt::Debug for ElementWrapper<'body> {
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

        impl<'body> ElementWrapper<'body> {
        	/// 분류를 알 수 없는 엘리먼트의 `tl::HTMLTag`로 [`ElementWrapper`]를 반환합니다.
            pub fn from_tag(tag: tl::HTMLTag<'body>) -> Result<ElementWrapper<'body>, WebDynproError> {
                let attrs = tag.attributes();
                let id = attrs.get("id").flatten().ok_or(BodyError::NoSuchAttribute("id".to_owned()))?.as_utf8_str().to_string();
                #[allow(unreachable_patterns)]
                match attrs.get("ct").flatten().and_then(|bytes| bytes.try_as_utf8_str()) {
                    $( Some(<$type>::CONTROL_ID) => {
                        let def = <$type as $crate::webdynpro::element::Element<'body>>::Def::new_dynamic(id);
                        Ok(<$type as $crate::webdynpro::element::Element<'body>>::from_tag(&def, tag)?.wrap())
                    }, )*
                    _ => {
                        let def = <$crate::webdynpro::element::unknown::Unknown as $crate::webdynpro::element::Element<'body>>::Def::new_dynamic(id);
                        Ok(<$crate::webdynpro::element::unknown::Unknown as $crate::webdynpro::element::Element<'body>>::from_tag(&def, tag)?.wrap())
                    }
                }
            }

            /// 엘리먼트의 id를 반환합니다.
            pub fn id(&self) -> &str {
                match self {
                    $( ElementWrapper::$enum(element) => <$type as $crate::webdynpro::element::Element<'body>>::id(element), )*
                    ElementWrapper::Unknown(element) => <$crate::webdynpro::element::unknown::Unknown<'body> as $crate::webdynpro::element::Element<'body>>::id(element),
                }
            }
        }

        $(
            impl<'body> std::convert::TryFrom<ElementWrapper<'body>> for $type {
                type Error = $crate::webdynpro::error::BodyError;

                fn try_from(wrapper: ElementWrapper<'body>) -> Result<$type, Self::Error> {
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
        pub enum ElementDefWrapper<'body> {
            $( $enum(<$type as $crate::webdynpro::element::Element<'body>>::Def), )*
            Unknown(<$crate::webdynpro::element::unknown::Unknown<'body> as $crate::webdynpro::element::Element<'body>>::Def)
        }

        impl<'body> ElementDefWrapper<'body> {
        	/// 분류를 알 수 없는 엘리먼트의 `tl::HTMLTag`로 [`ElementDefWrapper`]를 반환합니다.
            pub fn from_tag(tag: tl::HTMLTag<'body>) -> Result<ElementDefWrapper<'body>, WebDynproError> {
                let attrs = tag.attributes();
                let id = attrs.get("id").flatten().ok_or(BodyError::NoSuchAttribute("id".to_owned()))?.as_utf8_str().to_string();
                #[allow(unreachable_patterns)]
                match attrs.get("ct").flatten().and_then(|bytes| bytes.try_as_utf8_str()) {
                    $( Some(<$type>::CONTROL_ID) => Ok(ElementDefWrapper::$enum(<$type as $crate::webdynpro::element::Element<'body>>::Def::new_dynamic(id))), )*
                    _ => Ok(ElementDefWrapper::Unknown(<$crate::webdynpro::element::unknown::Unknown<'body> as $crate::webdynpro::element::Element<'body>>::Def::new_dynamic(id)))
                }
            }

            /// 엘리먼트의 id를 반환합니다.
            pub fn id(&self) -> &str {
                match self {
                    $( ElementDefWrapper::$enum(element_def) => <$type as $crate::webdynpro::element::Element<'body>>::Def::id(element_def), )*
                    ElementDefWrapper::Unknown(element_def) => <$crate::webdynpro::element::unknown::Unknown<'body> as $crate::webdynpro::element::Element<'body>>::Def::id(element_def),
                }
            }
        }

    };
}

register_elements![
    Button: Button<'body>,
    ButtonRow: ButtonRow<'body>,
    CheckBox: CheckBox<'body>,
    ClientInspector: ClientInspector<'body>,
    ComboBox: ComboBox<'body>,
    Container: Container<'body>,
    Custom: Custom,
    FlowLayout: FlowLayout<'body>,
    Form: Form<'body>,
    GridLayout: GridLayout<'body>,
    GridLayoutCell: GridLayoutCell<'body>,
    Image: Image<'body>,
    InputField: InputField<'body>,
    Label: Label<'body>,
    Link: Link<'body>,
    ListBoxPopup: ListBoxPopup<'body>,
    ListBoxPopupFiltered: ListBoxPopupFiltered<'body>,
    ListBoxPopupJson: ListBoxPopupJson<'body>,
    ListBoxPopupJsonFiltered: ListBoxPopupJsonFiltered<'body>,
    ListBoxMultiple: ListBoxMultiple<'body>,
    ListBoxSingle: ListBoxSingle<'body>,
    ListBoxItem: ListBoxItem<'body>,
    ListBoxActionItem: ListBoxActionItem<'body>,
    LoadingPlaceholder: LoadingPlaceholder<'body>,
    PopupWindow: PopupWindow<'body>,
    TabStrip: TabStrip<'body>,
    TabStripItem: TabStripItem<'body>,
    Tray: Tray<'body>,
    SapTable: SapTable<'body>,
    Scrollbar: Scrollbar<'body>,
    ScrollContainer: ScrollContainer<'body>,
    TextView: TextView<'body>,
    Caption: Caption<'body>,
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

use crate::webdynpro::element::parser::ElementParser;
pub use define_elements;

// TODO: Do multiple replacements without owning
fn normalize_lsjson(lsjson: &str) -> String {
    let quote_key = Regex::new(r"([{,])(\w+):").unwrap();
    let quote_to_double = Regex::new(r"([^\\])'([\s\S]*?)'").unwrap();
    let convert_escape_to_rust = Regex::new(r"\\x([a-f0-9]{2})").unwrap();
    let quoted = quote_key.replace_all(lsjson, r#"$1"$2":"#).into_owned();
    let double_quoted = quote_to_double
        .replace_all(&quoted, r#"$1"$2""#)
        .into_owned();
    let ret = convert_escape_to_rust
        .replace_all(&double_quoted, r"\u00$1")
        .into_owned();
    ret
}

pub(crate) fn children_tag<'body>(
    root: &tl::HTMLTag<'body>,
    parser: &'body ElementParser,
) -> Vec<ElementWrapper<'body>> {
    let mut next_refs = vec![root.clone()];
    let mut cts: Vec<tl::HTMLTag> = vec![];
    while let Some(elem) = next_refs.pop() {
        for child in elem.children().all(parser.dom().parser()) {
            if let tl::Node::Tag(child_tag) = child {
                if child_tag.attributes().get("ct").flatten().is_some() {
                    cts.push(child_tag.clone());
                } else {
                    next_refs.push(child_tag.clone());
                }
            }
        }
    }
    cts.into_iter()
        .rev()
        .filter_map(|tag| ElementWrapper::from_tag(tag).ok())
        .collect()
}

/// 엘리먼트의 JSON 객체 형태의 LSData를 반환합니다.
pub(crate) fn lsdata_tag(tag: &tl::HTMLTag) -> Result<Value, WebDynproError> {
    let attr = tag.attributes();
    let raw_data = attr
        .get("lsdata")
        .flatten()
        .ok_or(ElementError::InvalidLSData(
            attr.get("id").flatten().unwrap().as_utf8_str().to_string(),
        ))?
        .as_utf8_str();
    let normalized = normalize_lsjson(&raw_data);
    Ok(
        serde_json::from_str(&normalized).or(Err(ElementError::InvalidLSData(
            attr.get("id").flatten().unwrap().as_utf8_str().to_string(),
        )))?,
    )
}

/// 엘리먼트의 기본 동작
pub trait Element<'body>: Sized {
    /// WebDynpro 상에서 사용하는 엘리먼트의 Id
    const CONTROL_ID: &'static str;
    /// WebDynpro 상에서 사용하는 엘리먼트의 이름
    const ELEMENT_NAME: &'static str;
    /// 엘리먼트의 LSData
    type ElementLSData;

    /// 엘리먼트의 정의
    type Def: ElementDefinition<'body>;

    /// 엘리먼트 정의와 [`tl::HTMLTag`]에서 엘리먼트를 가져옵니다.
    fn from_tag(
        elem_def: &impl ElementDefinition<'body>,
        tag: tl::HTMLTag<'body>,
    ) -> Result<Self, WebDynproError>;

    /// 엘리먼트의 자식 엘리먼트를 가져옵니다.
    fn children(&self, parser: &'body ElementParser) -> Vec<ElementWrapper<'body>>;

    /// 엘리먼트의 LSData를 가져옵니다.
    fn lsdata(&self) -> &Self::ElementLSData;

    /// 엘리먼트의 Id를 가져옵니다.
    fn id(&self) -> &str;

    /// 엘리먼트의 [`tl::HTMLTag`]를 가져옵니다.
    fn tag(&self) -> &tl::HTMLTag<'body>;

    /// 엘리먼트를 [`ElementWrapper`]로 감쌉니다.
    fn wrap(self) -> ElementWrapper<'body>;
}

/// 이벤트를 통해 상호작용 할 수 있는 [`Element`]의 기본 동작
pub trait Interactable<'body>: Element<'body> {
    /// 엘리먼트가 이벤트를 발생시킬 수 있는가와 관계 없이 이벤트를 발생시킵니다.
    /// > | **주의** | 엘리먼트가 이벤트를 발생시킬 수 있는지 여부를 확인하지 않으므로 예상치 않은 오류가 발생할 수 있습니다.
    unsafe fn fire_event_unchecked(
        event: String,
        parameters: HashMap<String, String>,
        ucf_params: UcfParameters,
        custom_params: HashMap<String, String>,
    ) -> Event {
        EventBuilder::default()
            .control(Self::ELEMENT_NAME.to_owned())
            .event(event)
            .parameters(parameters)
            .ucf_parameters(ucf_params)
            .custom_parameters(custom_params)
            .build()
            .unwrap()
    }

    /// 엘리먼트의 주어진 이벤트에 대한 파라메터들을 가져옵니다.
    fn event_parameter(
        &self,
        event: &str,
    ) -> Result<&(UcfParameters, HashMap<String, String>), ElementError> {
        if let Some(lsevents) = self.lsevents() {
            lsevents.get(event).ok_or(ElementError::NoSuchEvent {
                element: self.id().to_string(),
                event: event.to_string(),
            })
        } else {
            Err(ElementError::NoSuchEvent {
                element: self.id().to_string(),
                event: event.to_string(),
            })
        }
    }

    /// 엘리먼트의 주어진 이벤트를 발생시킵니다.
    fn fire_event(
        &self,
        event: String,
        parameters: HashMap<String, String>,
    ) -> Result<Event, WebDynproError> {
        let (ucf_params, custom_params) = self.event_parameter(&event)?;
        Ok(unsafe {
            Self::fire_event_unchecked(
                event,
                parameters,
                ucf_params.to_owned(),
                custom_params.to_owned(),
            )
        })
    }

    /// 주어진 엘리먼트의 이벤트 데이터를 반환합니다.
    fn lsevents(&self) -> Option<&EventParameterMap>;
}

/// 엘리먼트가 발생시킬 수 있는 이벤트와 파라메터를 가져옵니다.
pub(self) fn lsevents_tag(tag: &tl::HTMLTag) -> Result<EventParameterMap, WebDynproError> {
    let raw_data = tag
        .attributes()
        .get("lsevents")
        .flatten()
        .ok_or(BodyError::Invalid(
            "Cannot find lsevents from element".to_string(),
        ))?
        .as_utf8_str();
    let normalized = normalize_lsjson(&raw_data);
    let json: Map<String, Value> = serde_json::from_str::<Map<String, Value>>(&normalized)
        .or(Err(BodyError::Invalid(
            "Cannot deserialize lsevents field".to_string(),
        )))?
        .to_owned();
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

impl<'body> ElementWrapper<'body> {
    /// 주어진 엘리먼트를 텍스트 형태로 변환하려고 시도합니다.
    pub fn textise(&self, parser: &'body ElementParser) -> Result<String, WebDynproError> {
        match self {
            ElementWrapper::TextView(tv) => Ok(tv.text(parser).to_string()),
            ElementWrapper::Caption(cp) => Ok(cp.text(parser).to_string()),
            ElementWrapper::CheckBox(c) => Ok(format!("{}", c.checked())),
            _ => Err(WebDynproError::Element(ElementError::InvalidContent {
                element: self.id().to_string(),
                content: "This element is cannot be textised.".to_string(),
            })),
        }
    }
}

mod macros;

/// 엘리먼트를 파싱하기 위한 [`ElementParser`]를 위한 모듈
pub mod parser;
/// [`SubElement`](sub::SubElement) 트레이트 모듈
pub mod sub;
