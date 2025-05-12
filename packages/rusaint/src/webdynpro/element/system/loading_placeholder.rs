use std::{borrow::Cow, cell::OnceCell, collections::HashMap};

use crate::webdynpro::error::WebDynproError;
use crate::webdynpro::event::Event;

use crate::webdynpro::element::{Interactable, macros::define_element_interactable};

define_element_interactable! {
    #[doc = "페이지가 로드되기 전 내부 컨텐츠가 로드될 위치의 자리 표시자"]
    #[doc = ""]
    #[doc = "이 엘리먼트는 최초 로드 전 컨텐츠가 로드될 위치를 표시하기 위한 엘리먼트입니다."]
    #[doc = "`LoadingPlaceholder.Load` 이벤트가 전송되면 사라지고, 이 엘리먼트가 있는 위치에 실제 페이지가 렌더링됩니다."]
    #[doc = ""]
    #[doc = "로드 이벤트가 전송되어 페이지가 렌더링되기 위해서는 [`Custom`] 및 [`ClientInspector`] 엘리먼트의 클라이언트 데이터가 전송되어야 합니다."]
    #[doc = ""]
    #[doc = "[`Custom`]: crate::webdynpro::element::system::Custom"]
    #[doc = "[`ClientInspector`]: crate::webdynpro::element::system::ClientInspector"]
    LoadingPlaceholder<"LP", "LoadingPlaceHolder"> {},
    #[doc = "[`LoadingPlaceholder`]의 정의"]
    LoadingPlaceholderDef,
    #[doc = "[`LoadingPlaceholder`] 내부 데이터"]
    LoadingPlaceholderLSData {
        id: String => "0",
        custom_data: String => "1",
    }
}

impl<'a> LoadingPlaceholder<'a> {
    /// HTML 엘리먼트로부터 새로운 [`LoadingPlaceholder`]를 생성합니다.
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }

    /// 페이지를 로드하기 위한 이벤트를 반환합니다.
    pub fn load(&self) -> Result<Event, WebDynproError> {
        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        self.fire_event("Load".to_string(), parameters)
    }
}
