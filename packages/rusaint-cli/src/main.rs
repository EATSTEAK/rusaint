mod semester_type;

use std::{fs::File, io::Write, sync::Arc};

use clap::{Parser, Subcommand};
use rusaint::{
    application::{
        course_schedule::{
            model::{Lecture, LectureCategory},
            CourseScheduleApplication,
        },
        USaintClientBuilder,
    },
    RusaintError, USaintSession,
};

use dotenv::dotenv;
use semester_type::SemesterType;
use serde_json::to_string_pretty;

#[derive(Parser)]
#[command(name = "Course Schedule CLI")]
#[command(about = "CLI for fetching course schedules")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    FindMajor {
        #[arg(long)]
        year: u32,
        #[arg(long)]
        semester: SemesterType,
        #[arg(long)]
        college: String,
        #[arg(long)]
        department: String,
    },
    FindRequiredElective {
        #[arg(long)]
        year: u32,
        #[arg(long)]
        semester: SemesterType,
        #[arg(long)]
        course_name: String,
    },
    FindOptionalElective {
        #[arg(long)]
        year: u32,
        #[arg(long)]
        semester: SemesterType,
        #[arg(long)]
        course_name: String,
    },
    FindChapel {
        #[arg(long)]
        year: u32,
        #[arg(long)]
        semester: SemesterType,
        #[arg(long)]
        chapel_name: String,
    },
    FindEducation {
        #[arg(long)]
        year: u32,
        #[arg(long)]
        semester: SemesterType,
    },
    FindConnectedMajor {
        #[arg(long)]
        year: u32,
        #[arg(long)]
        semester: SemesterType,
        #[arg(long)]
        major_name: String,
    },
    FindUnitedMajor {
        #[arg(long)]
        year: u32,
        #[arg(long)]
        semester: SemesterType,
        #[arg(long)]
        major_name: String,
    },
    FindRecognizedOtherMajor {
        #[arg(long)]
        year: u32,
        #[arg(long)]
        semester: SemesterType,
        #[arg(long)]
        college: String,
        #[arg(long)]
        department: String,
    },
    FindCyber {
        #[arg(long)]
        year: u32,
        #[arg(long)]
        semester: SemesterType,
    },
}

#[tokio::main]
async fn main() -> Result<(), RusaintError> {
    let cli = Cli::parse();

    dotenv().ok();
    let id = std::env::var("SSO_ID").expect("SSO_ID 환경변수가 설정되지 않았습니다.");
    let password =
        std::env::var("SSO_PASSWORD").expect("SSO_PASSWORD 환경변수가 설정되지 않았습니다.");

    let session = Arc::new(USaintSession::with_password(&id, &password).await?);

    match cli.command {
        Commands::FindMajor {
            year,
            semester,
            college,
            department,
        } => {
            let lectures =
                find_major(session.clone(), year, semester, &college, &department).await?;

            create_json(
                format!("{}_{}_{}_{}_전공", year, semester, college, department),
                lectures,
            )
        }
        Commands::FindRequiredElective {
            year,
            semester,
            course_name,
        } => {
            let lectures =
                find_required_elective(session.clone(), year, semester, &course_name).await?;
            create_json(
                format!("{}_{}_{}_교양필수", year, semester, course_name),
                lectures,
            )
        }
        Commands::FindOptionalElective {
            year,
            semester,
            course_name,
        } => {
            let lectures =
                find_optional_elective(session.clone(), year, semester, &course_name).await?;
            create_json(
                format!("{}_{}_{}_교양선택", year, semester, course_name),
                lectures,
            )
        }
        Commands::FindChapel {
            year,
            semester,
            chapel_name,
        } => {
            let lectures = find_chapel(session.clone(), year, semester, &chapel_name).await?;
            create_json(
                format!("{}_{}_{}_채플", year, semester, chapel_name),
                lectures,
            )
        }
        Commands::FindEducation { year, semester } => {
            let lectures = find_education(session.clone(), year, semester).await?;
            create_json(format!("{}_{}_교직", year, semester), lectures)
        }
        Commands::FindConnectedMajor {
            year,
            semester,
            major_name,
        } => {
            let lectures =
                find_connected_major(session.clone(), year, semester, &major_name).await?;
            create_json(
                format!("{}_{}_{}_연계전공", year, semester, major_name),
                lectures,
            )
        }
        Commands::FindUnitedMajor {
            year,
            semester,
            major_name,
        } => {
            let lectures = find_united_major(session.clone(), year, semester, &major_name).await?;
            create_json(
                format!("{}_{}_{}_융합전공", year, semester, major_name),
                lectures,
            )
        }
        Commands::FindRecognizedOtherMajor {
            year,
            semester,
            college,
            department,
        } => {
            let lectures =
                find_recognized_other_major(session.clone(), year, semester, &college, &department)
                    .await?;
            create_json(
                format!(
                    "{}_{}_{}_{}_타전공인정",
                    year, semester, college, department
                ),
                lectures,
            )
        }
        Commands::FindCyber { year, semester } => {
            let lectures = find_cyber(session.clone(), year, semester).await?;
            create_json(format!("{}_{}_숭사대", year, semester), lectures)
        }
    };

    Ok(())
}

