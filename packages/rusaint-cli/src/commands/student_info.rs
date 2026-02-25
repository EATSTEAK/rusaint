use std::sync::Arc;

use clap::Subcommand;
use rusaint::{
    USaintSession,
    application::student_information::StudentInformationApplication,
    client::USaintClientBuilder,
};

use crate::output::write_json;

#[derive(Subcommand)]
pub enum StudentInfoCommands {
    /// 일반 학생정보 조회
    General,
    /// 졸업정보 조회
    Graduation,
    /// 자격증 정보 조회
    Qualifications,
    /// 직장정보 조회
    Work,
    /// 가족정보 조회
    Family,
    /// 종교정보 조회
    Religion,
    /// 편입정보 조회
    Transfer,
    /// 은행계좌 정보 조회
    BankAccount,
    /// 학적상태 이력 조회
    AcademicRecord,
    /// 연구비 계좌 조회
    ResearchBankAccount,
}

pub async fn execute(
    session: Arc<USaintSession>,
    command: StudentInfoCommands,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<StudentInformationApplication>()
        .await?;

    match command {
        StudentInfoCommands::General => {
            let result = app.general()?;
            write_json("student_general", &result)?;
        }
        StudentInfoCommands::Graduation => {
            let result = app.graduation()?;
            write_json("student_graduation", &result)?;
        }
        StudentInfoCommands::Qualifications => {
            let result = app.qualifications()?;
            write_json("student_qualifications", &result)?;
        }
        StudentInfoCommands::Work => {
            let result = app.work().await?;
            write_json("student_work", &result)?;
        }
        StudentInfoCommands::Family => {
            let result = app.family().await?;
            write_json("student_family", &result)?;
        }
        StudentInfoCommands::Religion => {
            let result = app.religion().await?;
            write_json("student_religion", &result)?;
        }
        StudentInfoCommands::Transfer => {
            let result = app.transfer().await?;
            write_json("student_transfer", &result)?;
        }
        StudentInfoCommands::BankAccount => {
            let result = app.bank_account().await?;
            write_json("student_bank_account", &result)?;
        }
        StudentInfoCommands::AcademicRecord => {
            let result = app.academic_record().await?;
            write_json("student_academic_record", &result)?;
        }
        StudentInfoCommands::ResearchBankAccount => {
            let result = app.research_bank_account().await?;
            write_json("student_research_bank_account", &result)?;
        }
    }

    Ok(())
}
