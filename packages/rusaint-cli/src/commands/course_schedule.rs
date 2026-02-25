use std::sync::Arc;

use clap::Subcommand;
use rusaint::{
    USaintSession,
    application::course_schedule::{CourseScheduleApplication, model::LectureCategory},
    client::USaintClientBuilder,
};

use crate::{output::write_json, types::SemesterType};

#[derive(Subcommand)]
pub enum CourseScheduleCommands {
    /// 강의명으로 검색
    ByLecture {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        #[arg(short = 'k', long)]
        keyword: String,
        /// 상세 정보 포함 조회
        #[arg(long)]
        detailed: bool,
        /// 강의계획서 포함 조회 (--detailed 자동 포함)
        #[arg(long)]
        fetch_syllabus: bool,
    },
    /// 전공별 검색
    Major {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        #[arg(short = 'c', long)]
        college: String,
        #[arg(short = 'd', long)]
        department: String,
        #[arg(short = 'm', long)]
        major: Option<String>,
        #[arg(long)]
        detailed: bool,
        #[arg(long)]
        fetch_syllabus: bool,
    },
    /// 교양필수 검색
    RequiredElective {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        #[arg(short = 'n', long)]
        course_name: String,
        #[arg(long)]
        detailed: bool,
        #[arg(long)]
        fetch_syllabus: bool,
    },
    /// 교양선택 검색
    OptionalElective {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        #[arg(short = 'n', long)]
        course_name: String,
        #[arg(long)]
        detailed: bool,
        #[arg(long)]
        fetch_syllabus: bool,
    },
    /// 채플 강의 검색
    Chapel {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        #[arg(short = 'n', long)]
        chapel_name: String,
        #[arg(long)]
        detailed: bool,
        #[arg(long)]
        fetch_syllabus: bool,
    },
    /// 교직 검색
    Education {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        #[arg(long)]
        detailed: bool,
        #[arg(long)]
        fetch_syllabus: bool,
    },
    /// 연계전공 검색
    ConnectedMajor {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        #[arg(short = 'n', long)]
        major_name: String,
        #[arg(long)]
        detailed: bool,
        #[arg(long)]
        fetch_syllabus: bool,
    },
    /// 융합전공 검색
    UnitedMajor {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        #[arg(short = 'n', long)]
        major_name: String,
        #[arg(long)]
        detailed: bool,
        #[arg(long)]
        fetch_syllabus: bool,
    },
    /// 타전공인정 검색
    RecognizedOtherMajor {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        #[arg(short = 'c', long)]
        college: String,
        #[arg(short = 'd', long)]
        department: String,
        #[arg(short = 'm', long)]
        major: Option<String>,
        #[arg(long)]
        detailed: bool,
        #[arg(long)]
        fetch_syllabus: bool,
    },
    /// 숭실사이버대학교 검색
    Cyber {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        #[arg(long)]
        detailed: bool,
        #[arg(long)]
        fetch_syllabus: bool,
    },
    /// 대학원 검색
    Graduated {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        #[arg(short = 'c', long)]
        college: String,
        #[arg(short = 'd', long)]
        department: String,
        #[arg(long)]
        detailed: bool,
        #[arg(long)]
        fetch_syllabus: bool,
    },
    /// 교수명으로 검색
    FindByProfessor {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        #[arg(short = 'k', long)]
        keyword: String,
        #[arg(long)]
        detailed: bool,
        #[arg(long)]
        fetch_syllabus: bool,
    },
}