fn create_json(file_name: String, lectures: Vec<Lecture>) {
    let json = to_string_pretty(&lectures).expect("Failed to serialize lectures to JSON");
    let mut file = File::create(format!("{}.json", file_name)).expect("Failed to create .json");
    file.write_all(json.as_bytes())
        .expect("Failed to write to file");
}

async fn find_major(
    session: Arc<USaintSession>,
    year: u32,
    semester: SemesterType,
    college: &str,
    department: &str,
) -> Result<Vec<Lecture>, RusaintError> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await?;
    let category = LectureCategory::major(college, department, None);
    let lectures = app.find_lectures(year, *semester, &category).await?;
    Ok(lectures.collect())
}

async fn find_required_elective(
    session: Arc<USaintSession>,
    year: u32,
    semester: SemesterType,
    course_name: &str,
) -> Result<Vec<Lecture>, RusaintError> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::required_elective(course_name);
    let lectures = app.find_lectures(year, *semester, &category).await.unwrap();
    Ok(lectures.collect())
}

async fn find_optional_elective(
    session: Arc<USaintSession>,
    year: u32,
    semester: SemesterType,
    course_name: &str,
) -> Result<Vec<Lecture>, RusaintError> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::optional_elective(course_name);
    let lectures = app.find_lectures(year, *semester, &category).await.unwrap();
    Ok(lectures.collect())
}

async fn find_chapel(
    session: Arc<USaintSession>,
    year: u32,
    semester: SemesterType,
    chapel_name: &str,
) -> Result<Vec<Lecture>, RusaintError> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::chapel(chapel_name);
    let lectures = app.find_lectures(year, *semester, &category).await.unwrap();
    Ok(lectures.collect())
}

async fn find_education(
    session: Arc<USaintSession>,
    year: u32,
    semester: SemesterType,
) -> Result<Vec<Lecture>, RusaintError> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::education();
    let lectures = app.find_lectures(year, *semester, &category).await.unwrap();
    Ok(lectures.collect())
}

async fn find_connected_major(
    session: Arc<USaintSession>,
    year: u32,
    semester: SemesterType,
    major_name: &str,
) -> Result<Vec<Lecture>, RusaintError> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::connected_major(major_name);
    let lectures = app.find_lectures(year, *semester, &category).await.unwrap();
    Ok(lectures.collect())
}

async fn find_united_major(
    session: Arc<USaintSession>,
    year: u32,
    semester: SemesterType,
    major_name: &str,
) -> Result<Vec<Lecture>, RusaintError> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::united_major(major_name);
    let lectures = app.find_lectures(year, *semester, &category).await.unwrap();
    Ok(lectures.collect())
}

async fn find_recognized_other_major(
    session: Arc<USaintSession>,
    year: u32,
    semester: SemesterType,
    college: &str,
    department: &str,
) -> Result<Vec<Lecture>, RusaintError> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::recognized_other_major(college, department, None);
    let lectures = app.find_lectures(year, *semester, &category).await.unwrap();
    Ok(lectures.collect())
}

async fn find_cyber(
    session: Arc<USaintSession>,
    year: u32,
    semester: SemesterType,
) -> Result<Vec<Lecture>, RusaintError> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::cyber();
    let lectures = app.find_lectures(year, *semester, &category).await.unwrap();
    Ok(lectures.collect())
}
