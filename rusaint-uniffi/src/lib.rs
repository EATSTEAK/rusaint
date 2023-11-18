use std::sync::Arc;

#[derive(thiserror::Error, Debug, uniffi::Error)]
#[uniffi(flat_error)]
pub enum RusaintError {
    /// 내부 WebDynpro 엔진 오류
    #[error("WebDynpro engine error: {0}")]
    WebDynproError(#[from] rusaint::webdynpro::error::WebDynproError),
    /// 숭실대학교 SSO 로그인 오류
    #[error("Failed to login with ssu sso: {0}")]
    SsoLoginError(#[from] rusaint::SsuSsoError),
}

#[derive(uniffi::Object)]
pub struct USaintSession(rusaint::USaintSession);

#[uniffi::export]
impl USaintSession {
    #[uniffi::constructor]
    pub fn new() -> Arc<USaintSession> {
        Arc::new(USaintSession(rusaint::USaintSession::anonymous()))
    }
}

#[uniffi::export]
pub async fn obtain_ssu_sso_token(id: String, password: String) -> Result<String, RusaintError> {
    Ok(rusaint::obtain_ssu_sso_token(&id, &password).await?)
}

mod application {
    #[derive(uniffi::Object)]
    pub struct CourseGrades(rusaint::application::course_grades::CourseGrades);
}

uniffi::setup_scaffolding!();
