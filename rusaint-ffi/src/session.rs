use crate::error::RusaintError;

#[derive(uniffi::Object)]
pub struct USaintSession(rusaint::USaintSession);

#[derive(uniffi::Object)]
pub struct USaintSessionBuilder {
    id: String,
}

#[uniffi::export]
impl USaintSessionBuilder {
    #[uniffi::constructor]
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }

    pub async fn build_with_password(&self, password: &str) -> Result<USaintSession, RusaintError> {
        let original_session = rusaint::USaintSession::with_password(&self.id, password).await?;
        Ok(USaintSession(original_session))
    }

    pub async fn build_with_token(&self, token: &str) -> Result<USaintSession, RusaintError> {
        let original_session = rusaint::USaintSession::with_token(&self.id, token).await?;
        Ok(USaintSession(original_session))
    }
}
