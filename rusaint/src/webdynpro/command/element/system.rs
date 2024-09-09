use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::event::Event;
use crate::webdynpro::{
    command::WebDynproCommand,
    element::system::{ClientInspectorDef, Custom, CustomClientInfo, LoadingPlaceholderDef},
    error::WebDynproError,
};

/// [`ClientInspector`](crate::webdynpro::element::system::ClientInspector)를 통해 서버에 클라이언트 정보를 전파하는 이벤트를 반환
pub struct ClientInspectorNotifyEventCommand {
    element_def: ClientInspectorDef,
    message: String,
}

impl ClientInspectorNotifyEventCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: ClientInspectorDef, message: &str) -> Self {
        Self {
            element_def,
            message: message.to_string(),
        }
    }
}

impl WebDynproCommand for ClientInspectorNotifyEventCommand {
    type Result = Event;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        parser
            .element_from_def(&self.element_def)?
            .notify(&self.message)
    }
}

/// [`LoadingPlaceholder`](crate::webdynpro::element::system::LoadingPlaceholder)를 로드하는 이벤트를 반환
pub struct LoadingPlaceholderLoadEventCommand {
    element_def: LoadingPlaceholderDef,
}

impl LoadingPlaceholderLoadEventCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: LoadingPlaceholderDef) -> Self {
        Self { element_def }
    }
}

impl WebDynproCommand for LoadingPlaceholderLoadEventCommand {
    type Result = Event;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        parser.element_from_def(&self.element_def)?.load()
    }
}

/// ClientInfo 명령을 커스텀 객체를 이용해 서버에 전송하는 이벤트를 반환
pub struct CustomClientInfoEventCommand {
    element: Custom,
    info: CustomClientInfo,
}

impl CustomClientInfoEventCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element: Custom, info: CustomClientInfo) -> Self {
        Self { element, info }
    }
}

impl WebDynproCommand for CustomClientInfoEventCommand {
    type Result = Event;

    fn dispatch(&self, _parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        Ok(self.element.client_infos(self.info.clone()))
    }
}
