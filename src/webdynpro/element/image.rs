use std::{borrow::Cow, cell::OnceCell};

use anyhow::Result;
use serde::Deserialize;
use crate::webdynpro::element::{EventParameterMap, Element, ElementDef, Elements};

// TODO: Implement additional events and data
#[derive(Debug)]
pub struct Image<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<ImageLSData>>,
    lsevents: OnceCell<Option<EventParameterMap>>,
}
impl<'a> Element<'a> for Image<'a> {
    const CONTROL_ID: &'static str = "IMG";

    const ELEMENT_NAME: &'static str = "Image";

    type ElementLSData = ImageLSData;

    fn lsdata(&self) -> Option<&Self::ElementLSData> {
        self.lsdata
            .get_or_init(|| {
                let lsdata_obj = Self::lsdata_elem(self.element_ref).ok()?;
                serde_json::from_value::<Self::ElementLSData>(lsdata_obj).ok()
            })
            .as_ref()
    }

    fn lsevents(&self) -> Option<&EventParameterMap> {
        self.lsevents
            .get_or_init(|| Self::lsevents_elem(self.element_ref).ok())
            .as_ref()
    }

    fn from_elem(elem_def: ElementDef<'a, Self>, element: scraper::ElementRef<'a>) -> Result<Self> {
        Ok(Self::new(elem_def.id.to_owned(), element))
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn element_ref(&self) -> &scraper::ElementRef<'a> {
        &self.element_ref
    }

    fn wrap(self) -> Elements<'a> {
        Elements::Image(self)
    }
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct ImageLSData {
    #[serde(rename = "0")]
    tooltip: Option<String>,
    #[serde(rename = "1")]
    width: Option<String>,
    #[serde(rename = "2")]
    height: Option<String>,
    #[serde(rename = "3")]
    src: Option<String>,
    #[serde(rename = "4")]
    is_interactive: Option<bool>,
    #[serde(rename = "5")]
    has_image_map: Option<bool>,
    #[serde(rename = "6")]
    visibility: Option<String>,
    #[serde(rename = "7")]
    is_nested: Option<bool>,
    #[serde(rename = "8")]
    label_text: Option<String>,
    #[serde(rename = "9")]
    adjust_image_size: Option<bool>,
    #[serde(rename = "10")]
    drag_source_info: Option<String>,
    #[serde(rename = "11")]
    is_drag_handle: Option<bool>,
    #[serde(rename = "12")]
    enabled: Option<bool>,
    #[serde(rename = "13")]
    error_image_src: Option<String>,
    #[serde(rename = "14")]
    custom_data: Option<String>,
    #[serde(rename = "15")]
    its_mode: Option<bool>,
    #[serde(rename = "16")]
    its_display_mode: Option<String>,
    #[serde(rename = "17")]
    custom_style: Option<String>,
    #[serde(rename = "18")]
    drop_target_info: Option<String>,
    #[serde(rename = "19")]
    vertical_text_align: Option<String>,
    #[serde(rename = "20")]
    horizontal_text_align: Option<String>,
    #[serde(rename = "21")]
    used_in_sap_table: Option<bool>,
    #[serde(rename = "22")]
    labelled_by: Option<String>,
}

impl<'a> Image<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }
}