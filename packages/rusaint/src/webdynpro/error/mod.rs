use thiserror::Error;

/// WebDynpro 엔진에서 발생할 수 있는 오류의 이늄
#[derive(Error, Debug)]
pub enum WebDynproError {
    /// 클라이언트 요청/응답 오류
    #[error("Error in client request/response: {0}")]
    Client(#[from] ClientError),
    /// WebDynpro 페이지 파싱 오류
    #[error("Error in parsing document body: {0}")]
    Body(#[from] BodyError),
    /// WebDynpro 페이지 업데이트 오류
    #[error("Error in updating document body from server response: {0}")]
    UpdateBody(#[from] UpdateBodyError),
    /// WebDynpro 엘리먼트 조작 오류
    #[error("Error in parse or construct event of element: {0}")]
    Element(#[from] ElementError),
}

/// 클라이언트 요청/응답 과정에서 발생하는 오류의 이늄
#[derive(Error, Debug)]
pub enum ClientError {
    /// 웹 리퀘스트에 실패
    #[error("Failed to request from web")]
    FailedRequest(#[from] reqwest::Error),
    /// HTML 문서를 파싱하지 못함
    #[error("Failed to parse HTML body")]
    Parse(#[from] BodyError),
    /// 웹 리퀘스트는 성공하였으나, 응답이 올바르지 않음
    #[error("Request is made, but failed")]
    InvalidResponse(reqwest::Response),
    /// 클라이언트에서 사용하는 Base URL 파싱 실패
    #[error("Failed to parse base url")]
    ParseBaseUrl(#[from] url::ParseError),
    /// WebDynpro 문서 업데이트 응답이 올바르지 않음
    #[error("Server's update response is invalid")]
    InvalidUpdate(#[from] UpdateBodyError),
    /// Base URL이 올바르지 않음
    #[error("Given base url is not valid: {0}")]
    InvalidBaseUrl(String),
    /// 클라이언트 요청에 필요한 폼을 찾을 수 없음
    #[error("No form {0} found in desired application")]
    NoSuchForm(String),
    /// 요청에 필요한 쿠키를 찾을 수 없음
    #[error("No cookie found: {0}")]
    NoSuchCookie(String),
    /// 주어진 Url에 대해 어떤 쿠키도 찾을 수 없음
    #[error("Empty cookie store for given url: {0}")]
    NoCookies(String),
}

/// WebDynpro 문서 업데이트 중 발생하는 오류의 이늄
#[derive(Error, Debug)]
pub enum UpdateBodyError {
    /// 업데이트 응답 XML을 파싱할 수 없음
    #[error("Failed to parse update document: {0}")]
    Parse(#[from] roxmltree::Error),
    /// 업데이트 응답에서 노드를 찾을 수 없음
    #[error("Cannot find a node from given document: {0}")]
    NoSuchNode(String),
    /// 업데이트 응답의 노드에서 어트리뷰트를 찾을 수 없음
    #[error("Cannot find an attribute {attribute:?} from a node {node:?}")]
    #[allow(missing_docs)]
    NoSuchAttribute { node: String, attribute: String },
    /// 업데이트 응답에서 필요한 콘텐츠를 찾을 수 없음
    #[error("{0} has no content")]
    NoSuchContent(String),
    /// 알 수 없는 엘리먼트가 업데이트 응답에서 발견됨
    #[error("Unknown element found: {0}")]
    UnknownElement(String),
    /// 업데이트 응답에 따라 도큐먼트를 재작성하는 데 실패함
    #[error("Failed to rewrite body document: {0}")]
    RewriteBody(#[from] lol_html::errors::RewritingError),
}

/// WebDynpro 문서를 파싱할 때 발생하는 오류의 이늄
#[derive(Error, Debug)]
pub enum BodyError {
    /// 문서 파싱 실패
    #[error("Failed to parse body document")]
    Parse,
    /// 파싱할 문서가 올바르지 않음
    #[error("Given body document is invalid: {0}")]
    Invalid(String),
    /// 문서 파싱에 필요한 CSS Selector가 올바르지 않음
    #[error("Given selector for parsing body is invalid")]
    InvalidSelector,
    /// 올바르지 않은 엘리먼트
    #[error("Invalid element")]
    InvalidElement,
    /// 엘리먼트를 찾을 수 없음
    #[error("Cannot find element from document: {0}")]
    NoSuchElement(String),
    /// 어트리뷰트를 찾을 수 없음
    #[error("Cannot find attribute: {0}")]
    NoSuchAttribute(String),
    /// 이벤트 문자열을 일반 문자열로 변환하지 못함
    #[error("Cannot parse event str: {0}")]
    ParseEvents(#[from] EventStrUnescapeError),
}

/// 엘리먼트 조작 중 발생하는 오류의 이늄
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum ElementError {
    /// 엘리먼트에서 데이터를 찾을 수 없음
    #[error("Cannot find data {field} in element: {element}")]
    NoSuchData { element: String, field: String },
    /// 주어진 엘리먼트는 해당 이벤트를 발생시킬 수 없음
    #[error("Cannot fire event {event} in element: {element}")]
    NoSuchEvent { element: String, event: String },
    /// 엘리먼트 콘텐츠를 찾을 수 없음
    #[error("Cannot find content {content} in element: {element}")]
    NoSuchContent { element: String, content: String },
    /// 올바르지 않은 엘리먼트 Id
    #[error("Invalid id {0}")]
    InvalidId(String),
    /// 올바르지 않은 엘리먼트 콘텐츠
    #[error("Invalid content {content} in element: {element}")]
    InvalidContent { element: String, content: String },
    /// 엘리먼트의 LSData가 올바른 형태가 아님
    #[error("Cannot parse lsdata: {0}")]
    InvalidLSData(String),
    /// LSData 오브젝트를 파싱할 수 없음
    #[error("Failed parse lsdata json-like object")]
    ParseLSData(#[from] serde_json::Error),
}

/// 이벤트 문자열을 일반 문자열로 변환할 떄 발생하는 오류의 이늄
#[derive(Error, Debug)]
pub enum EventStrUnescapeError {
    /// 이벤트 문자열의 HEX 문자열 파싱 실패
    #[error("Failed read hex string")]
    ParseInt(#[from] std::num::ParseIntError),
    /// 이벤트 문자열의 HEX 문자열이 올바르지 않음
    #[error("hex string is not valid")]
    ParseHex(#[from] std::string::FromUtf16Error),
    /// 주어진 애플리케이션에서 폼을 찾을 수 없음
    #[error("No form found in desired application")]
    NoForm,
}
