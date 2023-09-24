use anyhow::Result;
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

    async fn test_button(&mut self) -> Result<()> {
        self.load_placeholder().await?;
        let events = {
            let body = self.body();
            let link = Self::LINK_TO_BUTTON.from_body(body)?;
            let btn = Self::TEST_BUTTON.from_body(body)?;
            vec![link.activate(false, false)?, btn.press()?]
        };
        self.send_events(events).await?;
        Ok(())
    }
}

#[tokio::test]
async fn test_button_events() {
    if let Ok(token) = std::env::var("SSO_TOKEN") {
        let mut suite = EventTestSuite::new(token.as_str()).await.unwrap();
        suite.test_button().await.unwrap();
    } else {
        panic!("No SSO_TOKEN Enivrionment Variable supplied, terminate.")
    }
}
