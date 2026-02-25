use std::sync::Arc;

use clap::Subcommand;
use rusaint::{USaintSession, application::chapel::ChapelApplication, client::USaintClientBuilder};

use crate::{output::write_json, types::SemesterType};

#[derive(Subcommand)]
pub enum ChapelCommands {
    /// 채플 정보 조회
    Information {
        #[arg(short = 'y', long)]
        year: u32,
        #[arg(short = 's', long)]
        semester: SemesterType,
    },
}

pub async fn execute(
    session: Arc<USaintSession>,
    command: ChapelCommands,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<ChapelApplication>()
        .await?;

    match command {
        ChapelCommands::Information { year, semester } => {
            let result = app.information(year, *semester).await?;
            write_json(&format!("chapel_{year}_{semester}"), &result)?;
        }
    }

    Ok(())
}
