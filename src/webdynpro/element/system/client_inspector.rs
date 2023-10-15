use std::{borrow::Cow, cell::OnceCell, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::webdynpro::error::WebDynproError;
use crate::webdynpro::event::Event;

use crate::webdynpro::element::{
    Element, ElementDef, ElementWrapper, EventParameterMap, Interactable,
};

/// 클라이언트의 변경 사항을 감시
/// 
/// 이 엘리먼트는 사용자와 직접 상호작용하지 않는 특별한 엘리먼트로, 브라우저의 여러 값(윈도우 높이 등)을 감시하고 변경 사항이 있다면
/// 이를 서버에 알려주어 서버가 해당 값을 토대로 SSR을 할 수 있도록 도와줍니다.
/// rusaint에서는 서버에 가상의 기본 값을 알려주어 이를 토대로 SSR를 수행하도록 [`USaintApplication`]에서 구현하고 있습니다.
/// 3개 정도의 `ClientInspector`가 최초에 초기화되며, 초기화 된 후에 [`LoadingPlaceholder`]를 통한 실제 페이지 로드를 수행합니다.
/// [`ClientInspectorLSData`]의 `notification_trigger` 값을 확인하면 해당 엘리먼트가 변경 사항이 있을 때마다 감시하는지, 혹은 최초 한번만 서버에 알리는지 확인할 수 있습니다.
#[derive(Debug)]
pub struct ClientInspector<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<ClientInspectorLSData>,
    lsevents: OnceCell<Option<EventParameterMap>>,
}

/// [`ClientInspector`] 내부 데이터
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all(serialize = "PascalCase"))]
#[allow(unused)]
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

impl<'a> Element<'a> for ClientInspector<'a> {
    const CONTROL_ID: &'static str = "CI";

    const ELEMENT_NAME: &'static str = "ClientInspector";

    type ElementLSData = ClientInspectorLSData;

    fn lsdata(&self) -> &Self::ElementLSData {
        self.lsdata.get_or_init(|| {
            let Ok(lsdata_obj) = Self::lsdata_elem(self.element_ref) else {
                    return ClientInspectorLSData::default();
                };
            serde_json::from_value::<Self::ElementLSData>(lsdata_obj)
                .unwrap_or(ClientInspectorLSData::default())
        })
    }

    fn from_elem(
        elem_def: ElementDef<'a, Self>,
        element: scraper::ElementRef<'a>,
    ) -> Result<Self, WebDynproError> {
        Ok(Self::new(elem_def.id.to_owned(), element))
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn element_ref(&self) -> &scraper::ElementRef<'a> {
        &self.element_ref
    }

    fn wrap(self) -> ElementWrapper<'a> {
        ElementWrapper::ClientInspector(self)
    }

    fn children(&self) -> Vec<ElementWrapper<'a>> {
        Self::children_elem(self.element_ref().clone())
    }
}

impl<'a> Interactable<'a> for ClientInspector<'a> {
    fn lsevents(&self) -> Option<&EventParameterMap> {
        self.lsevents
            .get_or_init(|| Self::lsevents_elem(self.element_ref).ok())
            .as_ref()
    }
}

impl<'a> ClientInspector<'a> {
    /// HTML 엘리먼트로 [`ClientInspector`] 엘리먼트를 생성합니다.
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }

    /// 서버에 이 엘리먼트가 감시하는 클라이언트 값을 알리는 이벤트를 반환합니다.
    /// 데이터는 특수한 형태의 JSON-like 값으로, 현재는 rusaint에서 Serialization/Deserialization 을 지원하지 않습니다.
    pub fn notify(&self, data: &str) -> Result<Event, WebDynproError> {
        let mut parameters: HashMap<String, String> = HashMap::new();

        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("Data".to_string(), data.to_string());
        self.fire_event("Notify".to_string(), parameters)
    }
}
