extern crate dotenv;

use dotenv::dotenv;

use anyhow::Result;
use rusaint::{
    element_ref,
    webdynpro::element::{button::Button, link::Link, text_view::TextView},
};

use super::EventTestSuite;

impl<'a> EventTestSuite {
    element_ref! {
        /* Button Test */
        LINK_TO_BUTTON: Link<'a> = "WDR_TEST_EVENTS.ID_0001:MAIN.TOBUTTON",
        TEST_BUTTON: Button<'a> = "WDR_TEST_EVENTS.ID_0001:BUTTON.BUTTON1",
        TEST_BUTTON_TEXTVIEW: TextView<'a> = "WDR_TEST_EVENTS.ID_0001:BUTTON.TEXTVIEW",
    }

    async fn test_button(&mut self) -> Result<()> {
        self.load_placeholder().await?;
        let events = {
            let body = self.body();
            let link = Self::LINK_TO_BUTTON.from_body(body)?;
            let btn = Self::TEST_BUTTON.from_body(body)?;
            vec![link.activate(false, false)?, btn.press()?]
        };
        self.send_events(events).await?;
        assert_eq!(
            Self::TEST_BUTTON_TEXTVIEW.from_body(self.body())?.text(),
            "An action has been triggered."
        );
        Ok(())
    }
}

#[tokio::test]
async fn test_button_events() {
    dotenv().ok();
    let id = std::env::var("SSO_ID").unwrap();
    let token = std::env::var("SSO_TOKEN").unwrap();
    let mut suite = EventTestSuite::new(id.as_str(), token.as_str())
        .await
        .unwrap();
    suite.test_button().await.unwrap();
}
