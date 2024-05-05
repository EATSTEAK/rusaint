macro_rules! define_subelement {
  {   $(#[$outer:meta])*
      $name:ident<$parent:ident, $parent_def:ty, $controlid:literal, $element_name:literal> {
          $($sfield:ident : $stype:ty),* $(,)?
      },
      $(#[$def_outer:meta])*
      $def_name:ident,
      $(#[$lsdata_outer:meta])*
      $lsdata:ident {
          $(
              $(#[$lsdata_inner:meta])*
              $field:ident: $ftype:ty => $encoded:literal
          ),* $(,)?
      }
  } => {

      $(#[$def_outer])*
      #[derive(Clone, Debug)]
      pub struct $def_name {
          id: std::borrow::Cow<'static, str>,
          parent: $parent_def,
          node_id: Option<crate::webdynpro::element::definition::ElementNodeId>
      }

      impl $def_name {
          /// 서브 엘리먼트의 정의를 생성합니다.
          pub const fn new(parent: $parent_def, id: &'static str) -> Self {
              Self {
                  id: std::borrow::Cow::Borrowed(id),
                  parent,
                  node_id: None
              }
          }
      }

      impl<'body> $crate::webdynpro::element::definition::sub::SubElementDefinition<'body> for $def_name {

          type Parent = $parent<'body>;

          type SubElement = $name<'body>;

          fn new_dynamic(parent: <Self::Parent as $crate::webdynpro::element::Element<'body>>::Def, id: String) -> Self {
              Self {
                  id: id.into(),
                  parent,
                  node_id: None
              }
          }

          fn from_element_ref(parent: <Self::Parent as $crate::webdynpro::element::Element<'body>>::Def, element_ref: scraper::ElementRef<'_>) -> Result<Self, $crate::webdynpro::error::WebDynproError> {
              let id = element_ref.value().id().ok_or($crate::webdynpro::error::BodyError::InvalidElement)?;
              Ok(Self {
                  id: id.to_string().into(),
                  parent,
                  node_id: None
              })
          }

          fn with_node_id(id: String, parent: <Self::Parent as $crate::webdynpro::element::Element<'body>>::Def, body_hash: u64, node_id: ego_tree::NodeId) -> Self {
              Self {
                  id: id.into(),
                  parent,
                  node_id: Some($crate::webdynpro::element::definition::ElementNodeId::new(body_hash, node_id))
              }
          }

          fn id(&self) -> &str {
              &self.id
          }

          fn id_cow(&self) -> Cow<'static, str> {
              self.id.clone()
          }

          fn node_id(&self) -> Option<&$crate::webdynpro::element::definition::ElementNodeId> {
              (&self.node_id).as_ref()
          }

          fn parent(&self) -> &<Self::Parent as $crate::webdynpro::element::Element<'body>>::Def {
            &self.parent
          }
      }

      $(#[$outer])*
      #[derive(custom_debug_derive::Debug)]
      #[allow(unused)]
      pub struct $name<'a> {
          id: std::borrow::Cow<'static, str>,
          #[debug(skip)]
          element_ref: scraper::ElementRef<'a>,
          lsdata: std::cell::OnceCell<$lsdata>,
          $($sfield: $stype, )*
      }

      impl<'a> $crate::webdynpro::element::sub::SubElement<'a> for $name<'a> {
          const SUBCONTROL_ID: &'static str = $controlid;

          const ELEMENT_NAME: &'static str = $element_name;

          type SubElementLSData = $lsdata;

          type Def = $def_name;

          fn lsdata(&self) -> &Self::SubElementLSData {
              self.lsdata
                  .get_or_init(|| {
                      let Ok(lsdata_obj) = Self::lsdata_element(self.element_ref) else {
                          return $lsdata::default();
                      };
                      serde_json::from_value::<Self::SubElementLSData>(lsdata_obj).ok().unwrap_or($lsdata::default())
                  })
          }

          fn from_subelement(
              element_def: &impl $crate::webdynpro::element::definition::sub::SubElementDefinition<'a>,
              element: scraper::ElementRef<'a>,
          ) -> Result<Self, $crate::webdynpro::error::WebDynproError> {
              Ok(Self::new($crate::webdynpro::element::definition::sub::SubElementDefinition::id_cow(element_def), element))
          }

          fn id(&self) -> &str {
              &self.id
          }

          fn element_ref(&self) -> &scraper::ElementRef<'a> {
              &self.element_ref
          }
      }

      $crate::webdynpro::element::define_lsdata! {
          $(#[$lsdata_outer])*
          $lsdata {
              $(
                  $(#[$lsdata_inner])*
                  $field : $ftype => $encoded,
              )+
          }
      }
  };
}

pub(crate) use define_subelement;
use scraper::ElementRef;
use serde_json::Value;

use crate::webdynpro::{
    client::body::Body,
    error::{ElementError, WebDynproError},
};

use super::{definition::sub::SubElementDefinition, normalize_lsjson};

/// 서브 엘리먼트의 기능
pub trait SubElement<'a>: Sized {
    /// WebDynpro 내부에서 사용하는 서브 엘리먼트의 Id
    const SUBCONTROL_ID: &'static str;
    /// WebDynpro 내부에서 사용하는 서브 엘리먼트의 이름
    const ELEMENT_NAME: &'static str;
    /// 서브 엘리먼트의 LSData
    type SubElementLSData;
    /// 서브 엘리먼트의 정의
    type Def: SubElementDefinition<'a>;

    /// 서브 엘리먼트의 LSData를 JSON 객체 형태로 반환합니다.
    fn lsdata_element(element: scraper::ElementRef) -> Result<Value, WebDynproError> {
        let raw_data = element
            .value()
            .attr("lsdata")
            .ok_or(ElementError::InvalidLSData(
                element.value().id().unwrap().to_string(),
            ))?;
        let normalized = normalize_lsjson(raw_data);
        return Ok(
            serde_json::from_str(&normalized).or(Err(ElementError::InvalidLSData(
                element.value().id().unwrap().to_string(),
            )))?,
        );
    }

    /// 서브 엘리먼트의 정의와 [`Body`]로부터 서브 엘리먼트를 가져옵니다.
    fn from_body(
        elem_def: &impl SubElementDefinition<'a>,
        body: &'a Body,
    ) -> Result<Self, WebDynproError> {
        let selector = &elem_def.selector()?;
        let element = body
            .document()
            .select(selector)
            .next()
            .ok_or(ElementError::InvalidId(elem_def.id().to_owned()))?;
        Self::from_subelement(elem_def, element)
    }

    /// 서브 엘리먼트 정의와[] `scraper::ElementRef`]로부터 서브 엘리먼트를 가져옵니다.
    fn from_subelement(
        elem_def: &impl SubElementDefinition<'a>,
        element: scraper::ElementRef<'a>,
    ) -> Result<Self, WebDynproError>;

    /// 서브 엘리먼트의 LSData를 가져옵니다.
    fn lsdata(&self) -> &Self::SubElementLSData;

    /// 서브 엘리먼트의 Id를 가져옵니다.
    fn id(&self) -> &str;

    /// 서브 엘리먼트의 [`scraper::ElementRef`]를 가져옵니다.
    fn element_ref(&self) -> &ElementRef<'a>;
}
