use std::sync::Arc;

#[derive(uniffi::Object)]
pub struct USaintSession(rusaint::USaintSession);

#[uniffi::export]
impl USaintSession {
    #[uniffi::constructor]
    pub fn new() -> Arc<USaintSession> {
        Arc::new(USaintSession(rusaint::USaintSession::anonymous()))
    }
}

uniffi::setup_scaffolding!();
