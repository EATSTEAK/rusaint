use std::sync::Arc;

use crate::error::RusaintError;

/// u-saint에서 사용할 세션
/// [`USaintSessionBuilder`]를 이용해 생성합니다.
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

/// [`USaintSession`]을 생성하기 위한 빌더
#[derive(Debug, uniffi::Object)]
pub struct USaintSessionBuilder();

#[uniffi::export(async_runtime = "tokio")]
impl USaintSessionBuilder {
    /// 새로운 [`USaintSessionBuilder`]를 만듭니다.
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self()
    }

    /// 익명 세션(비로그인)을 만듭니다.
    /// ## Kotlin
    /// ```kotlin
    /// fun createAnonymousSession() {
    ///     val anonymous = USaintSessionBuilder().anonymous()
    /// }
    /// ```
    pub fn anonymous(&self) -> USaintSession {
        USaintSession(Arc::new(rusaint::USaintSession::anonymous()))
    }

    /// ID, 비밀번호로 세션을 만듭니다.
    /// ## Kotlin
    /// ```kotlin
    /// suspend fun createSessionWithPassword() {
    ///     val withPassword = USaintSessionBuilder().withPassword("20211561", "password") // suspend
    /// }
    pub async fn with_password(
        &self,
        id: &str,
        password: &str,
    ) -> Result<USaintSession, RusaintError> {
        let original = rusaint::USaintSession::with_password(id, password).await?;
        Ok(USaintSession(Arc::new(original)))
    }

    /// SSO 토큰으로 세션을 만듭니다.
    /// ## Kotlin
    /// ```kotlin
    /// suspend fun createSessionWithSsoToken() {
    ///     val withToken = USaintSessionBuilder().withToken("<example sso token>") // suspend
    /// }
    pub async fn with_token(&self, id: &str, token: &str) -> Result<USaintSession, RusaintError> {
        let original = rusaint::USaintSession::with_token(id, token).await?;
        Ok(USaintSession(Arc::new(original)))
    }
}

impl Default for USaintSessionBuilder {
    fn default() -> Self {
        Self::new()
    }
}
