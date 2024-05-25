use crate::{webdynpro::client::body::Body, RusaintError};

use super::{USaintApplication, USaintClient};

pub struct StudentInformation {
    client: USaintClient,
}

impl USaintApplication for StudentInformation {
    const APP_NAME: &'static str = "ZCMW2100";

    fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
        if client.name() != Self::APP_NAME {
            Err(RusaintError::InvalidClientError)
        } else {
            Ok(Self { client })
        }
    }
}

impl StudentInformation {
    fn body(&self) -> &Body {
        self.client.body()
    }
}

#[cfg(test)]
mod test {
    use serial_test::serial;

    use crate::{
        application::{student_information::StudentInformation, USaintClientBuilder},
        global_test_utils::get_session,
        session::USaintSession,
        webdynpro::element::ElementWrapper,
    };

    #[tokio::test]
    #[serial]
    async fn examine_elements() {
        let session = get_session().await.unwrap();
        let app = USaintClientBuilder::new()
            .session(session)
            .build_into::<StudentInformation>()
            .await
            .unwrap();
        let ct_selector = scraper::Selector::parse("[ct]").unwrap();
        for elem_ref in app.body().document().select(&ct_selector) {
            let elem = ElementWrapper::dyn_element(elem_ref);
            if let Ok(elem) = elem {
                println!("{:?}", elem);
            }
        }
    }
}
