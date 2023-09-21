use serde::Deserialize;

struct ListBox {

}

struct ListBoxPopup(ListBox);
struct ListBoxPopupJson(ListBox);
struct ListBoxPopupFiltered(ListBox);
struct ListBoxPopupJsonFiltered(ListBox);
struct ListBoxMultiple(ListBox);
struct ListBoxSingle(ListBox);

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
struct ListBoxLSData {
    #[serde(rename = "0")]
    visible_items: Option<i32>,
    #[serde(rename = "1")]
    height: Option<String>,
    #[serde(rename = "2")]
    icon_visibility: Option<String>,
    #[serde(rename = "3")]
    first_value_visibility: Option<String>,
    #[serde(rename = "4")]
    second_value_visibility: Option<String>,
    #[serde(rename = "5")]
    available: Option<bool>,
    #[serde(rename = "6")]
    server_filter: Option<String>,
    #[serde(rename = "7")]
    complete: Option<bool>,
    #[serde(rename = "8")]
    filtered: Option<bool>,
    #[serde(rename = "9")]
    table_data_definition: Option<String>,
    #[serde(rename = "10")]
    item_table_data: Option<String>,
    #[serde(rename = "11")]
    history_table_data: Option<String>,
    #[serde(rename = "12")]
    custom_data: Option<String>,
    #[serde(rename = "13")]
    custom_style: Option<String>,
    #[serde(rename = "14")]
    table_data_item_design: Option<String>,
}

mod item {
    use serde::Deserialize;

    struct ListBoxItem {

    }

    #[derive(Deserialize, Debug, Default)]
    #[allow(unused)]
    struct ListBoxItemLSData {
        #[serde(rename = "0")]
        icon_src: Option<String>,
        #[serde(rename = "1")]
        disabled_icon_src: Option<String>,
        #[serde(rename = "2")]
        semantic_text_color: Option<String>,
        #[serde(rename = "3")]
        is_deletable: Option<bool>,
        #[serde(rename = "4")]
        custom_data: Option<String>,
    }

    #[allow(unused)]
    struct ListBoxItemData {
        index: Option<String>,
        key: Option<String>,
        disabled: Option<bool>,
        tooltip: Option<String>,
        icon_tooltip: Option<String>,
        group_title: Option<String>,
        value_1: Option<String>,
        value_2: Option<String>
    }

    struct ListBoxActionItem {
        
    }

    #[derive(Deserialize, Debug, Default)]
    #[allow(unused)]
    struct ListBoxActionItemLSData {
        #[serde(rename = "0")]
        custom_data: Option<String>,
    }
}