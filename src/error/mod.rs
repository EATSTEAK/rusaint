use thiserror::Error;

use crate::webdynpro::error::WebDynproError;

#[derive(Error, Debug)]
pub enum RusaintError {
    /// 내부 WebDynpro 엔진 오류
    #[error("WebDynpro engine error: {0}")]
    WebDynproError(#[from] WebDynproError),
    /// 숭실대학교 SSO 로그인 오류
    #[error("Failed to login with ssu sso: {0}")]
    SsoLoginError(#[from] SsuSsoError),
}

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
