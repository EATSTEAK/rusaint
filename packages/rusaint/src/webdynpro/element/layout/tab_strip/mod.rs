use std::{borrow::Cow, cell::OnceCell, collections::HashMap};

use scraper::Selector;

use crate::webdynpro::{
    element::{
        Element, Interactable, definition::ElementDefinition, macros::define_element_interactable,
        property::Visibility,
    },
    error::{BodyError, WebDynproError},
    event::Event,
};

use self::item::TabStripItem;

define_element_interactable! {
    // Note: This element renders as "TS_ie6" if >= IE6
    #[doc = "상단 버튼으로 선택할 수 있는 탭 레이아웃"]
    #[doc = ""]
    #[doc = "> |**참고**| 이 엘리먼트는 실제 구현에서 >= IE6 용 구현과 기본 구현으로 나누어져 있지만, rusaint에서는 최신의 브라우저를 기준으로 하므로 전자의 구현은 구현되어있지 않습니다."]
    TabStrip<"TS_standards", "TabStrip"> {
        tab_items: OnceCell<Vec<<TabStripItem<'a> as Element<'a>>::Def>>,
    },
    #[doc = "[`TabStrip`]의 정의"]
    TabStripDef,
    #[doc = "[`TabStrip`] 내부 데이터"]
    TabStripLSData {
        current_index: i32 => "0",
        height: String => "1",
        width: String => "2",
        accessibility_description: String => "3",
        visibility: Visibility => "4",
        first_visible_item_idx: i32 => "5",
        scrollable: bool => "6",
        exact_tab_alignment: bool => "7",
        client_tab_select: bool => "8",
        drag_source_info: String => "9",
        drop_target_info: String => "10",
        tab_items_position: String => "11",
        custom_data: String => "12",
        custom_style: String => "13",
        tab_items_design: String => "14",
        heading_level: i32 => "15",
    }
}

impl<'a> TabStrip<'a> {
    /// HTML 엘리먼트로부터 새로운 [`TabStrip`] 엘리먼트를 생성합니다.
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
            tab_items: OnceCell::new(),
        }
    }

    /// 탭 내부 [`TabStripItem`]의 정의를 반환합니다.
    pub fn tab_items(
        &self,
    ) -> impl ExactSizeIterator<Item = &<TabStripItem<'a> as Element<'a>>::Def> {
        self.tab_items
            .get_or_init(|| {
                let Ok(items_selector) =
                    Selector::parse(format!(r#"[ct="{}"]"#, TabStripItem::CONTROL_ID).as_str())
                        .or(Err(BodyError::InvalidSelector))
                else {
                    return vec![];
                };
                self.element_ref
                    .select(&items_selector)
                    .filter_map(|eref| {
                        let id = eref.value().id()?;
                        Some(<TabStripItem<'a> as Element<'a>>::Def::new_dynamic(
                            id.to_owned(),
                        ))
                    })
                    .collect()
            })
            .iter()
    }

    /// 특정 탭을 선택하는 이벤트를 반환합니다.
    pub fn tab_select(
        &self,
        item_id: &str,
        item_index: u32,
        first_visible_item_index: u32,
    ) -> Result<Event, WebDynproError> {
        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("ItemId".to_string(), item_id.to_string());
        parameters.insert("ItemIndex".to_string(), item_index.to_string());
        parameters.insert(
            "FirstVisibleItemIndex".to_string(),
            first_visible_item_index.to_string(),
        );
        self.fire_event("TabSelect".to_string(), parameters)
    }
}

/// [`TabStrip`] 내부 아이템
pub mod item;
