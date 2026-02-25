use std::{path::Path, sync::Arc};

use clap::Subcommand;
use rusaint::{
    USaintSession, application::course_registration_status::CourseRegistrationStatusApplication,
    client::USaintClientBuilder,
};

use crate::{
    output::{OutputFormat, write_output},
    types::SemesterType,
};

#[derive(Subcommand)]
pub enum RegistrationCommands {
    /// 수강신청 내역 조회
    Lectures {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
    },
}

pub async fn execute(
    session: Arc<USaintSession>,
    command: RegistrationCommands,
    format: &OutputFormat,
    output: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseRegistrationStatusApplication>()
        .await?;

    match command {
        RegistrationCommands::Lectures { year, semester } => {
            let lectures: Vec<_> = app.lectures(year, *semester).await?.collect();
            write_output(format, output, &lectures)?;
        }
    }

    Ok(())
}
