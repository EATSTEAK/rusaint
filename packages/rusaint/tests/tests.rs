use anyhow::{Error, Result};
use dotenvy::dotenv;
use lazy_static::lazy_static;
use rusaint::USaintSession;
use rusaint::model::SemesterType;
use std::{fs::File, io::BufReader, sync::Arc};
use test_log::test;

lazy_static! {
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
            _ => panic!("{:?}", Error::msg("Invalid semester")),
        }
    };
}

pub async fn get_session() -> Result<Arc<USaintSession>> {
    let session_file_path = std::env::var("SSO_SESSION_FILE").unwrap_or("session.json".to_string());
    let f = File::open(&session_file_path)
        .map_err(|e| Error::msg(format!("Failed to open session file: {}", e)))?;
    let reader = BufReader::new(f);
    let session: USaintSession = USaintSession::from_json(reader)
        .map_err(|e| Error::msg(format!("Failed to parse session file: {}", e)))?;
    let session = Arc::new(session);
    Ok(session)
}

#[cfg(test)]
#[test(tokio::test)]
async fn test_session() {
    let _ = get_session().await.unwrap();
}

mod application;
#[cfg(test)]
mod webdynpro;
