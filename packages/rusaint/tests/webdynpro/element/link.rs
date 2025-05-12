use super::{EventTestSuite, get_event_test_suite};
use rusaint::webdynpro::element::parser::ElementParser;
use rusaint::{
    define_elements,
    webdynpro::{
        element::{action::Link, text::TextView},
        error::WebDynproError,
    },
};
use test_log::test;

impl<'a> EventTestSuite {
    define_elements! {
        /* Link Test */
        LINK_TO_LINKTOACTION: Link<'a> = "WDR_TEST_EVENTS.ID_0001:MAIN.TOLINKTOACTION";
        TEST_LINK: Link<'a> = "WDR_TEST_EVENTS.ID_0001:LINKTOACTION.LINKTOACTION";
        TEST_LINK_TEXTVIEW: TextView<'a> = "WDR_TEST_EVENTS.ID_0001:LINKTOACTION.TEXTVIEW";
    }

    async fn test_link(&mut self) -> Result<(), WebDynproError> {
        let load_link_pane = {
            let parser = ElementParser::new(self.body());
            let link = parser.element_from_def(&Self::LINK_TO_LINKTOACTION)?;
            link.activate(false, false)?
        };
        self.process_event(false, load_link_pane).await?;
        let link_events = {
            let parser = ElementParser::new(self.body());
            let link = parser.element_from_def(&Self::TEST_LINK)?;
            link.activate(false, false)?
        };
        self.process_event(false, link_events).await?;
        let parser = ElementParser::new(self.body());
        assert_eq!(
            parser.element_from_def(&Self::TEST_LINK_TEXTVIEW)?.text(),
            "An action has been triggered."
        );
        Ok(())
    }
}

#[test(tokio::test)]
async fn test_link_event() {
    let lock = get_event_test_suite().await.unwrap();
    let mut suite = lock.write().await;
    suite.test_link().await.unwrap();
}
