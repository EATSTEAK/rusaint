use std::collections::HashMap;

use regex_lite::Regex;
use scraper::ElementRef;
use selection::CheckBox;

use self::{
    action::{Button, Link},
    complex::SapTable,
    definition::ElementDefinition,
    graphic::Image,
    layout::{
        ButtonRow, Container, FlowLayout, Form, GridLayout, PopupWindow, ScrollContainer,
        Scrollbar, TabStrip, Tray, grid_layout::cell::GridLayoutCell,
        tab_strip::item::TabStripItem,
    },
    selection::{
        ComboBox,
        list_box::{
            ListBoxMultiple, ListBoxPopup, ListBoxPopupFiltered, ListBoxPopupJson,
            ListBoxPopupJsonFiltered, ListBoxSingle,
            item::{ListBoxActionItem, ListBoxItem},
        },
    },
    system::{ClientInspector, Custom, LoadingPlaceholder},
    text::{Caption, InputField, Label, TextView},
};

use super::{
    error::{BodyError, ElementError, WebDynproError},
    event::{Event, EventBuilder, ucf_parameters::UcfParameters},
};

/// [`SubElement`](sub::SubElement) 트레이트 모듈
pub mod sub;

/// 엘리먼트의 정의를 다루는 모듈
pub mod definition;

/// [`ElementParser`]의 모듈
pub mod parser;

/// 엘리먼트 생성에 사용되는 메크로
pub mod macros;

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
        #[allow(missing_docs, clippy::large_enum_variant)]
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
            pub fn from_ref(element: scraper::ElementRef<'a>) -> Result<ElementWrapper<'a>, WebDynproError> {
                let value = element.value();
                let id = value.id().ok_or(BodyError::NoSuchAttribute("id".to_owned()))?.to_owned();
                #[allow(unreachable_patterns)]
                match element.value().attr("ct") {
                    $( Some(<$type>::CONTROL_ID) => {
                        let def = <$type as $crate::webdynpro::element::Element<'a>>::Def::new_dynamic(id);
                        Ok(<$type as $crate::webdynpro::element::Element<'a>>::from_ref(&def, element)?.wrap())
                    }, )*
                    _ => {
                        let def = <$crate::webdynpro::element::unknown::Unknown as $crate::webdynpro::element::Element<'a>>::Def::new_dynamic(id);
                        Ok(<$crate::webdynpro::element::unknown::Unknown as $crate::webdynpro::element::Element<'a>>::from_ref(&def, element)?.wrap())
                    }
                }
            }

            /// 주어진 [`ElementDefWrapper`]와 일치하는 [`ElementWrapper`]를 반환합니다.
            pub fn from_def(wrapper: &'a ElementDefWrapper, parser: &'a $crate::webdynpro::element::parser::ElementParser) -> Result<ElementWrapper<'a>, WebDynproError> {
                match wrapper {
                    $( ElementDefWrapper::$enum(def) => Ok(parser.element_from_def(def)?.wrap()), )*
                    ElementDefWrapper::Unknown(def) => Ok(parser.element_from_def(def)?.wrap()),
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
            pub fn from_ref(element: scraper::ElementRef<'a>) -> Result<ElementDefWrapper<'a>, WebDynproError> {
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
    let double_quoted = quote_to_double
        .replace_all(&quoted, r#"$1"$2""#)
        .into_owned();
    convert_escape_to_rust
        .replace_all(&double_quoted, r"\u00$1")
        .into_owned()
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

    /// 엘리먼트 정의와 [`ElementRef`]에서 엘리먼트를 가져옵니다.
    fn from_ref(
        elem_def: &impl ElementDefinition<'a>,
        element: ElementRef<'a>,
    ) -> Result<Self, WebDynproError>;

    /// 엘리먼트의 자식 엘리먼트를 가져옵니다.
    fn children(&self) -> Vec<ElementWrapper<'a>>;

    /// 엘리먼트의 LSData를 가져옵니다.
    fn lsdata(&self) -> &Self::ElementLSData;

    /// 엘리먼트의 Id를 가져옵니다.
    fn id(&self) -> &str;

    /// 엘리먼트의 [`ElementRef`]를 가져옵니다.
    fn element_ref(&self) -> &ElementRef<'a>;

    /// 엘리먼트를 [`ElementWrapper`]로 감쌉니다.
    fn wrap(self) -> ElementWrapper<'a>;
}

/// 이벤트를 통해 상호작용 할 수 있는 [`Element`]의 기본 동작
pub trait Interactable<'a>: Element<'a> {
    /// 엘리먼트가 이벤트를 발생시킬 수 있는가와 관계 없이 이벤트를 발생시킵니다.
    /// # Safety
    /// 엘리먼트가 이벤트를 발생시킬 수 있는지 여부를 확인하고 이 함수를 호출해야 합니다.
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

impl ElementWrapper<'_> {
    /// 주어진 엘리먼트를 텍스트 형태로 변환하려고 시도합니다.
    pub fn textise(&self) -> Result<String, WebDynproError> {
        match self {
            ElementWrapper::TextView(tv) => Ok(tv.text().to_string()),
            ElementWrapper::Caption(cp) => Ok(cp.text().to_string()),
            ElementWrapper::CheckBox(c) => Ok(format!("{}", c.checked())),
            ElementWrapper::ComboBox(cb) => Ok(cb.value().unwrap_or_default().to_string()),
            ElementWrapper::InputField(ifield) => {
                Ok(ifield.value().unwrap_or_default().to_string())
            }
            _ => Err(WebDynproError::Element(ElementError::InvalidContent {
                element: self.id().to_string(),
                content: "This element is cannot be textised.".to_string(),
            })),
        }
    }
}
mod utils;
