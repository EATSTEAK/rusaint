use std::sync::Arc;

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