use anyhow::{Error, Result};
use dotenv::dotenv;
use std::sync::{Arc, OnceLock};

use rusaint::USaintSession;

static SESSION: OnceLock<Arc<USaintSession>> = OnceLock::new();

pub async fn get_session() -> Result<Arc<USaintSession>> {
    if let Some(session) = SESSION.get() {
        Ok(session.to_owned())
    } else {
        dotenv().ok();
        let id = std::env::var("SSO_ID")?;
        let password = std::env::var("SSO_PASSWORD")?;
        let session = USaintSession::with_password(&id, &password).await?;
        let _ = SESSION.set(Arc::new(session));
        SESSION
            .get()
            .and_then(|arc| Some(arc.to_owned()))
            .ok_or(Error::msg("Session is not initsiated"))
    }
}

mod application;
#[cfg(test)]
mod webdynpro;
