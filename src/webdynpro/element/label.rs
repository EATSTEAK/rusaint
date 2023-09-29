use std::{borrow::Cow, cell::OnceCell};

use anyhow::Result;
use serde::Deserialize;
use crate::webdynpro::element::{EventParameterMap, Element, ElementDef, Elements};

// TODO: Implement additional events and data
#[derive(Debug)]
pub struct Label<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<Option<LabelLSData>>,
    lsevents: OnceCell<Option<EventParameterMap>>,
}
impl<'a> Element<'a> for Label<'a> {
    const CONTROL_ID: &'static str = "L";

    const ELEMENT_NAME: &'static str = "Label";

    type ElementLSData = LabelLSData;

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
        Elements::Label(self)
    }
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct LabelLSData {
    #[serde(rename = "0")]
    tooltip: Option<String>,
    #[serde(rename = "1")]
    label_for: Option<String>,
    #[serde(rename = "2")]
    wrapping: Option<bool>,
    #[serde(rename = "3")]
    text: Option<String>,
    #[serde(rename = "4")]
    required: Option<bool>,
    #[serde(rename = "5")]
    enabled: Option<bool>,
    #[serde(rename = "6")]
    design_bar: Option<String>,
    #[serde(rename = "7")]
    width: Option<String>,
    #[serde(rename = "8")]
    has_icon: Option<bool>,
    #[serde(rename = "9")]
    image_first: Option<bool>,
    #[serde(rename = "10")]
    visibility: Option<String>,
    #[serde(rename = "11")]
    show_help: Option<bool>,
    #[serde(rename = "12")]
    access_key: Option<String>,
    #[serde(rename = "13")]
    align: Option<String>,
    #[serde(rename = "14")]
    text_overflow: Option<bool>,
    #[serde(rename = "15")]
    required_indicator_at_front: Option<bool>,
    #[serde(rename = "16")]
    interaction_behavior: Option<String>,
    #[serde(rename = "17")]
    is_link: Option<bool>,
    #[serde(rename = "18")]
    editable: Option<bool>,
    #[serde(rename = "19")]
    custom_data: Option<String>,
    #[serde(rename = "20")]
    custom_style: Option<String>,
    #[serde(rename = "21")]
    height: Option<String>,
    #[serde(rename = "22")]
    labelled_by: Option<String>,
}

impl<'a> Label<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }
}