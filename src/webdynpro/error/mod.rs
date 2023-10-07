use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebDynproError {
    #[error("Error in client request/response: {0}")]
    ClientError(#[from] ClientError),
    #[error("Error in parsing document body: {0}")]
    BodyError(#[from] BodyError),
    #[error("Error in updating document body from server response: {0}")]
    BodyUpdateError(#[from] BodyUpdateError),
    #[error("Error in parse or construct event of element: {0}")]
    ElementError(#[from] ElementError),
}

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
    #[error("Given base url is not valid: {0}")]
    InvalidBaseUrl(String),
    #[error("No form {0} found in desired application")]
    NoSuchForm(String),
    #[error("No cookie found: {0}")]
    NoSuchCookie(String),
    #[error("Empty cookie store for given url: {0}")]
    NoCookies(String),
}

#[derive(Error, Debug)]
pub enum BodyUpdateError {
    #[error("Failed to parse update document: {0}")]
    Parse(#[from] roxmltree::Error),
    #[error("Cannot find a node from given document: {0}")]
    NoSuchNode(String),
    #[error("Cannot find an attribute {attribute:?} from a node {node:?}")]
    NoSuchAttribute { node: String, attribute: String },
    #[error("{0} has no content")]
    NoSuchContent(String),
    #[error("Unknown element found: {0}")]
    UnknownElement(String),
    #[error("Failed to rewrite body document: {0}")]
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
    #[error("Invalid element")]
    InvalidElement,
    #[error("Cannot find element from document: {0}")]
    NoSuchElement(String),
    #[error("Cannot find attribute: {0}")]
    NoSuchAttribute(String),
    #[error("Cannot parse event str/struct: {0}")]
    CannotParseEvents(#[from] EventStrUnescapeError)
}

#[derive(Error, Debug)]
pub enum ElementError {
    #[error("Cannot find data {field} in element: {element}")]
    NoSuchData { element: String, field: String },
    #[error("Cannot fire event {event} in element: {element}")]
    NoSuchEvent { element: String, event: String },
    #[error("Cannot find content {content} in element: {element}")]
    NoSuchContent { element: String, content: String },
    #[error("Invalid id {0}")]
    InvalidId(String),
    #[error("Invalid content {content} in element: {element}")]
    InvalidContent { element: String, content: String },
    #[error("Element {0} has invalid lsdata attribute")]
    InvalidLSData(String),
    #[error("Failed parse lsdata json-like object")]
    ParseLSDataFailed(#[from] serde_json::Error),
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
