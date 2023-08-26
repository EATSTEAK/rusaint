use derive_builder::Builder;

use super::{EVENT_DATA_START, EVENT_DATA_COLON, EVENT_DATA_COMMA, EVENT_DATA_END};

#[derive(Builder, Default, Clone)]
pub struct UcfParameters {
    action: Option<UcfAction>,
    enqueue_cardinality: Option<UcfCardinality>,
    response_data: Option<UcfResponseData>,
    transport_method: Option<UcfTransportMethod>,
    delay: Option<UcfDelay>
}

// TODO: Cleanup code
impl UcfParameters {
    pub fn serialize(&self) -> String {
        let mut owned = "".to_owned();
        owned.push_str(EVENT_DATA_START);
        if let Some(action) = &self.action {
            owned.push_str("ClientAction");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(action.to_string());
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(enqueue_cardinality) = &self.enqueue_cardinality {
            owned.push_str("EnqueueCardinality");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(enqueue_cardinality.to_string());
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(response_data) = &self.response_data {
            owned.push_str("ResponseData");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(response_data.to_string());
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(transport_method) = &self.transport_method {
            owned.push_str("TransportMethod");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(transport_method.to_string());
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(delay) = &self.delay {
            owned.push_str("Delay");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(delay.to_string());
            owned.push_str(EVENT_DATA_COMMA);
        }
        if owned.ends_with(EVENT_DATA_COMMA) { owned.truncate(owned.len() - 5) };
        owned.push_str(EVENT_DATA_END);
        owned
    }
}

#[derive(Clone)]
pub enum UcfAction {
    Submit,
    SubmitAsync,
    Enqueue,
    None
}

impl UcfAction {
    fn to_string(&self) -> &str {
        match &self {
            Self::Submit => "submit",
            Self::SubmitAsync => "submit_async",
            Self::Enqueue => "enqueue",
            _ => "none"
        }
    }
}

#[derive(Clone)]
pub enum UcfCardinality {
    Multiple,
    Single,
    None
}

impl UcfCardinality {
    fn to_string(&self) -> &str {
        match &self {
            Self::Multiple => "multiple",
            Self::Single => "single",
            _ => "none"
        }
    }
}

#[derive(Clone)]
pub enum UcfResponseData {
    Full,
    Delta,
    Inherit
}

impl UcfResponseData {
    fn to_string(&self) -> &str {
        match &self {
            Self::Full => "full",
            Self::Delta => "delta",
            Self::Inherit => "inherit"
        }
    }
}

#[derive(Clone)]
pub enum UcfTransportMethod {
    Full,
    Partial
}

impl UcfTransportMethod {
    fn to_string(&self) -> &str {
        match &self {
            Self::Full => "full",
            Self::Partial => "partial"
        }
    }
}

#[derive(Clone)]
pub enum UcfDelay {
    Full,
    None
}

impl UcfDelay {
    fn to_string(&self) -> &str {
        match &self {
            Self::Full => "full",
            _ => "none"
        }
    }
}