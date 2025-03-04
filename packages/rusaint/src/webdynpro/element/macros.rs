macro_rules! define_lsdata {
    {   $(#[$lsdata_outer:meta])*
        $lsdata:ident {
            $(
                $(#[$lsdata_inner:meta])*
                $field:ident: $ftype:ty => $encoded:literal
            ),* $(,)?
        }
    } => {
        $(#[$lsdata_outer])*
        #[derive(Clone, serde::Deserialize, Debug, Default)]
        #[allow(unused)]
        pub struct $lsdata {
            $(
                $(#[$lsdata_inner])*
                #[serde(rename = $encoded)]
                $field: Option<$ftype>,
            )*
        }

        #[allow(missing_docs)]
        impl $lsdata {
            $(
                pub fn $field(&self) -> Option<&$ftype> {
                    (&self.$field).as_ref()
                }
            )*
        }
    }
}

macro_rules! define_element_base {
    {   $(#[$outer:meta])*
        $name:ident<$controlid:literal, $element_name:literal> {
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
            id: std::borrow::Cow<'static, str>
        }

        impl $def_name {
            /// 엘리먼트 정의를 생성합니다. 이 함수를 직접 실행하기보다는 [`define_elements`](crate::define_elements)매크로 사용을 추천합니다.
            pub const fn new(id: &'static str) -> Self {
                Self {
                    id: std::borrow::Cow::Borrowed(id)
                }
            }
        }

        impl<'body> $crate::webdynpro::element::definition::ElementDefinition<'body> for $def_name {
            type Element = $name<'body>;

            fn new_dynamic(id: String) -> Self {
                Self {
                    id: id.into()
                }
            }

            fn from_ref(element_ref: scraper::ElementRef<'_>) -> Result<Self, $crate::webdynpro::error::WebDynproError> {
                let id = element_ref.value().id().ok_or($crate::webdynpro::error::BodyError::InvalidElement)?;
                Ok(Self {
                    id: id.to_string().into()
                })
            }

            fn id(&self) -> &str {
                &self.id
            }

            fn id_cow(&self) -> Cow<'static, str> {
                self.id.clone()
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

        impl<'a> $crate::webdynpro::element::Element<'a> for $name<'a> {
            const CONTROL_ID: &'static str = $controlid;

            const ELEMENT_NAME: &'static str = $element_name;

            type ElementLSData = $lsdata;

            type Def = $def_name;

            fn lsdata(&self) -> &Self::ElementLSData {
                self.lsdata
                    .get_or_init(|| {
                        let lsdata_attr = self.element_ref.value().attr("lsdata").unwrap_or("");
                        let Ok(lsdata_obj) = $crate::webdynpro::element::utils::parse_lsdata(lsdata_attr).or_else(|e| { log::warn!(e:?; "failed to parse lsdata"); Err(e) }) else {
                            return $lsdata::default();
                        };
                        serde_json::from_value::<Self::ElementLSData>(lsdata_obj).or_else(|e| { log::warn!(e:?; "failed to convert lsdata to struct"); Err(e) }).ok().unwrap_or($lsdata::default())
                    })
            }

            fn from_ref(
                element_def: &impl $crate::webdynpro::element::definition::ElementDefinition<'a>,
                element: scraper::ElementRef<'a>,
            ) -> Result<Self, $crate::webdynpro::error::WebDynproError> {
                Ok(Self::new($crate::webdynpro::element::definition::ElementDefinition::id_cow(element_def), element))
            }

            fn id(&self) -> &str {
                &self.id
            }

            fn element_ref(&self) -> &scraper::ElementRef<'a> {
                &self.element_ref
            }

            fn wrap(self) -> $crate::webdynpro::element::ElementWrapper<'a> {
                $crate::webdynpro::element::ElementWrapper::$name(self)
            }

            fn children(&self) -> Vec<$crate::webdynpro::element::ElementWrapper<'a>> {
                $crate::webdynpro::element::utils::children_element(self.element_ref().clone())
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

macro_rules! define_element_interactable {
    {
        $(#[$outer:meta])*
        $name:ident<$controlid:literal, $element_name:literal> {
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
        $crate::webdynpro::element::macros::define_element_base!{
            $(#[$outer])*
            $name<$controlid, $element_name> {
                lsevents: std::cell::OnceCell<Option<$crate::webdynpro::element::EventParameterMap>>,
                $($sfield : $stype, )*
            },
            $(#[$def_outer])*
            $def_name,
            $(#[$lsdata_outer])*
            $lsdata {
                $(
                    $(#[$lsdata_inner])*
                    $field: $ftype => $encoded,
                )*
            }
        }

        impl<'a> $crate::webdynpro::element::Interactable<'a> for $name<'a> {
            fn lsevents(&self) -> Option<&$crate::webdynpro::element::EventParameterMap> {
                self.lsevents
                    .get_or_init(|| {
                        let lsevents_attr = self.element_ref.value().attr("lsevents").unwrap_or("");
                        $crate::webdynpro::element::utils::parse_lsevents(lsevents_attr).or_else(|e| { log::warn!(e:?; "failed to parse lsevents"); Err(e) }).ok()
                    })
                    .as_ref()
            }
        }
    }
}

pub(crate) use define_element_base;
pub(crate) use define_element_interactable;
pub(crate) use define_lsdata;
