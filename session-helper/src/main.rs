use std::io::stdout;

use dotenvy::dotenv;
use eyre::Result;
use rusaint::USaintSession;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let id = std::env::var("SSO_ID")?;
    let password = std::env::var("SSO_PASSWORD")?;

    let session = USaintSession::with_password(&id, &password).await?;

    let mut writer = stdout();

    session.save_to_json(&mut writer)?;
    Ok(())
}
