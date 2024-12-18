use anyhow::{Error, Result};
use dotenv::dotenv;
use std::sync::{Arc, OnceLock};

use rusaint::{model::SemesterType, USaintSession};

pub async fn get_session() -> Result<Arc<USaintSession>> {
    static SESSION: OnceLock<Arc<USaintSession>> = OnceLock::new();
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
            .map(|arc| arc.to_owned())
            .ok_or(Error::msg("Session is not initsiated"))
    }
}

pub fn get_year() -> Result<u32> {
    static TARGET_YEAR: OnceLock<u32> = OnceLock::new();
    if let Some(year) = TARGET_YEAR.get() {
        Ok(*year)
    } else {
        let year = std::env::var("TARGET_YEAR")?.parse()?;
        let _ = TARGET_YEAR.set(year);
        TARGET_YEAR
            .get()
            .copied()
            .ok_or(Error::msg("Year is not initsiated"))
    }
}

pub fn get_semester() -> Result<SemesterType> {
    static TARGET_SEMESTER: OnceLock<SemesterType> = OnceLock::new();
    if let Some(semester) = TARGET_SEMESTER.get() {
        Ok(*semester)
    } else {
        let semester = std::env::var("TARGET_SEMESTER")?;
        let semester_type = match semester.to_uppercase().as_str() {
            "1" | "ONE" => SemesterType::One,
            "SUMMER" => SemesterType::Summer,
            "2" | "TWO" => SemesterType::Two,
            "WINTER" => SemesterType::Winter,
            _ => return Err(Error::msg("Invalid semester")),
        };
        let _ = TARGET_SEMESTER.set(semester_type);
        TARGET_SEMESTER
            .get()
            .copied()
            .ok_or(Error::msg("Semester is not initsiated"))
    }
}

mod application;
#[cfg(test)]
mod webdynpro;
