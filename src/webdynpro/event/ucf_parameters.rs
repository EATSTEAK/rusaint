use derive_builder::Builder;

#[derive(Builder, Default, Clone)]
pub struct UcfParameters {
    action: Option<UcfAction>,
    enqueue_cardinality: Option<UcfCardinality>,
    response_data: Option<UcfResponseData>,
    transport_method: Option<UcfTransportMethod>,
    delay: Option<UcfDelay>
}

#[derive(Clone)]
pub enum UcfAction {
    Submit,
    SubmitAsync,
    Enqueue,
    None
}

#[derive(Clone)]
pub enum UcfCardinality {
    Multiple,
    Single,
    None
}

#[derive(Clone)]
pub enum UcfResponseData {
    Full,
    Delta,
    Inherit
}

#[derive(Clone)]
pub enum UcfTransportMethod {
    Full,
    Partial
}

#[derive(Clone)]
pub enum UcfDelay {
    Full,
    None
}