use std::{fs::File, io::BufReader, path::Path, sync::Arc};

use rusaint::USaintSession;

pub async fn get_session(
    session_file: Option<&Path>,
    allow_anonymous: bool,
) -> Result<Arc<USaintSession>, Box<dyn std::error::Error>> {
    if let Some(path) = session_file {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        Ok(Arc::new(USaintSession::from_json(reader)?))
    } else {
        let id = std::env::var("SSO_ID");
        let password = std::env::var("SSO_PASSWORD");
        match (id, password) {
            (Ok(id), Ok(password)) => Ok(Arc::new(
                USaintSession::with_password(&id, &password).await?,
            )),
            _ if allow_anonymous => Ok(Arc::new(USaintSession::anonymous())),
            _ => Err("SSO_ID 또는 SSO_PASSWORD 환경변수가 설정되지 않았습니다.".into()),
        }
    }
}
