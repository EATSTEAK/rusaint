use std::{borrow::Cow, cell::OnceCell, collections::HashMap};

use scraper::Selector;

use crate::webdynpro::{
    element::{define_element_interactable, Element, ElementDef, Interactable},
    error::{BodyError, WebDynproError},
    event::Event,
};

use self::item::TabStripItem;

type TabItems<'a> = Vec<ElementDef<'a, TabStripItem<'a>>>;

define_element_interactable! {
    // Note: This element renders as "TS_ie6" if >= IE6
    TabStrip<"TS_standards", "TabStrip"> {
        tab_items: OnceCell<Option<TabItems<'a>>>,
    },
    TabStripLSData {
        current_index: i32 => "0",
        height: String => "1",
        width: String => "2",
        accessibility_description: String => "3",
        visibility: String => "4",
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
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
            tab_items: OnceCell::new(),
        }
    }

    pub fn tab_items(&self) -> Option<&TabItems<'a>> {
        self.tab_items
            .get_or_init(|| {
                let items_selector =
                    Selector::parse(format!(r#"[ct="{}"]"#, TabStripItem::CONTROL_ID).as_str())
                        .or(Err(BodyError::InvalidSelector))
                        .ok()?;
                Some(
                    self.element_ref
                        .select(&items_selector)
                        .filter_map(|eref| {
                            let id = eref.value().id()?;
                            Some(ElementDef::<TabStripItem>::new_dynamic(id.to_owned()))
                        })
                        .collect(),
                )
            })
            .as_ref()
    }

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

pub mod item;
