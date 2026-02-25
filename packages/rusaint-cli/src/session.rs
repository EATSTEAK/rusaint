use std::{
    fs::File,
    io::BufReader,
    path::Path,
    sync::Arc,
};

use rusaint::USaintSession;

pub async fn get_session(
    session_file: Option<&Path>,
) -> Result<Arc<USaintSession>, Box<dyn std::error::Error>> {
    if let Some(path) = session_file {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        Ok(Arc::new(USaintSession::from_json(reader)?))
    } else {
        dotenvy::dotenv().ok();
        let id = std::env::var("SSO_ID")
            .map_err(|_| "SSO_ID 환경변수가 설정되지 않았습니다.")?;
        let password = std::env::var("SSO_PASSWORD")
            .map_err(|_| "SSO_PASSWORD 환경변수가 설정되지 않았습니다.")?;
        Ok(Arc::new(USaintSession::with_password(&id, &password).await?))
    }
}
