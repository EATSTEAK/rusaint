use std::borrow::Cow;

use scraper::Selector;

use crate::webdynpro::{
    client::body::Body,
    element::{definition::ElementDefinition, sub::SubElement, Element},
    error::{ElementError, WebDynproError},
};

use super::ElementNodeId;

/// [`SapTable`](crate::webdynpro::element::complex::sap_table::SapTable)등에서 사용하는 [`SubElement`]의 정의
pub trait SubElementDefinition<'body>: Sized {
    /// 부모 [`Element`]
    type Parent: Element<'body>;

    /// 이 정의가 생성하는 [`SubElement`]
    type SubElement: SubElement<'body>;

    /// 런타임에서 서브 엘리먼트의 정의를 만듭니다.
    fn new_dynamic(parent: <Self::Parent as Element<'body>>::Def, id: String) -> Self;

    /// 빠른 엘리먼트 탐색을 위해 `ego_tree::NodeId`와 함께 서브 엘리먼트 정의를 생성합니다.
    fn with_node_id(
        id: String,
        parent: <Self::Parent as Element<'body>>::Def,
        body_hash: u64,
        node_id: ego_tree::NodeId,
    ) -> Self;

    /// [`scraper::ElementRef`]에서 엘리먼트 정의를 생성합니다.
    fn from_element_ref(
        parent: <Self::Parent as Element<'body>>::Def,
        element: scraper::ElementRef<'body>,
    ) -> Result<Self, WebDynproError>;

    /// 서브 엘리먼트의 Id를 반환합니다.
    fn id(&self) -> &str;

    /// [`Cow`]형태의 Id가 필요한 경우 사용합니다.
    fn id_cow(&self) -> Cow<'static, str>;

    /// [`Body`]상 엘리먼트 노드 Id가 포함되었을 경우 이를 반환합니다.
    fn node_id(&self) -> Option<&ElementNodeId>;

    /// 이 [`SubElement`]의 부모 [`Element`]의 정의를 반환합니다.
    fn parent(&self) -> &<Self::Parent as Element<'body>>::Def;

    /// 서브 엘리먼트의 CSS Selector를 반환합니다.
    fn selector(&self) -> Result<Selector, WebDynproError> {
        Selector::parse(format!(r#"[id="{}"] [id="{}"]"#, self.parent().id(), self.id()).as_str())
            .or_else(|e| {
                println!("{e:?}");
                Err(ElementError::InvalidId(format!(
                    "{}, {}",
                    self.parent().id(),
                    self.id()
                )))?
            })
    }

    /// [`Body`]에서 서브 엘리먼트를 가져옵니다.
    fn from_body(&self, body: &'body Body) -> Result<Self::SubElement, WebDynproError> {
        Self::SubElement::from_body(self, body)
    }

    /// [`scraper::ElementRef`]에서 서브 엘리먼트를 가져옵니다.
    fn from_element(
        &self,
        element: scraper::ElementRef<'body>,
    ) -> Result<Self::SubElement, WebDynproError> {
        Self::SubElement::from_subelement(self, element)
    }
}
