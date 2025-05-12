use std::{borrow::Cow, cell::OnceCell};

use crate::webdynpro::element::utils::{children_element, parse_lsdata, parse_lsevents};
use crate::webdynpro::error::{BodyError, WebDynproError};
use serde_json::Value;

use super::{Element, EventParameterMap, Interactable, definition::ElementDefinition};

// Type for unimplemented elements
/// rusaint에 구현되지 않은 엘리먼트를 위한 가상 엘리먼트
#[derive(Debug)]
pub struct Unknown<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    ct: OnceCell<Option<String>>,
    lsdata: OnceCell<Value>,
    lsevents: OnceCell<Option<EventParameterMap>>,
}

/// [`Unknown`]의 정의
#[derive(Clone, Debug)]
pub struct UnknownDef {
    id: Cow<'static, str>,
}

impl UnknownDef {
    /// 엘리먼트의 정의를 생성합니다.
    pub const fn new(id: &'static str) -> Self {
        Self {
            id: Cow::Borrowed(id),
        }
    }
}

impl<'body> ElementDefinition<'body> for UnknownDef {
    type Element = Unknown<'body>;

    fn new_dynamic(id: String) -> Self {
        Self { id: id.into() }
    }

    fn from_ref(element_ref: scraper::ElementRef<'_>) -> Result<Self, WebDynproError> {
        let id = element_ref.value().id().ok_or(BodyError::InvalidElement)?;
        Ok(Self {
            id: id.to_string().into(),
        })
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn id_cow(&self) -> Cow<'static, str> {
        self.id.clone()
    }
}

impl<'a> Element<'a> for Unknown<'a> {
    /// 실제로 사용하지 않는 가상의 Id
    const CONTROL_ID: &'static str = "_UNKNOWN";
    /// 실제로 사용하지 않는 가상의 이름
    const ELEMENT_NAME: &'static str = "Unknown";

    type ElementLSData = Value;

    type Def = UnknownDef;

    fn from_ref(
        elem_def: &impl ElementDefinition<'a>,
        element: scraper::ElementRef<'a>,
    ) -> Result<Self, WebDynproError> {
        Ok(Self::new(elem_def.id_cow(), element))
    }

    fn children(&self) -> Vec<super::ElementWrapper<'a>> {
        children_element(*self.element_ref())
    }

    fn lsdata(&self) -> &Self::ElementLSData {
        self.lsdata.get_or_init(|| {
            parse_lsdata(self.element_ref.value().attr("lsdata").unwrap_or("")).unwrap_or_default()
        })
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn element_ref(&self) -> &scraper::ElementRef<'a> {
        &self.element_ref
    }

    fn wrap(self) -> super::ElementWrapper<'a> {
        super::ElementWrapper::Unknown(self)
    }
}

impl<'a> Interactable<'a> for Unknown<'a> {
    fn lsevents(&self) -> Option<&EventParameterMap> {
        self.lsevents
            .get_or_init(|| parse_lsevents(self.element_ref.value().attr("lsevents")?).ok())
            .as_ref()
    }
}

impl<'a> Unknown<'a> {
    /// 엘리먼트를 생성합니다.
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            ct: OnceCell::new(),
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }

    /// 이 엘리먼트의 실제 엘리먼트 Id를 반환합니다.
    pub fn ct(&self) -> Option<&String> {
        self.ct
            .get_or_init(|| {
                self.element_ref
                    .value()
                    .attr("ct")
                    .map(|str| str.to_string())
            })
            .as_ref()
    }
}
