use crate::webdynpro::element::{definition::ElementDefinition, Element};
use crate::webdynpro::error::{ElementError, WebDynproError};

pub struct ElementParser(scraper::Html);

impl ElementParser {
    pub fn element_from_def<'body, T: ElementDefinition<'body>>(
        &self,
        definition: &T,
    ) -> Result<T::Element, WebDynproError> {
        let selector = definition.selector()?;
        let element_ref = self
            .0
            .select(&selector)
            .next()
            .ok_or(ElementError::InvalidId(definition.id().to_string()))?;
        T::Element::from_ref(&definition, element_ref)
    }
}
