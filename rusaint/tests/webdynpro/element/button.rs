use crate::get_session;
use rusaint::webdynpro::element::parser::ElementParser;
use rusaint::{
    application::USaintClientBuilder,
    define_elements,
    webdynpro::{
        element::{
            action::{Button, Link},
            definition::ElementDefinition,
            text::TextView,
        },
        error::WebDynproError,
    },
};

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
            let parser = ElementParser::new(self.body())?;
            let link = parser.element_from_def(&Self::LINK_TO_BUTTON)?;
            link.activate(false, false)?
        };
        self.process_event(false, load_btn_pane).await?;
        let btn_events = {
            let parser = ElementParser::new(self.body())?;
            let btn = parser.element_from_def(&Self::TEST_BUTTON)?;
            btn.press()?
        };
        self.process_event(false, btn_events).await?;
        let parser = ElementParser::new(self.body())?;
        assert_eq!(
            parser
                .element_from_def(&Self::TEST_BUTTON_TEXTVIEW)?
                .text()?,
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
