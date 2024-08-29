use std::{borrow::Cow, cell::OnceCell};

use serde::Deserialize;

use crate::webdynpro::{
    element::{parser::ElementParser, ElementDefWrapper, EventParameterMap},
    error::WebDynproError,
};

use self::item::{ListBoxItemDefWrapper, ListBoxItemInfo};

macro_rules! def_listbox_subset {
    [$({
        $(#[$attr:meta])*
        $name:ident = $id:literal,
        $(#[$def_attr:meta])*
        $def_name:ident
    }),+ $(,)?] => {$(
        $(#[$def_attr])*
        #[derive(Clone, Debug)]
        pub struct $def_name {
            id: std::borrow::Cow<'static, str>
        }

        impl $def_name {
            /// 엘리먼트의 정의를 생성합니다.
            pub const fn new(id: &'static str) -> Self {
                Self {
                    id: std::borrow::Cow::Borrowed(id)
                }
            }
        }

        impl<'body> $crate::webdynpro::element::definition::ElementDefinition<'body> for $def_name {
            type Element = $name<'body>;

            fn new_dynamic(id: String) -> Self {
                Self {
                    id: id.into()
                }
            }

            fn id(&self) -> &str {
                &self.id
            }

            fn id_cow(&self) -> Cow<'static, str> {
                self.id.clone()
            }
        }

        $(#[$attr])*
        #[derive(Debug)]
        pub struct $name<'a>($crate::webdynpro::element::selection::list_box::ListBox<'a>);

        impl<'a> $name<'a> {
            /// 이 엘리먼트의 원본 [`ListBox`]를 반환합니다.
            pub fn list_box(&self) -> &$crate::webdynpro::element::selection::list_box::ListBox<'a> {
                &self.0
            }
        }

        impl<'a> $crate::webdynpro::element::Element<'a> for $name<'a> {
            const CONTROL_ID: &'static str = $id;
            const ELEMENT_NAME: &'static str = "ListBox";

            type ElementLSData = ListBoxLSData;

            type Def = $def_name;

            fn lsdata(&self) -> &Self::ElementLSData {
                self.list_box().lsdata
                    .get_or_init(|| {
                        let Ok(lsdata_obj) = $crate::webdynpro::element::lsdata_tag(&self.list_box().tag) else {
                            return ListBoxLSData::default();
                        };
                        serde_json::from_value::<Self::ElementLSData>(lsdata_obj).unwrap_or(ListBoxLSData::default())
                    })
            }

            fn from_tag(elem_def: &impl $crate::webdynpro::element::definition::ElementDefinition<'a>, tag: tl::HTMLTag<'a>) -> Result<Self, $crate::webdynpro::error::WebDynproError> {
                Ok(Self::new($crate::webdynpro::element::definition::ElementDefinition::id_cow(elem_def), tag))
            }

            fn id(&self) -> &str {
                &self.list_box().id
            }

            fn tag(&self) -> &tl::HTMLTag<'a> {
                &self.list_box().tag
            }

            fn wrap(self) -> $crate::webdynpro::element::ElementWrapper<'a> {
                $crate::webdynpro::element::ElementWrapper::$name(self)
            }

            fn children(&self, parser: &'a $crate::webdynpro::element::parser::ElementParser) -> Vec<$crate::webdynpro::element::ElementWrapper<'a>> {
                $crate::webdynpro::element::children_tag(self.tag(), parser)
            }
        }

        impl<'a> $crate::webdynpro::element::Interactable<'a> for $name<'a> {
            fn lsevents(&self) -> Option<&EventParameterMap> {
                self.list_box().lsevents
                    .get_or_init(|| $crate::webdynpro::element::lsevents_tag(&self.list_box().tag).ok())
                    .as_ref()
            }
        }

        impl<'a> $name<'a> {
            #[doc = concat!("새로운 [`", stringify!($name), "`] 을 반환합니다.")]
            pub const fn new(id: std::borrow::Cow<'static, str>, tag: tl::HTMLTag<'a>) -> Self {
                Self($crate::webdynpro::element::selection::list_box::ListBox::new(id, tag))
            }
        }
    )+

    /// [`ListBox`] 분류의 엘리먼트의 정의를 위한 Wrapper
    #[derive(Clone, Debug)]
    pub enum ListBoxDefWrapper {
        $(
            $(#[$attr])*
            $name($def_name),
        )+
    }

    impl ListBoxDefWrapper {

        /// [`ElementDefWrapper`]에서 [`ListBoxDefWrapper`]로 변환을 시도합니다.
        pub fn from_def(element_def: $crate::webdynpro::element::ElementDefWrapper) -> Option<ListBoxDefWrapper> {
            match element_def {
                $($crate::webdynpro::element::ElementDefWrapper::$name(elem) => Some(ListBoxDefWrapper::$name(elem)),)+
                _ => None
            }
        }

        /// [`ListBoxWrapper`]를 가져옵니다.
        pub fn value<'body>(&self, parser: &'body $crate::webdynpro::element::parser::ElementParser) -> Result<ListBoxWrapper<'body>, $crate::webdynpro::error::WebDynproError> {
            match self {
                $(ListBoxDefWrapper::$name(def) => {
                    let raw_element = parser.element_from_def(def)?;
                    let elem_wrapper = $crate::webdynpro::element::Element::wrap(raw_element);
                    Ok(ListBoxWrapper::from_elements(elem_wrapper).ok_or($crate::webdynpro::error::BodyError::InvalidElement)?)
                },)+
            }
        }
    }

    /// [`ListBox`] 분류의 엘리먼트를 위한 공통된 Wrapper
    #[derive(Debug)]
    pub enum ListBoxWrapper<'a> {
        $(
            $(#[$attr])*
            $name($name<'a>),
        )+
    }

    impl<'a> ListBoxWrapper<'a> {

        /// [`ElementWrapper`](crate::webdynpro::element::ElementWrapper)에서 [`ListBoxWrapper`](crate::webdynpro::element::selection::list_box::ListBoxWrapper)로 변환을 시도합니다.
        pub fn from_elements(elements: $crate::webdynpro::element::ElementWrapper<'a>) -> Option<ListBoxWrapper<'a>> {
            match elements {
                $($crate::webdynpro::element::ElementWrapper::$name(elem) => Some(ListBoxWrapper::$name(elem)),)+
                _ => None
            }
        }
    }
};
}

