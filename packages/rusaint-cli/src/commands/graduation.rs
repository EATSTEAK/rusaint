use std::{path::Path, sync::Arc};

use clap::Subcommand;
use rusaint::{
    USaintSession, application::graduation_requirements::GraduationRequirementsApplication,
    client::USaintClientBuilder,
};

use crate::output::{OutputFormat, write_output};

#[derive(Subcommand)]
pub enum GraduationCommands {
    /// 졸업사정 학생정보 조회
    StudentInfo,
    /// 졸업요건 조회
    Requirements,
}

pub async fn execute(
    session: Arc<USaintSession>,
    command: GraduationCommands,
    format: &OutputFormat,
    output: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<GraduationRequirementsApplication>()
        .await?;

    match command {
        GraduationCommands::StudentInfo => {
            let result = app.student_info().await?;
            write_output(format, output, &result)?;
        }
        GraduationCommands::Requirements => {
            let result = app.requirements().await?;
            write_output(format, output, &result)?;
        }
    }

    Ok(())
}
