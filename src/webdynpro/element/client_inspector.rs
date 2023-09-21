use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::webdynpro::{
    application::client::body::Body,
    error::{BodyError, ElementError},
    event::Event,
};

use super::{Element, ElementDef, EventParameterMap};

pub struct ClientInspector<'a> {
    id: &'a str,
    lsdata: Option<ClientInspectorLSData>,
    lsevents: Option<EventParameterMap>,
}
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all(serialize = "PascalCase"))]
pub struct ClientInspectorLSData {
    #[serde(rename(deserialize = "0"), skip_serializing)]
    notification_trigger: Option<String>,
    #[serde(rename(deserialize = "1"))]
    client_width: Option<String>,
    #[serde(rename(deserialize = "2"))]
    client_height: Option<String>,
    #[serde(rename(deserialize = "3"))]
    screen_width: Option<String>,
    #[serde(rename(deserialize = "4"))]
    screen_height: Option<String>,
    #[serde(rename(deserialize = "5"))]
    screen_orientation: Option<String>,
    #[serde(rename(serialize = "QME", deserialize = "6"))]
    qme: Option<String>,
    #[serde(rename(deserialize = "7"))]
    rendering_mode_compatibility: Option<String>,
    #[serde(rename(serialize = "ThemeID", deserialize = "8"))]
    theme_id: Option<String>,
    #[serde(rename(serialize = "SapThemeID", deserialize = "9"))]
    sap_theme_id: Option<String>,
    #[serde(rename(deserialize = "10"))]
    theme_scope: Option<String>,
    #[serde(rename(deserialize = "11"))]
    themed_table_row_height: Option<String>,
    #[serde(rename(deserialize = "12"))]
    themed_form_layout_row_height: Option<String>,
    #[serde(rename(deserialize = "13"))]
    themed_scrollbar_dimension: Option<String>,
    #[serde(rename(deserialize = "14"))]
    themed_document_background_color: Option<String>,
    #[serde(rename(deserialize = "15"))]
    themed_svg_libs: Option<String>,
    #[serde(rename(deserialize = "16"))]
    themed_svg_lib_urls: Option<String>,
    #[serde(rename(deserialize = "17"))]
    themed_raster_height: Option<String>,
    #[serde(rename(deserialize = "18"))]
    themed_raster_width: Option<String>,
    #[serde(rename(deserialize = "19"))]
    themed_layout_padding_top: Option<String>,
    #[serde(rename(deserialize = "20"))]
    themed_layout_padding_left: Option<String>,
    #[serde(rename(deserialize = "21"))]
    themed_layout_padding_bottom: Option<String>,
    #[serde(rename(deserialize = "22"))]
    themed_layout_padding_right: Option<String>,
    #[serde(rename(deserialize = "23"))]
    themed_abap_list_raster_height: Option<String>,
    #[serde(rename(deserialize = "24"))]
    themed_abap_list_raster_width: Option<String>,
    #[serde(rename(deserialize = "25"))]
    themed_value_help_height: Option<String>,
    #[serde(rename(deserialize = "26"))]
    themed_value_help_width: Option<String>,
    #[serde(rename(deserialize = "27"))]
    theme_tags: Option<String>,
    #[serde(rename(deserialize = "28"))]
    word_enabled: Option<String>,
    #[serde(rename(deserialize = "29"))]
    excel_enabled: Option<String>,
    #[serde(rename(deserialize = "30"))]
    flash_enabled: Option<String>,
    #[serde(rename(deserialize = "31"))]
    acrobat_enabled: Option<String>,
    #[serde(rename(deserialize = "32"))]
    silverlight_enabled: Option<String>,
    #[serde(rename(deserialize = "33"))]
    java_enabled: Option<String>,
    #[serde(rename(deserialize = "34"))]
    java_version: Option<String>,
    #[serde(rename(deserialize = "35"))]
    web_sockets_enabled: Option<String>,
    #[serde(rename(deserialize = "36"))]
    device_type: Option<String>,
    #[serde(rename(deserialize = "37"))]
    css_matches_html_version: Option<String>,
    #[serde(rename(deserialize = "38"))]
    custom_data: Option<String>,
    #[serde(rename(deserialize = "39"))]
    platform: Option<String>,
    #[serde(rename(deserialize = "40"))]
    window_opener_exists: Option<String>,
    #[serde(rename(serialize = "ClientURL", deserialize = "41"))]
    client_url: Option<String>,
    #[serde(rename(deserialize = "42"))]
    document_domain: Option<String>,
    #[serde(rename(deserialize = "43"))]
    is_top_window: Option<String>,
    #[serde(rename(deserialize = "44"))]
    parent_accessible: Option<String>,
}

impl<'a> Element for ClientInspector<'a> {
    const CONTROL_ID: &'static str = "CI";

    const ELEMENT_NAME: &'static str = "ClientInspector";

    type ElementLSData = ClientInspectorLSData;

    fn lsdata(&self) -> Option<&Self::ElementLSData> {
        self.lsdata.as_ref()
    }

    fn lsevents(&self) -> Option<&EventParameterMap> {
        self.lsevents.as_ref()
    }
}

impl<'a> ElementDef<'a, ClientInspector<'a>> {
    pub fn elem(&self, body: &'_ Body) -> Result<ClientInspector<'a>, BodyError> {
        ClientInspector::from_body(self, body)
    }
}

impl<'a> ClientInspector<'a> {
    pub const fn new(
        id: &'a str,
        lsdata: Option<ClientInspectorLSData>,
        lsevents: Option<EventParameterMap>,
    ) -> Self {
        Self {
            id,
            lsdata,
            lsevents,
        }
    }

    pub fn from_body(elem_def: &ElementDef<'a, Self>, body: &'_ Body) -> Result<Self, BodyError> {
        let selector = &elem_def.selector().or(Err(BodyError::InvalidSelector))?;
        let lsdata_obj = Self::lsdata_elem(selector, body.document())?;
        let lsdata = serde_json::from_value::<ClientInspectorLSData>(lsdata_obj).or(Err(ElementError::InvalidLSData))?;
        let lsevents = Self::lsevents_elem(selector, body.document())?;
        Ok(Self::new(elem_def.id, Some(lsdata), Some(lsevents)))
    }

    pub fn notify(&self, data: &str) -> Result<Event, ElementError> {
        let mut parameters: IndexMap<String, String> = IndexMap::new();

        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("Data".to_string(), data.to_string());
        self.fire_event("Notify", parameters)
    }
}
