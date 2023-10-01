use anyhow::{Error, Result};
use dotenv::dotenv;
use std::sync::{Arc, OnceLock};

use rusaint::session::USaintSession;

static SESSION: OnceLock<Arc<USaintSession>> = OnceLock::new();

pub(crate) async fn get_session() -> Result<Arc<USaintSession>> {
    if let Some(session) = SESSION.get() {
        Ok(session.to_owned())
    } else {
        dotenv().ok();
        let id = std::env::var("SSO_ID").unwrap();
        let password = std::env::var("SSO_PASSWORD").unwrap();
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
