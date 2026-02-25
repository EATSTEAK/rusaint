use std::{path::Path, sync::Arc};

use clap::Subcommand;
use rusaint::{
    USaintSession, application::lecture_assessment::LectureAssessmentApplication,
    client::USaintClientBuilder,
};

use crate::{
    output::{OutputFormat, write_output},
    types::SemesterType,
};

#[derive(Subcommand)]
pub enum AssessmentCommands {
    /// 강의평가 검색
    Find {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        /// 강의명 필터
        #[arg(long)]
        lecture_name: Option<String>,
        /// 과목 코드 필터
        #[arg(long)]
        lecture_code: Option<u32>,
        /// 교수명 필터
        #[arg(long)]
        professor_name: Option<String>,
    },
}

pub async fn execute(
    session: Arc<USaintSession>,
    command: AssessmentCommands,
    format: &OutputFormat,
    output: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<LectureAssessmentApplication>()
        .await?;

    match command {
        AssessmentCommands::Find {
            year,
            semester,
            lecture_name,
            lecture_code,
            professor_name,
        } => {
            let result = app
                .find_assessments(
                    year,
                    *semester,
                    lecture_name.as_deref(),
                    lecture_code,
                    professor_name.as_deref(),
                )
                .await?;
            write_output(format, output, &result)?;
        }
    }

    Ok(())
}