pub async fn execute(
    session: Arc<USaintSession>,
    command: CourseScheduleCommands,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await?;

    let (year, semester, category, file_name, detailed, fetch_syllabus) = match &command {
        CourseScheduleCommands::ByLecture {
            year,
            semester,
            keyword,
            detailed,
            fetch_syllabus,
        } => (
            *year,
            *semester,
            LectureCategory::find_by_lecture(keyword),
            format!("{year}_{semester}_{keyword}"),
            *detailed,
            *fetch_syllabus,
        ),
        CourseScheduleCommands::Major {
            year,
            semester,
            college,
            department,
            major,
            detailed,
            fetch_syllabus,
        } => {
            let name = if let Some(m) = major {
                format!("{year}_{semester}_{college}_{department}_{m}_전공")
            } else {
                format!("{year}_{semester}_{college}_{department}_전공")
            };
            (
                *year,
                *semester,
                LectureCategory::major(college, department, major.as_deref()),
                name,
                *detailed,
                *fetch_syllabus,
            )
        }
        CourseScheduleCommands::RequiredElective {
            year,
            semester,
            course_name,
            detailed,
            fetch_syllabus,
        } => (
            *year,
            *semester,
            LectureCategory::required_elective(course_name),
            format!("{year}_{semester}_{course_name}_교양필수"),
            *detailed,
            *fetch_syllabus,
        ),
        CourseScheduleCommands::OptionalElective {
            year,
            semester,
            course_name,
            detailed,
            fetch_syllabus,
        } => (
            *year,
            *semester,
            LectureCategory::optional_elective(course_name),
            format!("{year}_{semester}_{course_name}_교양선택"),
            *detailed,
            *fetch_syllabus,
        ),
        CourseScheduleCommands::Chapel {
            year,
            semester,
            chapel_name,
            detailed,
            fetch_syllabus,
        } => (
            *year,
            *semester,
            LectureCategory::chapel(chapel_name),
            format!("{year}_{semester}_{chapel_name}_채플"),
            *detailed,
            *fetch_syllabus,
        ),
        CourseScheduleCommands::Education {
            year,
            semester,
            detailed,
            fetch_syllabus,
        } => (
            *year,
            *semester,
            LectureCategory::education(),
            format!("{year}_{semester}_교직"),
            *detailed,
            *fetch_syllabus,
        ),
        CourseScheduleCommands::ConnectedMajor {
            year,
            semester,
            major_name,
            detailed,
            fetch_syllabus,
        } => (
            *year,
            *semester,
            LectureCategory::connected_major(major_name),
            format!("{year}_{semester}_{major_name}_연계전공"),
            *detailed,
            *fetch_syllabus,
        ),
        CourseScheduleCommands::UnitedMajor {
            year,
            semester,
            major_name,
            detailed,
            fetch_syllabus,
        } => (
            *year,
            *semester,
            LectureCategory::united_major(major_name),
            format!("{year}_{semester}_{major_name}_융합전공"),
            *detailed,
            *fetch_syllabus,
        ),
        CourseScheduleCommands::RecognizedOtherMajor {
            year,
            semester,
            college,
            department,
            major,
            detailed,
            fetch_syllabus,
        } => {
            let name = if let Some(m) = major {
                format!("{year}_{semester}_{college}_{department}_{m}_타전공인정")
            } else {
                format!("{year}_{semester}_{college}_{department}_타전공인정")
            };
            (
                *year,
                *semester,
                LectureCategory::recognized_other_major(college, department, major.as_deref()),
                name,
                *detailed,
                *fetch_syllabus,
            )
        }
        CourseScheduleCommands::Cyber {
            year,
            semester,
            detailed,
            fetch_syllabus,
        } => (
            *year,
            *semester,
            LectureCategory::cyber(),
            format!("{year}_{semester}_숭사대"),
            *detailed,
            *fetch_syllabus,
        ),
        CourseScheduleCommands::Graduated {
            year,
            semester,
            college,
            department,
            detailed,
            fetch_syllabus,
        } => (
            *year,
            *semester,
            LectureCategory::graduated(college, department),
            format!("{year}_{semester}_{college}_{department}_대학원"),
            *detailed,
            *fetch_syllabus,
        ),
        CourseScheduleCommands::FindByProfessor {
            year,
            semester,
            keyword,
            detailed,
            fetch_syllabus,
        } => (
            *year,
            *semester,
            LectureCategory::find_by_professor(keyword),
            format!("{year}_{semester}_{keyword}_교수"),
            *detailed,
            *fetch_syllabus,
        ),
    };

    if fetch_syllabus || detailed {
        let lectures = app
            .find_detailed_lectures(year, *semester, &category, fetch_syllabus)
            .await?;
        write_json(&file_name, &lectures)?;
    } else {
        let lectures: Vec<_> = app
            .find_lectures(year, *semester, &category)
            .await?
            .collect();
        write_json(&file_name, &lectures)?;
    }

    Ok(())
}
