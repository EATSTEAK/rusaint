mod commands;
mod output;
mod session;
mod types;

use std::path::PathBuf;

use clap::Parser;
use commands::Commands;

#[derive(Parser)]
#[command(name = "rusaint-cli")]
#[command(about = "CLI for rusaint - 숭실대학교 u-Saint 학사 정보 조회")]
struct Cli {
    /// 세션 JSON 파일 경로. 지정 시 SSO_ID/SSO_PASSWORD 대신 사용
    #[arg(long, global = true)]
    session_file: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::CreateSession(args) => {
            commands::create_session::execute(args).await?;
        }
        command => {
            let session = session::get_session(cli.session_file.as_deref()).await?;
            match command {
                Commands::CreateSession(_) => unreachable!(),
                Commands::CourseSchedule { command } => {
                    commands::course_schedule::execute(session, command).await?;
                }
                Commands::StudentInfo { command } => {
                    commands::student_info::execute(session, command).await?;
                }
                Commands::Grades { command } => {
                    commands::grades::execute(session, command).await?;
                }
                Commands::ChapelInfo { command } => {
                    commands::chapel::execute(session, command).await?;
                }
                Commands::Registration { command } => {
                    commands::registration::execute(session, command).await?;
                }
                Commands::Graduation { command } => {
                    commands::graduation::execute(session, command).await?;
                }
                Commands::Assessment { command } => {
                    commands::assessment::execute(session, command).await?;
                }
                Commands::PersonalSchedule { command } => {
                    commands::personal_schedule::execute(session, command).await?;
                }
                Commands::Scholarships { command } => {
                    commands::scholarships::execute(session, command).await?;
                }
            }
        }
    }

    Ok(())
}
