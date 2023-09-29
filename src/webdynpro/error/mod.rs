use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Failed to request from web")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to parse HTML body")]
    Parse(#[from] BodyError),
    #[error("Request is made, but failed")]
    RequestFailed(reqwest::Response),
    #[error("Failed to parse base url")]
    BaseUrlParse(#[from] url::ParseError),
    #[error("Server's update response is invalid")]
    InvalidUpdate(#[from] BodyUpdateError),
    #[error("Given base url is not valid")]
    InvalidBaseUrl,
    #[error("No form found in desired application")]
    NoForm,
    #[error("No cookie found in client")]
    NoCookie,
    #[error("Tried to use invalid element")]
    InvalidElement(#[from] ElementError),
}

#[derive(Error, Debug)]
pub enum BodyUpdateError {
    #[error("Failed to parse update document")]
    Parse(#[from] roxmltree::Error),
    #[error("Cannot find a node from given document: {0}")]
    CannotFindNode(String),
    #[error("Cannot find an attribute {attribute:?} from a node {node:?}")]
    CannotFindAttribute { node: String, attribute: String },
    #[error("{0} has no content")]
    NoContent(String),
    #[error("Unknown element found: {0}")]
    UnknownElement(String),
    #[error("Failed to rewrite body document")]
    Rewrite(#[from] lol_html::errors::RewritingError),
}

#[derive(Error, Debug)]
pub enum BodyError {
    #[error("Failed to parse body document")]
    Parse,
    #[error("Given body document is invalid")]
    Invalid,
    #[error("Given selector for parsing body is invalid")]
    InvalidSelector,
    #[error("Element data is cannot be parsed")]
    InvalidElement(#[from] ElementError),
}

#[derive(Error, Debug)]
pub enum ElementError {
    #[error("Cannot find attribute {0}")]
    NoSuchAttribute(String),
    #[error("Invalid id {0}")]
    InvalidId(String),
    #[error("Element has invalid lsdata attribute")]
    InvalidLSData,
    #[error("Failed parse lsdata json-like object")]
    ParseLSDataFailed(#[from] serde_json::Error),
    #[error("Cannot find given element from document")]
    NoSuchElement,
    #[error("Cannot fire given event in this element")]
    NoSuchEvent,
}

#[derive(Error, Debug)]
pub enum EventStrUnescapeError {
    #[error("Failed read hex string")]
    Int(#[from] std::num::ParseIntError),
    #[error("hex string is not valid")]
    Parse(#[from] std::string::FromUtf16Error),
    #[error("No form found in desired application")]
    NoForm,
}
