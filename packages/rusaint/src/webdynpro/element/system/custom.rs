use std::{borrow::Cow, collections::HashMap};

use crate::webdynpro::{
    element::{ElementWrapper, definition::ElementDefinition},
    error::{BodyError, WebDynproError},
    event::{
        Event, EventBuilder,
        ucf_parameters::{UcfAction, UcfParametersBuilder, UcfResponseData},
    },
};

use crate::webdynpro::element::Element;

/// 페이지 최초 로드시 서버에 전송하는 클라이언트 정보 값입니다.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct CustomClientInfo {
    pub window_opener_exists: bool,
    pub client_url: String,
    pub client_width: u32,
    pub client_height: u32,
    pub document_domain: String,
    pub is_top_window: bool,
    pub parent_accessible: bool,
}

impl Default for CustomClientInfo {
    fn default() -> Self {
        Self {
            window_opener_exists: true,
            client_url: Default::default(),
            client_width: 1920,
            client_height: 1000,
            document_domain: Default::default(),
            is_top_window: true,
            parent_accessible: true,
        }
    }
}

impl CustomClientInfo {
    /// 주어진 값을 토대로 [`Custom`] 엘리먼트를 만듭니다.
    pub fn new(
        window_opener_exists: bool,
        client_url: &str,
        client_width: u32,
        client_height: u32,
        document_domain: &str,
        is_top_window: bool,
        parent_accessible: bool,
    ) -> CustomClientInfo {
        CustomClientInfo {
            window_opener_exists,
            client_url: client_url.to_owned(),
            client_width,
            client_height,
            document_domain: document_domain.to_string(),
            is_top_window,
            parent_accessible,
        }
    }
}

/// 클라이언트의 정보를 알리기 위해 사용하는 가상 엘리먼트
///
/// 이 엘리먼트는 실제로 페이지에 존재하지 않으며, 최초 페이지 로드 및 초기화 시 서버에 기본 렌더링을 위한 클라이언트 정보를 보내기 위한 가상 엘리먼트입니다.
/// 최초 한번 `Custom.ClientInfos` 이벤트를 전송하고 사용되지 않습니다.
#[derive(Debug)]
pub struct Custom {
    id: Cow<'static, str>,
}

#[doc = "[`Custom`]의 정의"]
#[derive(Clone, Debug)]
pub struct CustomDef {
    id: Cow<'static, str>,
}

impl CustomDef {
    /// 엘리먼트의 정의를 생성합니다.
    pub const fn new(id: &'static str) -> Self {
        Self {
            id: Cow::Borrowed(id),
        }
    }
}

impl ElementDefinition<'_> for CustomDef {
    type Element = Custom;

    fn new_dynamic(id: String) -> Self {
        Self { id: id.into() }
    }

    fn from_ref(element_ref: scraper::ElementRef<'_>) -> Result<Self, WebDynproError> {
        let id = element_ref.value().id().ok_or(BodyError::InvalidElement)?;
        Ok(Self {
            id: id.to_string().into(),
        })
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn id_cow(&self) -> Cow<'static, str> {
        self.id.clone()
    }
}

impl<'a> Element<'a> for Custom {
    // Note: This element is not rendered to client itself. This control id is a dummy.
    const CONTROL_ID: &'static str = "CUSTOM";

    const ELEMENT_NAME: &'static str = "Custom";

    type ElementLSData = ();

    type Def = CustomDef;

    fn from_ref(
        elem_def: &impl ElementDefinition<'a>,
        _element: scraper::ElementRef,
    ) -> Result<Self, WebDynproError> {
        Ok(Self::new(elem_def.id_cow()))
    }

    fn children(&self) -> Vec<ElementWrapper<'a>> {
        vec![]
    }

    fn lsdata(&self) -> &Self::ElementLSData {
        &()
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn element_ref(&self) -> &scraper::ElementRef<'a> {
        panic!("Element Custom is pseudo-element")
    }

    fn wrap(self) -> ElementWrapper<'a> {
        ElementWrapper::Custom(self)
    }
}

impl Custom {
    /// 식별자를 바탕으로 새로운 [`Custom`] 엘리먼트를 생성합니다.
    pub const fn new(id: Cow<'static, str>) -> Self {
        Self { id }
    }

    /// 서버에 클라이언트 정보를 보내는 이벤트를 반환합니다.
    pub fn client_infos(&self, infos: CustomClientInfo) -> Event {
        let mut parameters: HashMap<String, String> = HashMap::new();
        let ucf_params = UcfParametersBuilder::default()
            .action(Some(UcfAction::Enqueue))
            .response(Some(UcfResponseData::Delta))
            .build()
            .unwrap();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert(
            "WindowOpenerExists".to_string(),
            infos.window_opener_exists.to_string(),
        );
        parameters.insert("ClientURL".to_string(), infos.client_url);
        parameters.insert("ClientWidth".to_string(), infos.client_width.to_string());
        parameters.insert("ClientHeight".to_string(), infos.client_height.to_string());
        parameters.insert("DocumentDomain".to_string(), infos.document_domain);
        parameters.insert("IsTopWindow".to_string(), infos.is_top_window.to_string());
        parameters.insert(
            "ParentAccessible".to_string(),
            infos.parent_accessible.to_string(),
        );
        EventBuilder::default()
            .control("Custom".to_owned())
            .event("ClientInfos".to_owned())
            .parameters(parameters)
            .ucf_parameters(ucf_params)
            .build()
            .unwrap()
    }
}
