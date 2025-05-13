use crate::webdynpro::client::body::Body;
use crate::webdynpro::element::sub::SubElement;
use crate::webdynpro::element::sub::definition::SubElementDefinition;
use crate::webdynpro::element::{Element, definition::ElementDefinition};
use crate::webdynpro::error::{ElementError, WebDynproError};
use scraper::Html;

/// DOM에서 엘리먼트를 파싱하기 위한 파서
pub struct ElementParser(Html);

// This is safe since `ElementParser` is immutable.
unsafe impl Send for ElementParser {}

unsafe impl Sync for ElementParser {}

impl<'s> ElementParser {
    /// [`Body`]로부터 새로운 파서를 만듭니다.
    pub fn new(body: &Body) -> Self {
        let document = Html::parse_document(body.raw_body());
        Self(document)
    }

    /// [`ElementDefinition`]을 구현하는 값에서 알맞는 [`Element`]를 만듭니다.
    pub fn element_from_def<T: ElementDefinition<'s>>(
        &'s self,
        definition: &T,
    ) -> Result<T::Element, WebDynproError> {
        let selector = definition.selector()?;
        let element_ref = self
            .0
            .select(&selector)
            .next()
            .ok_or(ElementError::InvalidId(definition.id().to_string()))?;
        Element::from_ref(definition, element_ref)
    }

    /// [`SubElementDefinition`]을 구현하는 값에서 알맞는 [`SubElement`]를 만듭니다.
    pub fn subelement_from_def<T: SubElementDefinition<'s>>(
        &'s self,
        definition: &T,
    ) -> Result<T::SubElement, WebDynproError> {
        let selector = definition.selector()?;
        let element_ref = self
            .0
            .select(&selector)
            .next()
            .ok_or(ElementError::InvalidId(definition.id().to_string()))?;
        SubElement::from_ref(definition, element_ref)
    }

    /// 파서 내의 [`Html`]을 반환합니다.
    pub(crate) fn document(&'s self) -> &'s Html {
        &self.0
    }
}
