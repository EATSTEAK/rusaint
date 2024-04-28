use std::{borrow::Cow, marker};

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
    pub(super) fn body_hash(&self) -> u64 {
        self.body_hash
    }

    pub(super) fn node_id(&self) -> ego_tree::NodeId {
        self.node_id
    }
}

/// 컴파일 타임에서도 생성할 수 있는 [`Element`] 정의
#[derive(Debug)]
pub struct ElementDef<'a, T>
where
    T: Element<'a>,
{
    id: Cow<'static, str>,
    node_id: Option<ElementNodeId>,
    _marker: marker::PhantomData<&'a T>,
}

impl<'a, T: Element<'a>> Clone for ElementDef<'a, T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            node_id: self.node_id.clone(),
            _marker: self._marker.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

impl<'a, T> ElementDef<'a, T>
where
    T: Element<'a>,
{
    /// 엘리먼트 정의를 생성합니다. 이 함수를 직접 실행하기보다는 [`define_elements`]매크로 사용을 추천합니다.
    /// ### 예시
    /// ```
    /// # use rusaint::webdynpro::element::{ElementDef, action::Button};
    /// const BUTTON: ElementDef<'_, Button<'_>> = ElementDef::new("TEST.BUTTON1");
    /// ```
    pub const fn new(id: &'static str) -> ElementDef<'a, T> {
        ElementDef {
            id: Cow::Borrowed(id),
            node_id: None,
            _marker: std::marker::PhantomData,
        }
    }

    /// 런타임에서 엘리먼트 정의를 생성합니다. 엘리먼트의 Id 등을 컴파일 타임에서 미리 알 수 없는 경우 유용합니다.
    /// ### 예시
    /// ```
    /// # use rusaint::webdynpro::element::{ ElementDef, action::Button };
    /// # fn get_dynamic_button() -> String { return "TEST.BUTTON1".to_string() }
    /// let runtime_string: String = get_dynamic_button();
    /// let button_def: ElementDef<'_, Button<'_>> = ElementDef::new_dynamic(runtime_string);
    /// ```
    pub fn new_dynamic(id: String) -> ElementDef<'a, T> {
        ElementDef {
            id: id.into(),
            node_id: None,
            _marker: std::marker::PhantomData,
        }
    }

    /// 빠른 엘리먼트 탐색을 위해 `ego_tree::NodeId`와 함께 엘리먼트 정의를 생성합니다.
    pub fn with_node_id(
        id: String,
        body_hash: u64,
        node_id: ego_tree::NodeId,
    ) -> ElementDef<'a, T> {
        ElementDef {
            id: id.into(),
            node_id: Some(ElementNodeId { body_hash, node_id }),
            _marker: std::marker::PhantomData,
        }
    }

    /// 엘리먼트의 Id를 반환합니다.
    pub fn id(&self) -> &str {
        &self.id
    }

    pub(crate) fn id_cow(&self) -> Cow<'static, str> {
        self.id.clone()
    }

    /// `scraper`에서 이 엘리먼트를 선택할 수 있는 CSS Selector를 반환합니다.
    /// ### 예시
    /// ```ignore
    /// let body = app.body();
    /// const BUTTON: ElementDef<'_, Button<'_>> = ElementDef::new("TEST.BUTTON1");
    /// let selector = BUTTON.selector().unwrap();
    /// let btn_elem = body.document().select(&selector).next().unwrap();
    /// let btn = ElementWrapper::dyn_elem(btn_elem).unwrap();
    /// if let ElementWrapper::Button(btn) = btn {
    ///     println!("It's button!");
    /// }
    /// ```
    pub fn selector(&self) -> Result<Selector, WebDynproError> {
        Ok(std::result::Result::or(
            Selector::parse(format!(r#"[id="{}"]"#, &self.id).as_str()),
            Err(BodyError::InvalidSelector),
        )?)
    }

    /// [`Body`]에서 엘리먼트를 불러옵니다.
    pub fn from_body(&self, body: &'a Body) -> Result<T, WebDynproError> {
        T::from_body(self, body)
    }

    /// `scraper::ElementRef`에서 엘리먼트를 불러옵니다.
    pub fn from_elem(&self, element: scraper::ElementRef<'a>) -> Result<T, WebDynproError> {
        T::from_elem(self, element)
    }

    pub(super) fn node_id(&self) -> Option<&ElementNodeId> {
        (&self.node_id).as_ref()
    }
}

/// [`SubElement`]의 정의에 관련된 모듈
pub mod sub;
