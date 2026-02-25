use std::{fs::File, io::BufWriter, path::PathBuf};

use clap::Args;
use rusaint::USaintSession;

#[derive(Args)]
pub struct CreateSessionArgs {
    /// 출력 파일 경로
    #[arg(short, long, default_value = "session.json")]
    pub output: PathBuf,
}

pub async fn execute(args: CreateSessionArgs) -> Result<(), Box<dyn std::error::Error>> {
    let id = std::env::var("SSO_ID").map_err(|_| "SSO_ID 환경변수가 설정되지 않았습니다.")?;
    let password = std::env::var("SSO_PASSWORD")
        .map_err(|_| "SSO_PASSWORD 환경변수가 설정되지 않았습니다.")?;

    let session = USaintSession::with_password(&id, &password).await?;
    let file = File::create(&args.output)?;
    let mut writer = BufWriter::new(file);
    session.save_to_json(&mut writer)?;

    eprintln!("세션 파일이 생성되었습니다: {}", args.output.display());
    Ok(())
}
