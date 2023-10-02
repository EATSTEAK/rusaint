use anyhow::Result;
use rusaint::{
    element_ref,
    webdynpro::element::{button::Button, link::Link, text_view::TextView},
};

use crate::get_session;

use super::EventTestSuite;

impl<'a> EventTestSuite {
    element_ref! {
        /* Button Test */
        LINK_TO_BUTTON: Link<'a> = "WDR_TEST_EVENTS.ID_0001:MAIN.TOBUTTON",
        TEST_BUTTON: Button<'a> = "WDR_TEST_EVENTS.ID_0001:BUTTON.BUTTON1",
        TEST_BUTTON_TEXTVIEW: TextView<'a> = "WDR_TEST_EVENTS.ID_0001:BUTTON.TEXTVIEW",
    }

    async fn test_button(&mut self) -> Result<()> {
        let load_btn_pane = {
            let body = self.body();
            let link = Self::LINK_TO_BUTTON.from_body(body)?;
            vec![link.activate(false, false)?]
        };
        self.send_events(load_btn_pane).await?;
        let btn_events = {
            let body = self.body();
            let btn = Self::TEST_BUTTON.from_body(body)?;
            vec![btn.press()?]
        };
        self.send_events(btn_events).await?;
        assert_eq!(
            Self::TEST_BUTTON_TEXTVIEW.from_body(self.body())?.text(),
            "An action has been triggered."
        );
        Ok(())
    }
}

#[tokio::test]
async fn test_button_events() {
    let session = get_session().await.unwrap();
    let mut suite = EventTestSuite::new(session).await.unwrap();
    suite.test_button().await.unwrap();
}
