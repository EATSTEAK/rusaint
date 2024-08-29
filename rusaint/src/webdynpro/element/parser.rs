use crate::webdynpro::client::body::Body;
use crate::webdynpro::element::{definition::ElementDefinition, Element};
use crate::webdynpro::error::{BodyError, WebDynproError};
use tl::{VDom, VDomGuard};

pub struct ElementParser(VDomGuard);

impl<'s> ElementParser {
    pub fn new(body: &Body) -> Result<ElementParser, WebDynproError> {
        Ok(ElementParser(
            unsafe { tl::parse_owned(body.raw_body().to_owned(), Default::default()) }
                .or_else(|err| Err(WebDynproError::Body(BodyError::Parse)))?,
        ))
    }
    pub(crate) fn dom(&'s self) -> &'s VDom {
        self.0.get_ref()
    }

    pub fn element_from_def<T: ElementDefinition<'s>>(
        &'s self,
        element_def: &T,
    ) -> Result<T::Element, WebDynproError> {
        let handle = self.dom().get_element_by_id(element_def.id());
        let tag = handle
            .unwrap()
            .get(self.dom().parser())
            .unwrap()
            .as_tag()
            .unwrap();
        Element::from_tag(element_def, tag.clone())
    }
}
