use thiserror::Error;

use crate::webdynpro::error::WebDynproError;

/// Rusaint 애플리케이션에서 반환하는 오류
#[derive(Error, Debug)]
pub enum RusaintError {
    /// 내부 WebDynpro 엔진 오류
    #[error("WebDynpro engine error: {0}")]
    WebDynproError(#[from] WebDynproError),
    /// 애플리케이션의 기반 클라이언트 불일치 오류
    #[error("Invalid Client for target application")]
    InvalidClientError,
    /// 숭실대학교 SSO 로그인 오류
    #[error("Failed to login with ssu sso: {0}")]
    SsoLoginError(#[from] SsuSsoError),
}

/// 숭실대학교 SSO 로그인 실패 시 반환하는 오류
#[derive(Error, Debug)]
pub enum SsuSsoError {
    /// 웹 요청, 응답 오류
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    /// 페이지 로그인 폼을 찾을 수 없음
    #[error("Can't load form data from page, is page changed?")]
    CantLoadForm,
    /// 페이지 로그인이 실패하여 토큰이 응답에 포함되지 않음
    #[error("Token is not included in response.")]
    CantFindToken,
}
