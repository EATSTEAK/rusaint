use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::{
    element::{ElementWrapper, macros::define_element_base},
    error::{ElementError, WebDynproError},
};

/// [`ListBox`](crate::webdynpro::element::selection::list_box::ListBox)의 아이템을 위한 Wrapper
#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
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
    pub(super) fn from_element_ref(
        element_ref: scraper::ElementRef<'_>,
    ) -> Result<ListBoxItemInfo, WebDynproError> {
        let element = ElementWrapper::from_ref(element_ref)?;
        match element {
            ElementWrapper::ListBoxItem(item) => Ok(ListBoxItemInfo::Item {
                index: item.index().unwrap_or("").to_string(),
                key: item.key().unwrap_or("").to_string(),
                value1: item.value1().unwrap_or("").to_string(),
                value2: item.value2().unwrap_or("").to_string(),
                selected: item.selected().unwrap_or(false),
                enabled: item.enabled().unwrap_or(true),
                title: item.title().to_string(),
            }),
            ElementWrapper::ListBoxActionItem(action_item) => Ok(ListBoxItemInfo::ActionItem {
                title: action_item.title().to_string(),
                text: action_item.text().to_string(),
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
        index: OnceCell<Option<&'a str>>,
        key: OnceCell<Option<&'a str>>,
        tooltip: OnceCell<Option<&'a str>>,
        value1: OnceCell<Option<&'a str>>,
        value2: OnceCell<Option<&'a str>>,
        selected: OnceCell<Option<bool>>,
        icon_tooltip: OnceCell<Option<&'a str>>,
        enabled: OnceCell<Option<bool>>,
        group_title: OnceCell<Option<&'a str>>,
        title: OnceCell<&'a str>,
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
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
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
    pub fn index(&self) -> Option<&str> {
        self.index
            .get_or_init(|| self.element_ref.value().attr("data-itemindex"))
            .to_owned()
    }

    /// 키를 반환합니다.
    pub fn key(&self) -> Option<&str> {
        self.key
            .get_or_init(|| self.element_ref.value().attr("data-itemkey"))
            .to_owned()
    }

    /// 툴팁을 반환합니다.
    pub fn tooltip(&self) -> Option<&str> {
        self.tooltip
            .get_or_init(|| self.element_ref.value().attr("data-itemtooltip"))
            .to_owned()
    }

    /// 첫번째 값을 반환합니다.
    /// 일반적으로 이 값이 페이지에 표시되는 값입니다.
    pub fn value1(&self) -> Option<&str> {
        self.value1
            .get_or_init(|| self.element_ref.value().attr("data-itemvalue1"))
            .to_owned()
    }

    /// 두번째 값을 반환합니다.
    pub fn value2(&self) -> Option<&str> {
        self.value2
            .get_or_init(|| self.element_ref.value().attr("data-itemvalue2"))
            .to_owned()
    }

    /// 선택 여부를 반환합니다.
    pub fn selected(&self) -> Option<bool> {
        self.selected
            .get_or_init(|| {
                self.element_ref
                    .value()
                    .attr("aria-selected")
                    .and_then(|str| str.parse::<bool>().ok())
            })
            .to_owned()
    }

    /// 아이콘의 툴팁을 반환합니다.
    pub fn icon_tooltip(&self) -> Option<&str> {
        self.icon_tooltip
            .get_or_init(|| self.element_ref.value().attr("data-itemicontooltip"))
            .to_owned()
    }

    /// 활성화 여부를 반환합니다.
    pub fn enabled(&self) -> Option<bool> {
        self.enabled
            .get_or_init(|| {
                self.element_ref
                    .value()
                    .attr("data-itemdisabled")
                    .and_then(|str| str.parse::<bool>().ok().map(|b| !b))
            })
            .to_owned()
    }

    /// 아이템 그룹의 제목을 반환합니다.
    pub fn group_title(&self) -> Option<&str> {
        self.group_title
            .get_or_init(|| self.element_ref.value().attr("data-itemgrouptitle"))
            .to_owned()
    }

    /// 아이템 제목을 반환합니다.
    pub fn title(&self) -> &str {
        self.title
            .get_or_init(|| self.element_ref.value().attr("title").unwrap_or(""))
    }
}

mod action_item;

pub use self::action_item::{ListBoxActionItem, ListBoxActionItemDef, ListBoxActionItemLSData};
