use std::sync::Arc;

use crate::error::RusaintError;

#[derive(Debug, uniffi::Object)]
pub struct USaintSession(Arc<rusaint::USaintSession>);

impl Clone for USaintSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl USaintSession {
    pub fn original(&self) -> Arc<rusaint::USaintSession> {
        self.0.clone()
    }
}

#[derive(Debug, uniffi::Object)]
pub struct USaintSessionBuilder();

#[uniffi::export]
impl USaintSessionBuilder {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self()
    }

    pub fn anonymous(&self) -> USaintSession {
        USaintSession(Arc::new(rusaint::USaintSession::anonymous()))
    }

    pub async fn with_password(&self, id: &str, password: &str) -> Result<USaintSession, RusaintError> {
        let original = rusaint::USaintSession::with_password(id, password).await?;
        Ok(USaintSession(Arc::new(original)))
    }

    pub async fn with_token(&self, id: &str, token: &str) -> Result<USaintSession, RusaintError> {
        let original = rusaint::USaintSession::with_token(id, token).await?;
        Ok(USaintSession(Arc::new(original)))
    }
}

