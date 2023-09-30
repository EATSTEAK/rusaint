use anyhow::Result;
use std::ops::{Deref, DerefMut};

use crate::{element_ref, webdynpro::element::popup_window::PopupWindow};

use super::BasicUSaintApplication;

pub struct CourseGrades(BasicUSaintApplication);

impl Deref for CourseGrades {
    type Target = BasicUSaintApplication;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for CourseGrades {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> CourseGrades {
    const APP_NAME: &str = "ZCMB3W0017";

    element_ref!(
        POPUP_WDW1: PopupWindow<'a> = "WDWL1"
    );

    pub async fn new(id: &str, token: &str) -> Result<CourseGrades> {
        Ok(CourseGrades(
            BasicUSaintApplication::with_auth(Self::APP_NAME, id, token).await?,
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        application::course_grades::CourseGrades,
        utils::obtain_ssu_sso_token,
        webdynpro::element::{Element, ElementWrapper},
    };
    use dotenv::dotenv;

    #[tokio::test]
    async fn children_test() {
        dotenv().ok();
        let id = std::env::var("SSO_ID").unwrap();
        let password = std::env::var("SSO_PASSWORD").unwrap();
        let token = obtain_ssu_sso_token(&id, &password).await.unwrap();
        let mut app = CourseGrades::new(&id, &token).await.unwrap();
        app.load_placeholder().await.unwrap();
        let popup_window = CourseGrades::POPUP_WDW1.from_body(app.body()).unwrap();
        println!("{:?}", popup_window.children());
    }

    #[tokio::test]
    async fn examine_elements() {
        dotenv().ok();
        let id = std::env::var("SSO_ID").unwrap();
        let password = std::env::var("SSO_PASSWORD").unwrap();
        let token = obtain_ssu_sso_token(&id, &password).await.unwrap();
        let mut app = CourseGrades::new(&id, &token).await.unwrap();
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