/// 선택할 수 있는 목록
#[derive(Debug)]
pub struct ListBox<'a> {
    id: Cow<'static, str>,
    tag: tl::HTMLTag<'a>,
    lsdata: OnceCell<ListBoxLSData>,
    lsevents: OnceCell<Option<EventParameterMap>>,
    items: OnceCell<Vec<ListBoxItemDefWrapper>>,
}

def_listbox_subset![
    {
        #[doc = "팝업 형태로 표시되는 [`ListBox`]"]
        ListBoxPopup = "LIB_P",
        #[doc = "[`ListBoxPopup`]의 정의"]
        ListBoxPopupDef
    },
    {
        #[doc = "팝업 형태로 표시되며 데이터 구조가 있는 [`ListBox`]"]
        ListBoxPopupJson = "LIB_PJ",
        #[doc = "[`ListBoxPopupJson`]의 정의"]
        ListBoxPopupJsonDef
    },
    {
        #[doc = "팝업 형태로 표시되며 필터 입력 상자가 있는 [`ListBox`]"]
        ListBoxPopupFiltered = "LIB_PS",
        #[doc = "[`ListBoxPopupFiltered`]의 정의"]
        ListBoxPopupFilteredDef
    },
    {
        #[doc = "팝업 형태로 표시되며 데이터 구조가 있고 필터 입력 상자가 있는 [`ListBox`]"]
        ListBoxPopupJsonFiltered = "LIB_PJS",
        #[doc = "[`ListBoxPopupJsonFiltered`]의 정의"]
        ListBoxPopupJsonFilteredDef
    },
    {
        #[doc = "여러 선택지를 선택할 수 있는 [`ListBox`]"]
        ListBoxMultiple = "LIB_M",
        #[doc = "[`ListBoxMultiple`]의 정의"]
        ListBoxMultipleDef
    },
    {
        #[doc = "하나의 선택지만 선택할 수 있는 [`ListBox`]"]
        ListBoxSingle = "LIB_S",
        #[doc = "[`ListBoxSingle`]의 정의"]
        ListBoxSingleDef
    }
];

