use rusaint::{
    application::USaintClientBuilder,
    define_elements,
    webdynpro::{
        element::{
            action::{Button, Link}, definition::ElementDefinition, text::TextView
        },
        error::WebDynproError,
    },
};

use crate::get_session;

use super::EventTestSuite;

impl<'a> EventTestSuite {
    define_elements! {
        /* Button Test */
        LINK_TO_BUTTON: Link<'a> = "WDR_TEST_EVENTS.ID_0001:MAIN.TOBUTTON";
        TEST_BUTTON: Button<'a> = "WDR_TEST_EVENTS.ID_0001:BUTTON.BUTTON1";
        TEST_BUTTON_TEXTVIEW: TextView<'a> = "WDR_TEST_EVENTS.ID_0001:BUTTON.TEXTVIEW";
    }

    async fn test_button(&mut self) -> Result<(), WebDynproError> {
        let load_btn_pane = {
            let body = self.body();
            let link = Self::LINK_TO_BUTTON.from_body(body)?;
            link.activate(false, false)?
        };
        self.process_event(false, load_btn_pane).await?;
        let btn_events = {
            let body = self.body();
            let btn = Self::TEST_BUTTON.from_body(body)?;
            btn.press()?
        };
        self.process_event(false, btn_events).await?;
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
    let mut suite = USaintClientBuilder::new()
        .session(session)
        .build_into::<EventTestSuite>()
        .await
        .unwrap();
    suite.test_button().await.unwrap();
}
