use std::sync::Arc;

use clap::Subcommand;
use rusaint::{
    USaintSession,
    application::personal_course_schedule::PersonalCourseScheduleApplication,
    client::USaintClientBuilder,
};

use crate::{output::write_json, types::SemesterType};

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
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<PersonalCourseScheduleApplication>()
        .await?;

    match command {
        PersonalScheduleCommands::Schedule { year, semester } => {
            let result = app.schedule(year, *semester).await?;
            write_json(
                &format!("personal_schedule_{year}_{semester}"),
                &result,
            )?;
        }
    }

    Ok(())
}
