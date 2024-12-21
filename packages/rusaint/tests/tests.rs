use anyhow::{Error, Result};
use dotenv::dotenv;
use lazy_static::lazy_static;
use rusaint::model::SemesterType;
use rusaint::USaintSession;
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;

lazy_static! {
    pub(crate) static ref SESSION: Mutex<OnceLock<Arc<USaintSession>>> =
        Mutex::new(OnceLock::new());
    pub(crate) static ref TARGET_YEAR: u32 = {
        dotenv().ok();
        std::env::var("TARGET_YEAR").unwrap().parse().unwrap()
    };
    pub(crate) static ref TARGET_SEMESTER: SemesterType = {
        dotenv().ok();
        let semester = std::env::var("TARGET_SEMESTER").unwrap();
        match semester.to_uppercase().as_str() {
            "1" | "ONE" => SemesterType::One,
            "SUMMER" => SemesterType::Summer,
            "2" | "TWO" => SemesterType::Two,
            "WINTER" => SemesterType::Winter,
            _ => Err(Error::msg("Invalid semester")).unwrap(),
        }
    };
}

pub async fn get_session() -> Result<Arc<USaintSession>> {
    let session_lock = SESSION.lock().await;
    if let Some(session) = session_lock.get() {
        // Throttle session access to prevent 500 error at server response
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        Ok(session.to_owned())
    } else {
        dotenv().ok();
        let id = std::env::var("SSO_ID")?;
        let password = std::env::var("SSO_PASSWORD")?;
        let session = USaintSession::with_password(&id, &password).await?;
        let _ = session_lock.set(Arc::new(session));
        // Throttle session access to prevent 500 error at server response
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        session_lock
            .get()
            .map(|arc| arc.to_owned())
            .ok_or(Error::msg("Session is not initiated"))
    }
}

mod application;
#[cfg(test)]
mod webdynpro;
