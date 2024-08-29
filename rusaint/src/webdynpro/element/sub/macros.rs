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
      }

      impl $def_name {
          /// 서브 엘리먼트의 정의를 생성합니다.
          pub const fn new(parent: $parent_def, id: &'static str) -> Self {
              Self {
                  id: std::borrow::Cow::Borrowed(id),
                  parent,
              }
          }
      }

      impl<'body> $crate::webdynpro::element::sub::definition::SubElementDefinition<'body> for $def_name {

          type Parent = $parent<'body>;

          type SubElement = $name<'body>;

          fn new_dynamic(parent: <Self::Parent as $crate::webdynpro::element::Element<'body>>::Def, id: String) -> Self {
              Self {
                  id: id.into(),
                  parent,
                  node_id: None
              }
          }

          fn id(&self) -> &str {
              &self.id
          }

          fn id_cow(&self) -> Cow<'static, str> {
              self.id.clone()
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
          tag: tl::HTMLTag<'a>,
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

          fn from_tag(
              element_def: &impl $crate::webdynpro::element::sub::definition::SubElementDefinition<'a>,
              tag: tl::HTMLTag<'a>,
          ) -> Result<Self, $crate::webdynpro::error::WebDynproError> {
              Ok(Self::new($crate::webdynpro::element::sub::definition::SubElementDefinition::id_cow(element_def), tag))
          }

          fn id(&self) -> &str {
              &self.id
          }

          fn tag(&self) -> &tl::HTMLTag<'a> {
              &self.tag
          }
      }

      $crate::webdynpro::element::macros::define_lsdata! {
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
