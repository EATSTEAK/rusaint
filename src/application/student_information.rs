use std::{
    sync::Arc,
};

use crate::{session::USaintSession, webdynpro::error::WebDynproError};

use super::USaintApplication;

pub struct StudentInformation(USaintApplication);

#[allow(unused)]
impl StudentInformation {
    const APP_NAME: &str = "ZCMW1001n";

    pub async fn new(session: Arc<USaintSession>) -> Result<StudentInformation, WebDynproError> {
        Ok(StudentInformation(
            USaintApplication::with_session(Self::APP_NAME, session).await?,
        ))
    }
}

#[cfg(test)]
mod test {
    use anyhow::{Error, Result};
    use std::sync::{Arc, OnceLock};

    use crate::{
        application::student_information::StudentInformation, session::USaintSession,
        webdynpro::element::ElementWrapper,
    };
    use dotenv::dotenv;

    static SESSION: OnceLock<Arc<USaintSession>> = OnceLock::new();

    async fn get_session() -> Result<Arc<USaintSession>> {
        if let Some(session) = SESSION.get() {
            Ok(session.to_owned())
        } else {
            dotenv().ok();
            let id = std::env::var("SSO_ID").unwrap();
            let password = std::env::var("SSO_PASSWORD").unwrap();
            let session = USaintSession::with_password(&id, &password).await?;
            let _ = SESSION.set(Arc::new(session));
            SESSION
                .get()
                .and_then(|arc| Some(arc.to_owned()))
                .ok_or(Error::msg("Session is not initsiated"))
        }
    }

    #[tokio::test]
    async fn examine_elements() {
        let session = get_session().await.unwrap();
        let app = StudentInformation::new(session).await.unwrap();
        let ct_selector = scraper::Selector::parse("[ct]").unwrap();
        for elem_ref in app.body().document().select(&ct_selector) {
            let elem = ElementWrapper::dyn_elem(elem_ref);
            if let Ok(elem) = elem {
                println!("{:?}", elem);
            }
        }
    }
}