/// [`ListBox`] 내부 데이터
#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct ListBoxLSData {
    #[serde(rename = "0")]
    width: Option<String>,
    #[serde(rename = "1")]
    visible_items: Option<i32>,
    #[serde(rename = "2")]
    height: Option<String>,
    #[serde(rename = "3")]
    invalid: Option<bool>,
    #[serde(rename = "4")]
    enabled: Option<bool>,
    #[serde(rename = "5")]
    readonly: Option<bool>,
    #[serde(rename = "6")]
    multiple_selection: Option<bool>,
    #[serde(rename = "7")]
    required: Option<bool>,
    #[serde(rename = "8")]
    is_popup: Option<bool>,
    #[serde(rename = "9")]
    icon_visibility: Option<String>,
    #[serde(rename = "10")]
    first_value_visibility: Option<String>,
    #[serde(rename = "11")]
    second_value_visibility: Option<String>,
    #[serde(rename = "12")]
    visibility: Option<String>,
    #[serde(rename = "13")]
    input_state: Option<String>,
    #[serde(rename = "14")]
    drag_source_info: Option<String>,
    #[serde(rename = "15")]
    drop_target_info: Option<String>,
    #[serde(rename = "16")]
    scroll_top: Option<i32>,
    #[serde(rename = "17")]
    access_key: Option<String>,
    #[serde(rename = "18")]
    available: Option<bool>,
    #[serde(rename = "19")]
    server_filter: Option<String>,
    #[serde(rename = "20")]
    complete: Option<bool>,
    #[serde(rename = "21")]
    filtered: Option<bool>,
    #[serde(rename = "22")]
    table_data_definition: Option<String>,
    #[serde(rename = "23")]
    item_table_data: Option<String>,
    #[serde(rename = "24")]
    history_table_data: Option<String>,
    #[serde(rename = "25")]
    custom_data: Option<String>,
    #[serde(rename = "26")]
    custom_style: Option<String>,
    #[serde(rename = "27")]
    table_data_item_design: Option<String>,
    #[serde(rename = "28")]
    labelled_by: Option<String>,
}

impl<'a> ListBox<'a> {
    /// HTML 엘리먼트로부터 새로운 [`ListBox`]를 생성합니다.
    pub const fn new(id: Cow<'static, str>, tag: tl::HTMLTag<'a>) -> Self {
        Self {
            id,
            tag,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
            items: OnceCell::new(),
        }
    }

    /// [`ListBoxItemDefWrapper`]의 목록을 반환합니다.
    pub fn items(&self, parser: &'a ElementParser) -> impl Iterator<Item = &ListBoxItemDefWrapper> {
        self.items
            .get_or_init(|| {
                self.tag
                    .query_selector(parser.dom().parser(), "[ct]")
                    .into_iter()
                    .flatten()
                    .filter_map(|handle| {
                        let tag = handle.get(parser.dom().parser())?.as_tag()?.clone();
                        let element = ElementDefWrapper::from_tag(tag).ok()?;
                        match element {
                            ElementDefWrapper::ListBoxItem(item) => {
                                Some(ListBoxItemDefWrapper::Item(item))
                            }
                            ElementDefWrapper::ListBoxActionItem(item) => {
                                Some(ListBoxItemDefWrapper::ActionItem(item))
                            }
                            _ => None,
                        }
                    })
                    .collect()
            })
            .iter()
    }

    /// [`ListBoxItemInfo`]의 Iterator를 반환합니다.
    pub fn item_infos(
        &self,
        parser: &'a ElementParser,
    ) -> Result<impl Iterator<Item = ListBoxItemInfo>, WebDynproError> {
        let vec = self
            .tag
            .query_selector(parser.dom().parser(), "[ct]")
            .into_iter()
            .flatten()
            .filter_map(|handle| {
                Some(ListBoxItemInfo::from_tag(
                    handle.get(parser.dom().parser())?.as_tag()?.clone(),
                    parser,
                ))
            })
            .collect::<Result<Vec<ListBoxItemInfo>, WebDynproError>>()?;
        Ok(vec.into_iter())
    }
}

/// [`ListBoxItem`](item::ListBoxItem)과 [`ListBoxActionItem`](item::ListBoxActionItem)이 포함된 모듈
pub mod item;
