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

    /// .env 파일 경로. 지정 시 해당 파일에서 환경변수를 로드
    #[arg(long, global = true)]
    env_file: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(env_file) = &cli.env_file {
        dotenvy::from_path(env_file).map_err(|e| {
            format!(
                "env 파일을 로드할 수 없습니다 ({}): {e}",
                env_file.display()
            )
        })?;
    } else {
        dotenvy::dotenv().ok();
    }

    match cli.command {
        Commands::CreateSession(args) => {
            commands::create_session::execute(args).await?;
        }
        Commands::CourseSchedule { command } => {
            let session = session::get_session(cli.session_file.as_deref()).await?;
            commands::course_schedule::execute(session, command).await?;
        }
        Commands::StudentInfo { command } => {
            let session = session::get_session(cli.session_file.as_deref()).await?;
            commands::student_info::execute(session, command).await?;
        }
        Commands::Grades { command } => {
            let session = session::get_session(cli.session_file.as_deref()).await?;
            commands::grades::execute(session, command).await?;
        }
        Commands::ChapelInfo { command } => {
            let session = session::get_session(cli.session_file.as_deref()).await?;
            commands::chapel::execute(session, command).await?;
        }
        Commands::Registration { command } => {
            let session = session::get_session(cli.session_file.as_deref()).await?;
            commands::registration::execute(session, command).await?;
        }
        Commands::Graduation { command } => {
            let session = session::get_session(cli.session_file.as_deref()).await?;
            commands::graduation::execute(session, command).await?;
        }
        Commands::Assessment { command } => {
            let session = session::get_session(cli.session_file.as_deref()).await?;
            commands::assessment::execute(session, command).await?;
        }
        Commands::PersonalSchedule { command } => {
            let session = session::get_session(cli.session_file.as_deref()).await?;
            commands::personal_schedule::execute(session, command).await?;
        }
        Commands::Scholarships { command } => {
            let session = session::get_session(cli.session_file.as_deref()).await?;
            commands::scholarships::execute(session, command).await?;
        }
    }

    Ok(())
}
