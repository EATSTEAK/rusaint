use std::{path::Path, sync::Arc};

use clap::Subcommand;
use rusaint::{
    USaintSession, application::personal_course_schedule::PersonalCourseScheduleApplication,
    client::USaintClientBuilder,
};

use crate::{
    output::{OutputFormat, write_output},
    types::SemesterType,
};

#[derive(Subcommand)]
pub enum PersonalScheduleCommands {
    /// 개인시간표 조회
    Schedule {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
    },
}

pub async fn execute(
    session: Arc<USaintSession>,
    command: PersonalScheduleCommands,
    format: &OutputFormat,
    output: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<PersonalCourseScheduleApplication>()
        .await?;

    match command {
        PersonalScheduleCommands::Schedule { year, semester } => {
            let result = app.schedule(year, *semester).await?;
            write_output(format, output, &result)?;
        }
    }

    Ok(())
}
