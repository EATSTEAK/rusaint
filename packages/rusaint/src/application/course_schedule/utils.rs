use crate::define_elements;
use crate::webdynpro::client::{EventProcessResult, WebDynproClient};
use crate::webdynpro::command::WebDynproCommandExecutor;
use crate::webdynpro::command::element::action::ButtonPressEventCommand;
use crate::webdynpro::command::element::layout::TabStripTabSelectEventCommand;
use crate::webdynpro::command::element::selection::{
    ComboBoxChangeEventCommand, ComboBoxItemListBoxCommand, ComboBoxSelectByValue1EventCommand,
    ListBoxItemInfoCommand,
};
use crate::webdynpro::element::action::ButtonDef;
use crate::webdynpro::element::layout::TabStrip;
use crate::webdynpro::element::layout::tab_strip::item::TabStripItemDef;
use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::element::selection::ComboBoxDef;
use crate::webdynpro::element::selection::list_box::item::ListBoxItemInfo;
use crate::webdynpro::error::WebDynproError;

define_elements! {
    TABSTRIP: TabStrip<'static> = "ZCMW2100.ID_0001:VIW_MAIN.MODULE_TABSTRIP";
}

#[allow(clippy::too_many_arguments)]
pub(super) async fn request_lv3(
    client: &mut WebDynproClient,
    tab_item: TabStripItemDef,
    tab_index: u32,
    lv1: ComboBoxDef,
    lv2: ComboBoxDef,
    lv3: ComboBoxDef,
    search_btn: ButtonDef,
    value_lv1: &str,
    value_lv2: &str,
    value_lv3: &str,
) -> Result<(), WebDynproError> {
    select_tab(client, tab_item, tab_index).await?;
    let lv1_event = ElementParser::new(client.body()).read(
        ComboBoxSelectByValue1EventCommand::new(lv1, value_lv1, false),
    )?;
    client.process_event(false, lv1_event).await?;
    let lv2_event = ElementParser::new(client.body()).read(
        ComboBoxSelectByValue1EventCommand::new(lv2, value_lv2, false),
    )?;
    client.process_event(false, lv2_event).await?;
    let parser = ElementParser::new(client.body());
    let lv3_event = parser.read(ComboBoxSelectByValue1EventCommand::new(
        lv3, value_lv3, false,
    ))?;
    client.process_event(false, lv3_event).await?;
    let btn_press = parser.read(ButtonPressEventCommand::new(search_btn))?;
    client.process_event(false, btn_press).await?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(super) async fn request_lv2(
    client: &mut WebDynproClient,
    tab_item: TabStripItemDef,
    tab_index: u32,
    lv1: ComboBoxDef,
    lv2: ComboBoxDef,
    search_btn: ButtonDef,
    value_lv1: &str,
    value_lv2: &str,
) -> Result<(), WebDynproError> {
    select_tab(client, tab_item, tab_index).await?;
    let lv1_event = ElementParser::new(client.body()).read(
        ComboBoxSelectByValue1EventCommand::new(lv1, value_lv1, false),
    )?;
    client.process_event(false, lv1_event).await?;
    let parser = ElementParser::new(client.body());
    let lv2_event = parser.read(ComboBoxSelectByValue1EventCommand::new(
        lv2, value_lv2, false,
    ))?;
    client.process_event(false, lv2_event).await?;
    let btn_press = parser.read(ButtonPressEventCommand::new(search_btn))?;
    client.process_event(false, btn_press).await?;
    Ok(())
}

pub(super) async fn select_lv2(
    client: &mut WebDynproClient,
    lv1: ComboBoxDef,
    lv2: ComboBoxDef,
    value_lv1: &str,
    value_lv2: &str,
) -> Result<(), WebDynproError> {
    let parser = ElementParser::new(client.body());
    let lv1_event = parser.read(ComboBoxSelectByValue1EventCommand::new(
        lv1, value_lv1, false,
    ))?;
    client.process_event(false, lv1_event).await?;
    let parser = ElementParser::new(client.body());
    let lv2_event = parser.read(ComboBoxSelectByValue1EventCommand::new(
        lv2, value_lv2, false,
    ))?;
    client.process_event(false, lv2_event).await?;
    Ok(())
}

pub(super) async fn request_lv1(
    client: &mut WebDynproClient,
    tab_item: TabStripItemDef,
    tab_index: u32,
    lv1: ComboBoxDef,
    search_btn: ButtonDef,
    value_lv1: &str,
) -> Result<(), WebDynproError> {
    select_tab(client, tab_item, tab_index).await?;
    let parser = ElementParser::new(client.body());
    let lv1_event = parser.read(ComboBoxSelectByValue1EventCommand::new(
        lv1, value_lv1, false,
    ))?;
    client.process_event(false, lv1_event).await?;
    let btn_press = parser.read(ButtonPressEventCommand::new(search_btn))?;
    client.process_event(false, btn_press).await?;
    Ok(())
}

pub(super) async fn select_lv1(
    client: &mut WebDynproClient,
    lv1: ComboBoxDef,
    value_lv1: &str,
) -> Result<(), WebDynproError> {
    let parser = ElementParser::new(client.body());
    let lv1_event = parser.read(ComboBoxSelectByValue1EventCommand::new(
        lv1, value_lv1, false,
    ))?;
    client.process_event(false, lv1_event).await?;
    Ok(())
}

pub(super) async fn request_text(
    client: &mut WebDynproClient,
    tab_item: TabStripItemDef,
    tab_index: u32,
    text_combo: ComboBoxDef,
    search_btn: ButtonDef,
    value: &str,
) -> Result<(), WebDynproError> {
    select_tab(client, tab_item, tab_index).await?;
    let parser = ElementParser::new(client.body());
    let change = parser.read(ComboBoxChangeEventCommand::new(text_combo, value, false))?;
    client.process_event(false, change).await?;
    let btn_press = parser.read(ButtonPressEventCommand::new(search_btn))?;
    client.process_event(false, btn_press).await?;
    Ok(())
}

pub(super) async fn request(
    client: &mut WebDynproClient,
    tab_item: TabStripItemDef,
    tab_index: u32,
    search_btn: ButtonDef,
) -> Result<(), WebDynproError> {
    select_tab(client, tab_item, tab_index).await?;
    let btn_press =
        ElementParser::new(client.body()).read(ButtonPressEventCommand::new(search_btn))?;
    client.process_event(false, btn_press).await?;
    Ok(())
}

pub(super) async fn select_tab(
    client: &mut WebDynproClient,
    tab_item: TabStripItemDef,
    tab_index: u32,
) -> Result<EventProcessResult, WebDynproError> {
    let tab_select = ElementParser::new(client.body()).read(TabStripTabSelectEventCommand::new(
        TABSTRIP, tab_item, tab_index, 0,
    ))?;
    client.process_event(false, tab_select).await
}

pub(super) fn combo_box_items(
    client: &mut WebDynproClient,
    combo_box: ComboBoxDef,
) -> Result<Vec<String>, WebDynproError> {
    let parser = ElementParser::new(client.body());
    let item_box = parser.read(ComboBoxItemListBoxCommand::new(combo_box))?;
    parser
        .read(ListBoxItemInfoCommand::new(item_box))
        .map(list_box_values)
}

pub(super) fn list_box_values(vec: Vec<ListBoxItemInfo>) -> Vec<String> {
    vec.iter()
        .map(|info| match info {
            ListBoxItemInfo::Item { value1, .. } => value1.clone(),
            ListBoxItemInfo::ActionItem { title, .. } => title.clone(),
        })
        .collect()
}
