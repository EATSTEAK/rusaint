use rusaint::{
    element_ref,
    webdynpro::element::{button::Button, link::Link},
};

use super::EventTestSuite;

impl<'a> EventTestSuite {
    element_ref! {
        /* Button Test */
        LINK_TO_BUTTON: Link<'a> = "WDR_TEST_EVENTS.ID_0001:MAIN.TOBUTTON",
        TEST_BUTTON: Button<'a> = "WDR_TEST_EVENTS.ID_0001:BUTTON.BUTTON1",
    }

    async fn test_button() {}
}

#[tokio::test]
async fn test_button_events() {
    if let Some(token) = std::env::var("SSO_TOKEN") {
        let suite = EventTestSuite::new(token).await.unwrap();
        suite.test_button()
    } else {
        panic!("No SSO_TOKEN Enivrionment Variable supplied, terminate.")
    }
}
