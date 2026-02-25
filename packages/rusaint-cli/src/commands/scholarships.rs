use std::{path::Path, sync::Arc};

use clap::Subcommand;
use rusaint::{
    USaintSession, application::scholarships::ScholarshipsApplication, client::USaintClientBuilder,
};

use crate::output::{OutputFormat, write_output};

#[derive(Subcommand)]
pub enum ScholarshipsCommands {
    /// 장학금 수혜 내역 조회
    List,
}

pub async fn execute(
    session: Arc<USaintSession>,
    command: ScholarshipsCommands,
    format: &OutputFormat,
    output: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<ScholarshipsApplication>()
        .await?;

    match command {
        ScholarshipsCommands::List => {
            let result = app.scholarships().await?;
            write_output(format, output, &result)?;
        }
    }

    Ok(())
}
