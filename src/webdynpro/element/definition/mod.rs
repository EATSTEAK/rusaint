use std::borrow::Cow;

use scraper::Selector;

use crate::webdynpro::{
    client::body::Body,
    error::{BodyError, WebDynproError},
};

use super::Element;

/// 엘리먼트의 정의로부터 빠른 엘리먼트 탐색을 위한 DOM Tree 노드 Id
#[derive(Clone, Debug)]
pub struct ElementNodeId {
    body_hash: u64,
    node_id: ego_tree::NodeId,
}

impl ElementNodeId {
    pub(super) fn new(body_hash: u64, node_id: ego_tree::NodeId) -> Self {
        Self { body_hash, node_id }
    }

    pub(super) fn body_hash(&self) -> u64 {
        self.body_hash
    }

    pub(super) fn node_id(&self) -> ego_tree::NodeId {
        self.node_id
    }
}

/// 컴파일 타임에서도 생성할 수 있는 [`Element`] 정의
pub trait ElementDefinition<'body>: Sized {
    /// 해당 정의가 생성할 수 있는 엘리먼트
    type Element: Element<'body>;

    /// 런타임에서 엘리먼트 정의를 생성합니다. 엘리먼트의 Id 등을 컴파일 타임에서 미리 알 수 없는 경우 유용합니다.
    /// ### 예시
    /// ```
    /// # use rusaint::webdynpro::element::{ action::ButtonDef, definition::ElementDefinition };
    /// # fn get_dynamic_button() -> String { return "TEST.BUTTON1".to_string() }
    /// let runtime_string: String = get_dynamic_button();
    /// let button_def: ButtonDef = ButtonDef::new_dynamic(runtime_string);
    /// ```
    fn new_dynamic(id: String) -> Self;

    /// [`scraper::ElementRef`]에서 엘리먼트 정의를 생성합니다.
    fn from_element_ref(element_ref: scraper::ElementRef<'_>) -> Result<Self, WebDynproError>;

    /// 빠른 엘리먼트 탐색을 위해 `ego_tree::NodeId`와 함께 엘리먼트 정의를 생성합니다.
    fn with_node_id(id: String, body_hash: u64, node_id: ego_tree::NodeId) -> Self;

    /// 엘리먼트의 Id를 반환합니다.
    fn id(&self) -> &str;

    /// [`Cow`]형태의 Id가 필요한 경우 사용합니다.
    fn id_cow(&self) -> Cow<'static, str>;

    /// [`Body`]상 엘리먼트 노드 Id가 포함되었을 경우 이를 반환합니다.
    fn node_id(&self) -> Option<&ElementNodeId>;

    /// `scraper`에서 이 엘리먼트를 선택할 수 있는 CSS Selector를 반환합니다.
    /// ### 예시
    /// ```ignore
    /// let body = app.body();
    /// const BUTTON: ElementDef<'_, Button<'_>> = ElementDef::new("TEST.BUTTON1");
    /// let selector = BUTTON.selector().unwrap();
    /// let btn_elem = body.document().select(&selector).next().unwrap();
    /// let btn = ElementWrapper::dyn_element(btn_elem).unwrap();
    /// if let ElementWrapper::Button(btn) = btn {
    ///     println!("It's button!");
    /// }
    /// ```
    fn selector(&self) -> Result<Selector, WebDynproError> {
        Ok(
            Selector::parse(format!(r#"[id="{}"]"#, self.id()).as_str()).or_else(|err| {
                eprintln!("{err:?}");
                Err(BodyError::InvalidSelector)
            })?,
        )
    }

    /// [`Body`]에서 엘리먼트를 불러옵니다.
    fn from_body(&self, body: &'body Body) -> Result<Self::Element, WebDynproError> {
        Self::Element::from_body(self, body)
    }

    /// `scraper::ElementRef`에서 엘리먼트를 불러옵니다.
    fn from_element(
        &self,
        element_ref: scraper::ElementRef<'body>,
    ) -> Result<Self::Element, WebDynproError> {
        Self::Element::from_element(self, element_ref)
    }
}

/// [`SubElement`]의 정의에 관련된 모듈
pub mod sub;
