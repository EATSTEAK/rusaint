use crate::webdynpro::{
    element::{macros::define_element_base, parser::ElementParser, ElementWrapper},
    error::{ElementError, WebDynproError},
};
use std::{borrow::Cow, cell::OnceCell};
use tl::Bytes;

/// [`ListBox`](crate::webdynpro::element::selection::list_box::ListBox)의 아이템을 위한 Wrapper
#[derive(Debug)]
pub enum ListBoxItemWrapper<'a> {
    /// [`ListBox`](crate::webdynpro::element::selection::list_box::ListBox)의 일반 아이템
    Item(ListBoxItem<'a>),
    /// 수행할 수 있는 액션이 포함된 [`ListBox`](crate::webdynpro::element::selection::list_box::ListBox) 아이템
    ActionItem(ListBoxActionItem<'a>),
}

/// [`ListBox`](crate::webdynpro::element::selection::list_box::ListBox)의 아이템의 정의 Wrapper
#[derive(Clone, Debug)]
pub enum ListBoxItemDefWrapper {
    /// 일반 아이템의 정의
    Item(ListBoxItemDef),
    /// 액션이 포함된 아이템의 정의
    ActionItem(ListBoxActionItemDef),
}

/// [`ListBoxItem`]의 정보
pub enum ListBoxItemInfo {
    /// 일반 [`ListBoxItem`]의 정보
    Item {
        /// 아이템의 인덱스(순서)
        index: String,
        /// 아이템의 키
        key: String,
        /// 아이템의 첫번째 값
        value1: String,
        /// 아이템의 두번째 값
        value2: String,
        /// 아이템의 선택 여부
        selected: bool,
        /// 아이템의 활성화 여부
        enabled: bool,
        /// 제목
        title: String,
    },
    /// [`ListBoxActionItem`]의 정보
    ActionItem {
        /// 제목
        title: String,
        /// 내부 문자열
        text: String,
    },
}

impl ListBoxItemInfo {
    pub(super) fn from_tag(
        tag: tl::HTMLTag,
        parser: &ElementParser,
    ) -> Result<ListBoxItemInfo, WebDynproError> {
        let element = ElementWrapper::from_tag(tag)?;
        match element {
            ElementWrapper::ListBoxItem(item) => {
                Ok(ListBoxItemInfo::Item {
                    index: item.index().unwrap_or("").to_string(),
                    key: item.key().unwrap_or("").to_string(),
                    value1: item.value1().unwrap_or("").to_string(),
                    value2: item.value2().unwrap_or("").to_string(),
                    selected: item.selected().unwrap_or(false),
                    enabled: item.enabled().unwrap_or(true),
                    title: item.title().to_string(),
                })
            },
            ElementWrapper::ListBoxActionItem(action_item) => Ok(ListBoxItemInfo::ActionItem {
                title: action_item.title().to_string(),
                text: action_item.text(parser).to_string(),
            }),
            _ => Err(ElementError::InvalidContent {
                element: "ListBox".to_string(),
                content: "ListBoxItem".to_string(),
            })?,
        }
    }
}

define_element_base! {
    #[doc = "[`ListBox`](crate::webdynpro::element::selection::list_box::ListBox)의 일반 아이템"]
    ListBoxItem<"LIB_I", "ListBoxItem"> {
        index: OnceCell<Option<String>>,
        key: OnceCell<Option<String>>,
        tooltip: OnceCell<Option<String>>,
        value1: OnceCell<Option<String>>,
        value2: OnceCell<Option<String>>,
        selected: OnceCell<Option<bool>>,
        icon_tooltip: OnceCell<Option<String>>,
        enabled: OnceCell<Option<bool>>,
        group_title: OnceCell<Option<String>>,
        title: OnceCell<String>,
    },
    #[doc = "[`ListBoxItem`]의 정의"]
    ListBoxItemDef,
    #[doc = "[`ListBoxItem`] 내부 데이터"]
    ListBoxItemLSData {
        icon_src: String => "0",
        disabled_icon_src: String => "1",
        semantic_text_color: String => "2",
        is_deletable: bool => "3",
        custom_data: String => "4",
    }
}

/* impl<'a> std::fmt::Debug for ListBoxItem<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListBoxItem")
            .field("id", &self.id())
            .field("lsdata", &self.lsdata())
            .field("index", &self.index())
            .field("key", &self.key())
            .field("tooltip", &self.tooltip())
            .field("value1", &self.value1())
            .field("value2", &self.value2())
            .field("selected", &self.selected())
            .field("icon_tooltip", &self.icon_tooltip())
            .field("enabled", &self.enabled())
            .field("group_title", &self.group_title())
            .field("title", &self.title())
            .finish()
    }
} */

impl<'a> ListBoxItem<'a> {
    /// HTML 엘리먼트로부터 새로운 [`ListBoxItem`]을 생성합니다.
    pub fn new(id: Cow<'static, str>, tag: tl::HTMLTag<'a>) -> Self {
        Self {
            id,
            tag,
            lsdata: OnceCell::new(),
            title: OnceCell::new(),
            index: OnceCell::new(),
            key: OnceCell::new(),
            tooltip: OnceCell::new(),
            value1: OnceCell::new(),
            value2: OnceCell::new(),
            selected: OnceCell::new(),
            icon_tooltip: OnceCell::new(),
            enabled: OnceCell::new(),
            group_title: OnceCell::new(),
        }
    }

    /// 인덱스를 반환합니다.
    pub fn index(&'a self) -> Option<&'a str> {
        self.index
            .get_or_init(|| {
                self.tag
                    .attributes()
                    .get("data-itemindex")
                    .flatten()
                    .and_then(Bytes::try_as_utf8_str)
                    .map(str::to_string)
            }).as_ref()
            .map(String::as_str)
    }

    /// 키를 반환합니다.
    pub fn key(&'a self) -> Option<&'a str> {
        self.key
            .get_or_init(|| {
                self.tag
                    .attributes()
                    .get("data-itemkey")
                    .flatten()
                    .and_then(Bytes::try_as_utf8_str)
                    .map(str::to_string)
            }).as_ref()
            .map(String::as_str)
    }

    /// 툴팁을 반환합니다.
    pub fn tooltip(&'a self) -> Option<&'a str> {
        self.tooltip
            .get_or_init(|| {
                self.tag
                    .attributes()
                    .get("data-itemtooltip")
                    .flatten()
                    .and_then(Bytes::try_as_utf8_str)
                    .map(str::to_string)
            }).as_ref()
            .map(String::as_str)
    }

    /// 첫번째 값을 반환합니다.
    /// 일반적으로 이 값이 페이지에 표시되는 값입니다.
    pub fn value1(&'a self) -> Option<&'a str> {
        self.value1
            .get_or_init(|| {
                self.tag
                    .attributes()
                    .get("data-itemvalue1")
                    .flatten()
                    .and_then(Bytes::try_as_utf8_str)
                    .map(str::to_string)
            }).as_ref()
            .map(String::as_str)
    }

    /// 두번째 값을 반환합니다.
    pub fn value2(&'a self) -> Option<&'a str> {
        self.value2
            .get_or_init(|| {
                self.tag
                    .attributes()
                    .get("data-itemvalue2")
                    .flatten()
                    .and_then(Bytes::try_as_utf8_str)
                    .map(str::to_string)
            }).as_ref()
            .map(String::as_str)
    }

    /// 선택 여부를 반환합니다.
    pub fn selected(&'a self) -> Option<bool> {
        self.selected
            .get_or_init(|| {
                self.tag
                    .attributes()
                    .get("aria-selected")
                    .flatten()
                    .and_then(Bytes::try_as_utf8_str)
                    .and_then(|str| str.parse::<bool>().ok())
            })
            .to_owned()
    }

    /// 아이콘의 툴팁을 반환합니다.
    pub fn icon_tooltip(&'a self) -> Option<&'a str> {
        self.icon_tooltip
            .get_or_init(|| {
                self.tag
                    .attributes()
                    .get("data-itemicontooltip")
                    .flatten()
                    .and_then(Bytes::try_as_utf8_str)
                    .map(str::to_string)
            }).as_ref()
            .map(String::as_str)
    }

    /// 활성화 여부를 반환합니다.
    pub fn enabled(&'a self) -> Option<bool> {
        self.enabled
            .get_or_init(|| {
                self.tag
                    .attributes()
                    .get("data-itemdisabled")
                    .flatten()
                    .and_then(Bytes::try_as_utf8_str)
                    .and_then(|str| str.parse::<bool>().ok().and_then(|b| Some(!b)))
            })
            .to_owned()
    }

    /// 아이템 그룹의 제목을 반환합니다.
    pub fn group_title(&'a self) -> Option<&'a str> {
        self.group_title
            .get_or_init(|| {
                self.tag
                    .attributes()
                    .get("data-itemgrouptitle")
                    .flatten()
                    .and_then(Bytes::try_as_utf8_str)
                    .map(str::to_string)
            }).as_ref()
            .map(String::as_str)
    }

    /// 아이템 제목을 반환합니다.
    pub fn title(&'a self) -> &'a str {
        self.title.get_or_init(|| {
            self.tag
                .attributes()
                .get("title")
                .flatten()
                .and_then(Bytes::try_as_utf8_str)
                .unwrap_or("")
                .to_string()
        })
    }
}

mod action_item;

pub use self::action_item::{ListBoxActionItem, ListBoxActionItemDef, ListBoxActionItemLSData};
