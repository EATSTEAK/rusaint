use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::define_element_base;

define_element_base! {
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

    pub fn index(&self) -> Option<&str> {
        self.index
            .get_or_init(|| self.element_ref.value().attr("data-itemindex"))
            .to_owned()
    }

    pub fn key(&self) -> Option<&str> {
        self.key
            .get_or_init(|| self.element_ref.value().attr("data-itemkey"))
            .to_owned()
    }

    pub fn tooltip(&self) -> Option<&str> {
        self.tooltip
            .get_or_init(|| self.element_ref.value().attr("data-itemtooltip"))
            .to_owned()
    }

    pub fn value1(&self) -> Option<&str> {
        self.value1
            .get_or_init(|| self.element_ref.value().attr("data-itemvalue1"))
            .to_owned()
    }

    pub fn value2(&self) -> Option<&str> {
        self.value2
            .get_or_init(|| self.element_ref.value().attr("data-itemvalue2"))
            .to_owned()
    }

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

    pub fn icon_tooltip(&self) -> Option<&str> {
        self.icon_tooltip
            .get_or_init(|| self.element_ref.value().attr("data-itemicontooltip"))
            .to_owned()
    }

    pub fn enabled(&self) -> Option<bool> {
        self.enabled
            .get_or_init(|| {
                self.element_ref
                    .value()
                    .attr("data-itemdisabled")
                    .and_then(|str| str.parse::<bool>().ok().and_then(|b| Some(!b)))
            })
            .to_owned()
    }

    pub fn group_title(&self) -> Option<&str> {
        self.group_title
            .get_or_init(|| self.element_ref.value().attr("data-itemgrouptitle"))
            .to_owned()
    }

    pub fn title(&self) -> &str {
        self.title
            .get_or_init(|| self.element_ref.value().attr("title").unwrap_or(""))
    }
}
