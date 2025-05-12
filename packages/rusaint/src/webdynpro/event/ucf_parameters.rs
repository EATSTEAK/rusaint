use derive_builder::Builder;
use serde::Deserialize;
use std::fmt::Display;

use super::{EVENT_DATA_COLON, EVENT_DATA_COMMA, EVENT_DATA_END, EVENT_DATA_START};

/// 이벤트의 특성을 정의하는 파라메터
#[allow(missing_docs)]
#[derive(Builder, Clone, Default, Debug, Deserialize)]
#[builder(default)]
pub struct UcfParameters {
    #[serde(rename = "ClientAction")]
    action: Option<UcfAction>,
    #[serde(rename = "EnqueueCardinality")]
    cardinality: Option<UcfCardinality>,
    #[serde(rename = "TransportMethod")]
    transport: Option<UcfTransportMethod>,
    #[serde(rename = "ResponseData")]
    response: Option<UcfResponseData>,
    #[serde(rename = "ActionUrl")]
    action_url: Option<String>,
    #[serde(rename = "PrepareScript")]
    prepare_script: Option<String>,
    #[serde(rename = "Delay")]
    delay: Option<UcfDelay>,
    #[serde(rename = "SyncExecution")]
    sync_execution: Option<bool>,
    #[serde(rename = "ClientListener")]
    client_listener: Option<String>,
}

impl Display for UcfParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut owned = "".to_owned();
        owned.push_str(EVENT_DATA_START);
        if let Some(action) = &self.action {
            owned.push_str("ClientAction");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(&action.to_string());
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(cardinality) = &self.cardinality {
            owned.push_str("EnqueueCardinality");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(&cardinality.to_string());
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(transport) = &self.transport {
            owned.push_str("TransportMethod");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(&transport.to_string());
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(response) = &self.response {
            owned.push_str("ResponseData");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(&response.to_string());
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(action_url) = &self.action_url {
            owned.push_str("ActionUrl");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(action_url);
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(prepare_script) = &self.prepare_script {
            owned.push_str("PrepareScript");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(prepare_script);
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(delay) = &self.delay {
            owned.push_str("Delay");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(&delay.to_string());
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(sync_execution) = &self.sync_execution {
            owned.push_str("SyncExecution");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(&sync_execution.to_string());
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(client_listener) = &self.client_listener {
            owned.push_str("ClientListener");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(client_listener);
            owned.push_str(EVENT_DATA_COMMA);
        }
        if owned.ends_with(EVENT_DATA_COMMA) {
            owned.truncate(owned.len() - 5)
        };
        owned.push_str(EVENT_DATA_END);
        write!(f, "{owned}")
    }
}

// TODO: Cleanup code
impl UcfParameters {
    /// [`UcfParameters`]값을 이벤트 웹 리퀘스트에 전송할 수 있는 형태의 문자열으로 변환합니다.
    pub fn serialize(&self) -> String {
        self.to_string()
    }

    /// 이 파라메터를 가진 이벤트를 큐에 저장할 수 있다면 참을 반환합니다.
    /// [`UcfAction`]값이 `Enqueue`이면 참입니다.
    pub fn is_enqueable(&self) -> bool {
        matches!(self.action, Some(UcfAction::Enqueue))
    }

    /// 이 파라메터를 가진 이벤트를 큐에 저장했을 때 바로 전송할 수 있다면 참을 반환합니다.
    /// [`UcfAction`]값이 `Submit`이거나 `SubmitAsync`일 경우 참을 반환합니다.
    pub fn is_submitable(&self) -> bool {
        matches!(
            self.action,
            Some(UcfAction::Submit) | Some(UcfAction::SubmitAsync)
        )
    }
}

/// 이벤트가 큐에 저장될지 바로 전송될지 여부
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UcfAction {
    /// 이벤트가 바로 전송되어야 함
    Submit,
    /// 이벤트가 비동기로 바로 전송되어야 함
    SubmitAsync,
    /// 이벤트가 큐에 저장되어야 함
    Enqueue,
    /// 없음
    None,
}

impl Display for UcfAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            Self::Submit => "submit",
            Self::SubmitAsync => "submit_async",
            Self::Enqueue => "enqueue",
            _ => "none",
        }
        .to_owned();
        write!(f, "{str}")
    }
}

/// 동일한 종류의 이벤트가 큐에 동시에 들어갈 수 있는지 여부
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UcfCardinality {
    /// 동일한 이벤트가 큐에 여러번 들어갈 수 있음
    Multiple,
    /// 동일한 이벤트가 큐에 한번만 들어갈 수 있음
    Single,
    /// 없음
    None,
}

impl Display for UcfCardinality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            Self::Multiple => "multiple",
            Self::Single => "single",
            _ => "none",
        }
        .to_owned();
        write!(f, "{str}")
    }
}

/// 이벤트의 응답 방법을 표현할 수 있는지 여부
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UcfResponseData {
    /// 이벤트로 인해 변경된 부분을 포함한 전체 컨텐츠를 응답
    Full,
    /// 응답으로 인해 변경된 부분만 응답
    Delta,
    /// 기본 응답 방식을 따름
    Inherit,
}

impl Display for UcfResponseData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            Self::Full => "full",
            Self::Delta => "delta",
            Self::Inherit => "inherit",
        }
        .to_owned();
        write!(f, "{str}")
    }
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UcfTransportMethod {
    Full,
    Partial,
}

impl Display for UcfTransportMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            Self::Full => "full",
            Self::Partial => "partial",
        }
        .to_owned();
        write!(f, "{str}")
    }
}

/// 이벤트의 반영 딜레이 여부
#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UcfDelay {
    Full,
    None,
}

impl Display for UcfDelay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            Self::Full => "full",
            _ => "none",
        }
        .to_owned();
        write!(f, "{str}")
    }
}
