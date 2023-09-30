use anyhow::Result;
use std::ops::{Deref, DerefMut};

use super::BasicUSaintApplication;

pub struct StudentInformation(BasicUSaintApplication);

impl Deref for StudentInformation {
    type Target = BasicUSaintApplication;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for StudentInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl StudentInformation {
    const APP_NAME: &str = "ZCMW1001n";

    pub async fn new(id: &str, token: &str) -> Result<StudentInformation> {
        Ok(StudentInformation(
            BasicUSaintApplication::with_auth(Self::APP_NAME, id, token).await?,
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        application::student_information::StudentInformation, utils::obtain_ssu_sso_token,
        webdynpro::element::ElementWrapper,
    };
    use dotenv::dotenv;

    #[tokio::test]
    async fn examine_elements() {
        dotenv().ok();
        let id = std::env::var("SSO_ID").unwrap();
        let password = std::env::var("SSO_PASSWORD").unwrap();
        let token = obtain_ssu_sso_token(&id, &password).await.unwrap();
        let mut app = StudentInformation::new(&id, &token).await.unwrap();
        app.load_placeholder().await.unwrap();
        let ct_selector = scraper::Selector::parse("[ct]").unwrap();
        for elem_ref in app.body().document().select(&ct_selector) {
            let elem = ElementWrapper::dyn_elem(elem_ref);
            if let Ok(elem) = elem {
                println!("{:?}", elem);
            }
        }
    }
}
