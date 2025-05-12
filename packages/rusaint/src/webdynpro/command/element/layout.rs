use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::event::Event;
use crate::webdynpro::{
    command::WebDynproCommand,
    element::{
        definition::ElementDefinition,
        layout::{TabStripDef, tab_strip::item::TabStripItemDef},
    },
    error::WebDynproError,
};

/// [`TabStrip`](crate::webdynpro::element::layout::TabStrip)의 탭을 선택하도록 하는 이벤트를 반환
pub struct TabStripTabSelectEventCommand {
    element_def: TabStripDef,
    item_id: String,
    item_index: u32,
    first_visible_item_index: u32,
}

impl TabStripTabSelectEventCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(
        element_def: TabStripDef,
        item: TabStripItemDef,
        item_index: u32,
        first_visible_item_index: u32,
    ) -> TabStripTabSelectEventCommand {
        Self {
            element_def,
            item_id: item.id().to_owned(),
            item_index,
            first_visible_item_index,
        }
    }
}

impl WebDynproCommand for TabStripTabSelectEventCommand {
    type Result = Event;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        parser.element_from_def(&self.element_def)?.tab_select(
            &self.item_id,
            self.item_index,
            self.first_visible_item_index,
        )
    }
}
