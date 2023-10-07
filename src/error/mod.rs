use thiserror::Error;

use crate::webdynpro::error::WebDynproError;

#[derive(Error, Debug)]
pub enum RusaintError {
    #[error("WebDynpro engine error: {0}")]
    WebDynproError(#[from] WebDynproError),
    #[error("Failed to login with ssu sso: {0}")]
    SsoLoginError(#[from] SsuSsoError),
}

#[derive(Error, Debug)]
pub enum SsuSsoError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Can't load form data from page, is page changed?")]
    CantLoadForm,
    #[error("Token is not included in response.")]
    CantFindToken,
}
