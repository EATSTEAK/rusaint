/// LSData를 쉽게 작성할 수 있도록 해주는 매크로
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

/// [`ElementDefinition`]을 작성하기 위한 매크로
macro_rules! define_element_definition {
    { $(#[$outer:meta])*
        $name: ident<$element_name: ident>
    } => {

        $(#[$outer])*
        #[derive(Clone, Debug)]
        pub struct $name {
            id: std::borrow::Cow<'static, str>
        }

        impl $name {
            /// 엘리먼트 정의를 생성합니다. 이 함수를 직접 실행하기보다는 [`define_elements`](crate::webdynpro::element::define_elements)매크로 사용을 추천합니다.
            pub const fn new(id: &'static str) -> Self {
                Self {
                    id: std::borrow::Cow::Borrowed(id)
                }
            }
        }

        impl<'body> $crate::webdynpro::element::definition::ElementDefinition<'body> for $name {
            type Element = $element_name<'body>;

            fn new_dynamic(id: String) -> Self {
                Self {
                    id: id.into()
                }
            }

            fn id(&self) -> &str {
                &self.id
            }

            fn id_cow(&self) -> Cow<'static, str> {
                self.id.clone()
            }
        }
    }
}

/// 일반적인 엘리먼트를 작성할 수 있도록 해주는 매크로
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

        impl<'a> $crate::webdynpro::element::Element<'a> for $name<'a> {
            const CONTROL_ID: &'static str = $controlid;

            const ELEMENT_NAME: &'static str = $element_name;

            type ElementLSData = $lsdata;

            type Def = $def_name;

            fn lsdata(&self) -> &Self::ElementLSData {
                self.lsdata
                    .get_or_init(|| {
                        let Ok(lsdata_obj) = $crate::webdynpro::element::lsdata_tag(self.tag()).or_else(|e| { eprintln!("{:?}", e); Err(e) }) else {
                            return $lsdata::default();
                        };
                        serde_json::from_value::<Self::ElementLSData>(lsdata_obj).or_else(|e| { eprintln!("{:?}", e); Err(e) }).ok().unwrap_or($lsdata::default())
                    })
            }

            fn from_tag(
                element_def: &impl $crate::webdynpro::element::definition::ElementDefinition<'a>,
                tag: tl::HTMLTag<'a>,
            ) -> Result<Self, $crate::webdynpro::error::WebDynproError> {
                Ok(Self::new($crate::webdynpro::element::definition::ElementDefinition::id_cow(element_def), tag))
            }

            fn id(&self) -> &str {
                &self.id
            }

            fn tag(&self) -> &tl::HTMLTag<'a> {
                &self.tag
            }

            fn wrap(self) -> $crate::webdynpro::element::ElementWrapper<'a> {
                $crate::webdynpro::element::ElementWrapper::$name(self)
            }

            fn children(&self, parser: &'a $crate::webdynpro::element::parser::ElementParser) -> Vec<$crate::webdynpro::element::ElementWrapper<'a>> {
                $crate::webdynpro::element::children_tag(self.tag(), parser)
            }
        }

        $crate::webdynpro::element::macros::define_element_definition! {
            $(#[$def_outer])*
            $def_name<$name>
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

/// `Interactable`한 엘리먼트를 작성할 수 있게 해주는 매크로
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
                    .get_or_init(|| $crate::webdynpro::element::lsevents_tag(&self.tag).ok())
                    .as_ref()
            }
        }
    }
}

pub(crate) use define_element_base;
pub(crate) use define_element_definition;
pub(crate) use define_element_interactable;
pub(crate) use define_lsdata;
