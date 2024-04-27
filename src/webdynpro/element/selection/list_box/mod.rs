use std::{borrow::Cow, cell::OnceCell};

use serde::Deserialize;

use crate::webdynpro::element::{ElementDef, ElementWrapper, EventParameterMap};

use self::item::ListBoxItemWrapper;

macro_rules! def_listbox_subset {
    [$(
        $(#[$attr:meta])*
        $name:ident = $id:literal
    ),+ $(,)?] => {$(
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

            fn lsdata(&self) -> &Self::ElementLSData {
                self.list_box().lsdata
                    .get_or_init(|| {
                        let Ok(lsdata_obj) = Self::lsdata_elem(self.list_box().element_ref) else {
                            return ListBoxLSData::default();
                        };
                        serde_json::from_value::<Self::ElementLSData>(lsdata_obj).unwrap_or(ListBoxLSData::default())
                    })
            }

            fn from_elem(elem_def: &ElementDef<'a, Self>, element: scraper::ElementRef<'a>) -> Result<Self, $crate::webdynpro::error::WebDynproError> {
                Ok(Self::new(elem_def.id().into(), element))
            }

            fn id(&self) -> &str {
                &self.list_box().id
            }

            fn element_ref(&self) -> &scraper::ElementRef<'a> {
                &self.list_box().element_ref
            }

            fn wrap(self) -> $crate::webdynpro::element::ElementWrapper<'a> {
                $crate::webdynpro::element::ElementWrapper::$name(self)
            }

            fn children(&self) -> Vec<$crate::webdynpro::element::ElementWrapper<'a>> {
                Self::children_elem(self.element_ref().clone())
            }
        }

        impl<'a> $crate::webdynpro::element::Interactable<'a> for $name<'a> {
            fn lsevents(&self) -> Option<&EventParameterMap> {
                self.list_box().lsevents
                    .get_or_init(|| Self::lsevents_elem(self.list_box().element_ref).ok())
                    .as_ref()
            }
        }

        impl<'a> $name<'a> {
            #[doc = concat!("새로운 [`", stringify!($name), "`] 을 반환합니다.")]
            pub const fn new(id: std::borrow::Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
                Self($crate::webdynpro::element::selection::list_box::ListBox::new(id, element_ref))
            }
        }
    )+

    /// [`ListBox`] 분류의 엘리먼트를 위한 공통된 Wrapper
    #[derive(Debug)]
    pub enum ListBoxWrapper<'a> {
        $(
            $(#[$attr])*
            $name($name<'a>),
        )+
    }

    impl<'a> ListBoxWrapper<'a> {

        /// [`ElementWrapper`]에서 [`ListBoxWrapper`]로 변환을 시도합니다.
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
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<ListBoxLSData>,
    lsevents: OnceCell<Option<EventParameterMap>>,
    items: OnceCell<Vec<ListBoxItemWrapper<'a>>>,
}

def_listbox_subset![
    #[doc = "팝업 형태로 표시되는 [`ListBox`]"]
    ListBoxPopup = "LIB_P",
    #[doc = "팝업 형태로 표시되며 데이터 구조가 있는 [`ListBox`]"]
    ListBoxPopupJson = "LIB_PJ",
    #[doc = "팝업 형태로 표시되며 필터 입력 상자가 있는 [`ListBox`]"]
    ListBoxPopupFiltered = "LIB_PS",
    #[doc = "팝업 형태로 표시되며 데이터 구조가 있고 필터 입력 상자가 있는 [`ListBox`]"]
    ListBoxPopupJsonFiltered = "LIB_PJS",
    #[doc = "여러 선택지를 선택할 수 있는 [`ListBox`]"]
    ListBoxMultiple = "LIB_M",
    #[doc = "하나의 선택지만 선택할 수 있는 [`ListBox`]"]
    ListBoxSingle = "LIB_S"
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
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
            items: OnceCell::new(),
        }
    }

    /// [`ListBoxItemWrapper`]의 목록을 반환합니다.
    pub fn items(&self) -> impl Iterator<Item = &ListBoxItemWrapper<'a>> {
        self.items
            .get_or_init(|| {
                let items_selector = scraper::Selector::parse("[ct]").unwrap();
                self.element_ref
                    .select(&items_selector)
                    .filter_map(|elem_ref| {
                        let element = ElementWrapper::dyn_elem(elem_ref).ok()?;
                        match element {
                            ElementWrapper::ListBoxItem(item) => {
                                Some(ListBoxItemWrapper::Item(item))
                            }
                            ElementWrapper::ListBoxActionItem(item) => {
                                Some(ListBoxItemWrapper::ActionItem(item))
                            }
                            _ => None,
                        }
                    })
                    .collect()
            })
            .iter()
    }
}

/// [`ListBoxItem`](self::item::ListBoxItem)과 [`ListBoxActionItem`](self::item::ListBoxActionItem)이 포함된 모듈
pub mod item;
