use std::{path::Path, sync::Arc};

use clap::Subcommand;
use rusaint::{
    USaintSession, application::student_information::StudentInformationApplication,
    client::USaintClientBuilder,
};

use crate::output::{OutputFormat, write_output};

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
    format: &OutputFormat,
    output: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<StudentInformationApplication>()
        .await?;

    match command {
        StudentInfoCommands::General => {
            let result = app.general()?;
            write_output(format, output, &result)?;
        }
        StudentInfoCommands::Graduation => {
            let result = app.graduation()?;
            write_output(format, output, &result)?;
        }
        StudentInfoCommands::Qualifications => {
            let result = app.qualifications()?;
            write_output(format, output, &result)?;
        }
        StudentInfoCommands::Work => {
            let result = app.work().await?;
            write_output(format, output, &result)?;
        }
        StudentInfoCommands::Family => {
            let result = app.family().await?;
            write_output(format, output, &result)?;
        }
        StudentInfoCommands::Religion => {
            let result = app.religion().await?;
            write_output(format, output, &result)?;
        }
        StudentInfoCommands::Transfer => {
            let result = app.transfer().await?;
            write_output(format, output, &result)?;
        }
        StudentInfoCommands::BankAccount => {
            let result = app.bank_account().await?;
            write_output(format, output, &result)?;
        }
        StudentInfoCommands::AcademicRecord => {
            let result = app.academic_record().await?;
            write_output(format, output, &result)?;
        }
        StudentInfoCommands::ResearchBankAccount => {
            let result = app.research_bank_account().await?;
            write_output(format, output, &result)?;
        }
    }

    Ok(())
}
