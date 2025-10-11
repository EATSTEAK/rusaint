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

#[uniffi::export]
impl USaintSession {
    /// 세션을 json 형식으로 변환합니다.
    pub fn to_json(&self) -> Result<String, RusaintError> {
        let mut buffer = Vec::new();
        {
            let mut writer = std::io::BufWriter::new(&mut buffer);
            self.0.save_to_json(&mut writer)?;
        }

        Ok(String::from_utf8(buffer).unwrap())
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

    /// json 형식으로 저장된 세션을 읽어 세션을 생성합니다.
    pub fn from_json(&self, json: &str) -> Result<USaintSession, RusaintError> {
        let reader = std::io::BufReader::new(json.as_bytes());
        let session = rusaint::USaintSession::from_json(reader)?;
        Ok(USaintSession(Arc::new(session)))
    }
}

impl Default for USaintSessionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_with_json() {
        let session = USaintSessionBuilder::new().anonymous();
        let json = session.to_json().unwrap();
        let session2 = USaintSessionBuilder::new().from_json(&json).unwrap();
        let json2 = session2.to_json().unwrap();
        assert_eq!(json, json2);
    }
}
