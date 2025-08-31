mod semester_type;

use std::{fs::File, io::Write, sync::Arc};

use clap::{Parser, Subcommand};
use rusaint::{
    RusaintError, USaintSession,
    application::course_schedule::{
        CourseScheduleApplication,
        model::{Lecture, LectureCategory},
    },
    client::USaintClientBuilder,
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
    ByLecture {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        #[arg(short = 'k', long)]
        keyword: String,
    },
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
    },
    RequiredElective {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        #[arg(short = 'n', long)]
        course_name: String,
    },
    OptionalElective {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        #[arg(short = 'n', long)]
        course_name: String,
    },
    Chapel {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        #[arg(short = 'n', long)]
        chapel_name: String,
    },
    Education {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
    },
    ConnectedMajor {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        #[arg(short = 'n', long)]
        major_name: String,
    },
    UnitedMajor {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
        #[arg(short = 'n', long)]
        major_name: String,
    },
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
    },
    Cyber {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<RusaintError>> {
    let cli = Cli::parse();

    dotenv().ok();
    let id = std::env::var("SSO_ID").expect("SSO_ID 환경변수가 설정되지 않았습니다.");
    let password =
        std::env::var("SSO_PASSWORD").expect("SSO_PASSWORD 환경변수가 설정되지 않았습니다.");

    let session = Arc::new(USaintSession::with_password(&id, &password).await?);

    match cli.command {
        Commands::ByLecture {
            year,
            semester,
            keyword,
        } => {
            let lectures = find_by_lecture(session.clone(), year, semester, &keyword).await?;
            create_json(format!("{year}_{semester}_{keyword}"), lectures)
        }
        Commands::Major {
            year,
            semester,
            college,
            department,
            major,
        } => {
            let lectures = find_major(
                session.clone(),
                year,
                semester,
                &college,
                &department,
                major.as_deref(),
            )
            .await?;

            if let Some(major) = major {
                create_json(
                    format!("{year}_{semester}_{college}_{department}_{major}_전공"),
                    lectures,
                )
            } else {
                create_json(
                    format!("{year}_{semester}_{college}_{department}_전공"),
                    lectures,
                )
            }
        }
        Commands::RequiredElective {
            year,
            semester,
            course_name,
        } => {
            let lectures =
                find_required_elective(session.clone(), year, semester, &course_name).await?;
            create_json(
                format!("{year}_{semester}_{course_name}_교양필수"),
                lectures,
            )
        }
        Commands::OptionalElective {
            year,
            semester,
            course_name,
        } => {
            let lectures =
                find_optional_elective(session.clone(), year, semester, &course_name).await?;
            create_json(
                format!("{year}_{semester}_{course_name}_교양선택"),
                lectures,
            )
        }
        Commands::Chapel {
            year,
            semester,
            chapel_name,
        } => {
            let lectures = find_chapel(session.clone(), year, semester, &chapel_name).await?;
            create_json(format!("{year}_{semester}_{chapel_name}_채플"), lectures)
        }
        Commands::Education { year, semester } => {
            let lectures = find_education(session.clone(), year, semester).await?;
            create_json(format!("{year}_{semester}_교직"), lectures)
        }
        Commands::ConnectedMajor {
            year,
            semester,
            major_name,
        } => {
            let lectures =
                find_connected_major(session.clone(), year, semester, &major_name).await?;
            create_json(format!("{year}_{semester}_{major_name}_연계전공"), lectures)
        }
        Commands::UnitedMajor {
            year,
            semester,
            major_name,
        } => {
            let lectures = find_united_major(session.clone(), year, semester, &major_name).await?;
            create_json(format!("{year}_{semester}_{major_name}_융합전공"), lectures)
        }
        Commands::RecognizedOtherMajor {
            year,
            semester,
            college,
            department,
            major,
        } => {
            let lectures = find_recognized_other_major(
                session.clone(),
                year,
                semester,
                &college,
                &department,
                major.as_deref(),
            )
            .await?;

            if let Some(major) = major {
                create_json(
                    format!("{year}_{semester}_{college}_{department}_{major}_타전공인정"),
                    lectures,
                )
            } else {
                create_json(
                    format!("{year}_{semester}_{college}_{department}_타전공인정"),
                    lectures,
                )
            }
        }
        Commands::Cyber { year, semester } => {
            let lectures = find_cyber(session.clone(), year, semester).await?;
            create_json(format!("{year}_{semester}_숭사대"), lectures)
        }
    };

    Ok(())
}

fn create_json(file_name: String, lectures: Vec<Lecture>) {
    let json = to_string_pretty(&lectures).expect("Failed to serialize lectures to JSON");
    let mut file = File::create(format!("{file_name}.json")).expect("Failed to create .json");
    file.write_all(json.as_bytes())
        .expect("Failed to write to file");
}

async fn find_by_lecture(
    session: Arc<USaintSession>,
    year: u32,
    semester: SemesterType,
    keyword: &str,
) -> Result<Vec<Lecture>, RusaintError> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await?;
    let category = LectureCategory::find_by_lecture(keyword);
    let lectures = app.find_lectures(year, *semester, &category).await?;
    Ok(lectures.collect())
}

async fn find_major(
    session: Arc<USaintSession>,
    year: u32,
    semester: SemesterType,
    college: &str,
    department: &str,
    major: Option<&str>,
) -> Result<Vec<Lecture>, RusaintError> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await?;
    let category = LectureCategory::major(college, department, major);
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
    major: Option<&str>,
) -> Result<Vec<Lecture>, RusaintError> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await
        .unwrap();
    let category = LectureCategory::recognized_other_major(college, department, major);
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
