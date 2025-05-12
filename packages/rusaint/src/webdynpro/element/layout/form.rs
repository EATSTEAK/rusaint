use std::{borrow::Cow, cell::OnceCell, collections::HashMap};

use crate::webdynpro::error::WebDynproError;
use crate::webdynpro::event::Event;

use crate::webdynpro::element::{Interactable, macros::define_element_interactable};

define_element_interactable! {
    #[doc = "서버에 전송하기 위한 HTML Form"]
    Form<"FOR", "Form"> {
        data: OnceCell<FormData>
    },
    #[doc = "[`Form`]의 정의"]
    FormDef,
    #[doc = "[`Form`] 내부 데이터"]
    FormLSData {
        has_event_queue: bool => "0",
        response_data: String => "1",
        custom_data: String => "2",
    }
}

/// 서버 전송과 연관된 [`Form`] 데이터
#[derive(Debug, Default)]
#[allow(unused)]
pub struct FormData {
    name: Option<String>,
    method: Option<String>,
    action: Option<String>,
    title: Option<String>,
    accept: Option<String>,
    accept_charset: Option<String>,
    enctype: Option<String>,
    target: Option<String>,
}

impl<'a> Form<'a> {
    /// HTML 엘리먼트로부터 새로운 [`Form`] 엘리먼트를 생성합니다.
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
            data: OnceCell::new(),
        }
    }

    /// 폼 `submit`을 요청하는 이벤트를 반환합니다.
    pub fn request(
        &self,
        is_async: bool,
        focus_info: &str,
        hash: &str,
        dom_changed: bool,
        is_dirty: bool,
    ) -> Result<Event, WebDynproError> {
        let mut parameters: HashMap<String, String> = HashMap::new();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        parameters.insert("Async".to_string(), is_async.to_string());
        parameters.insert("FocusInfo".to_string(), focus_info.to_string());
        parameters.insert("Hash".to_string(), hash.to_string());
        parameters.insert("DomChanged".to_string(), dom_changed.to_string());
        parameters.insert("IsDirty".to_string(), is_dirty.to_string());
        self.fire_event("Request".to_string(), parameters)
    }
}
