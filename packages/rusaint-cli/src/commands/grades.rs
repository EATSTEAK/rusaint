use std::sync::Arc;

use clap::Subcommand;
use rusaint::{
    USaintSession, application::course_grades::CourseGradesApplication, client::USaintClientBuilder,
};

use crate::{
    output::write_json,
    types::{CourseType, SemesterType},
};

#[derive(Subcommand)]
pub enum GradesCommands {
    /// 전체 학기 성적 요약 (기록부 기준)
    RecordedSummary {
        /// 과정 구분
        #[arg(short = 't', long)]
        course_type: CourseType,
    },
    /// 전체 학기 성적 요약 (증명서 기준)
    CertificatedSummary {
        #[arg(short = 't', long)]
        course_type: CourseType,
    },
    /// 이수구분별 성적 조회
    ByClassification {
        #[arg(short = 't', long)]
        course_type: CourseType,
    },
    /// 학기별 성적 목록
    Semesters {
        #[arg(short = 't', long)]
        course_type: CourseType,
    },
    /// 과목별 성적 목록
    Classes {
        #[arg(short = 't', long)]
        course_type: CourseType,
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        /// 상세 정보 포함 여부
        #[arg(long)]
        include_details: bool,
    },
    /// 개별 과목 성적 상세
    ClassDetail {
        #[arg(short = 't', long)]
        course_type: CourseType,
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        /// 과목 코드
        #[arg(short = 'c', long)]
        code: String,
    },
}

pub async fn execute(
    session: Arc<USaintSession>,
    command: GradesCommands,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseGradesApplication>()
        .await?;

    match command {
        GradesCommands::RecordedSummary { course_type } => {
            let result = app.recorded_summary(*course_type).await?;
            write_json("grades_recorded_summary", &result)?;
        }
        GradesCommands::CertificatedSummary { course_type } => {
            let result = app.certificated_summary(*course_type).await?;
            write_json("grades_certificated_summary", &result)?;
        }
        GradesCommands::ByClassification { course_type } => {
            let result = app.grades_by_classification(*course_type).await?;
            write_json("grades_by_classification", &result)?;
        }
        GradesCommands::Semesters { course_type } => {
            let result = app.semesters(*course_type).await?;
            write_json("grades_semesters", &result)?;
        }
        GradesCommands::Classes {
            course_type,
            year,
            semester,
            include_details,
        } => {
            let result = app
                .classes(*course_type, year, *semester, include_details)
                .await?;
            write_json(&format!("grades_classes_{year}_{semester}"), &result)?;
        }
        GradesCommands::ClassDetail {
            course_type,
            year,
            semester,
            code,
        } => {
            let result = app
                .class_detail(*course_type, year, *semester, &code)
                .await?;
            write_json(
                &format!("grades_class_detail_{year}_{semester}_{code}"),
                &result,
            )?;
        }
    }

    Ok(())
}
