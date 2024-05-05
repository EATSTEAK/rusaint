use crate::webdynpro::{
    client::EventProcessResult,
    command::WebDynproCommand,
    element::{
        definition::ElementDefinition,
        layout::{tab_strip::item::TabStripItemDef, TabStripDef},
    },
    error::WebDynproError,
};

/// [`TabStrip`]의 탭을 선택하도록 함
pub struct TabStripTabSelectCommand {
    element_def: TabStripDef,
    item_id: String,
    item_index: u32,
    first_visible_item_index: u32,
}

impl<'a> TabStripTabSelectCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(
        element_def: TabStripDef,
        item: TabStripItemDef,
        item_index: u32,
        first_visible_item_index: u32,
    ) -> TabStripTabSelectCommand {
        Self {
            element_def,
            item_id: item.id().to_owned(),
            item_index,
            first_visible_item_index,
        }
    }
}

impl WebDynproCommand for TabStripTabSelectCommand {
    type Result = EventProcessResult;

    async fn dispatch(
        &self,
        client: &mut crate::webdynpro::client::WebDynproClient,
    ) -> Result<Self::Result, WebDynproError> {
        let event = self.element_def.from_body(client.body())?.tab_select(
            &self.item_id,
            self.item_index,
            self.first_visible_item_index,
        )?;
        client.process_event(false, event).await
    }
}
