use std::ops::{Deref, DerefMut};

use rusaint::{definitions::BasicUSaintApplication, element_ref};

pub(crate) struct EventTestSuite(BasicUSaintApplication);

impl Deref for EventTestSuite {
    type Target = BasicUSaintApplication;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for EventTestSuite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> EventTestSuite {
    const APP_NAME: &str = "WDR_TEST_EVENTS";

    element_ref! {
        PERIOD_YEAR: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERYR",
        PERIOD_ID: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERID",
        TABLE_ROWS: ComboBox<'a> = "ZCMW2100.ID_0001:VIW_MODULES.ROWS",
        TABSTRIP: TabStrip<'a> = "ZCMW2100.ID_0001:VIW_MAIN.MODULE_TABSTRIP",
        BUTTON_EDU: Button<'a> = "ZCMW2100.ID_0001:VIW_MAIN.BUTTON_EDU",
        MAIN_TABLE: SapTable<'a> = "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.SALV_WD_UIE_TABLE"
    }

    pub async fn new() -> Result<EventTestSuite> {
        Ok(EventTestSuite(
            BasicUSaintApplication::new(Self::APP_NAME).await?,
        ))
    }
}
