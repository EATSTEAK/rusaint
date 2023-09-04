use std::ops::{Deref, DerefMut};

use crate::webdynpro::{application::client::WDClientError, element::{button::Button, combo_box::ComboBox, custom::Custom, tab_strip::TabStrip}};

use super::BasicUSaintApplication;

pub struct CourseSchedule(BasicUSaintApplication);

impl Deref for CourseSchedule {
    type Target = BasicUSaintApplication;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for CourseSchedule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CourseSchedule {
    const APP_NAME: &str = "ZCMW2100";
    const PERIOD_YEAR: ComboBox<'_> = ComboBox::new("ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERYR");
    const PERIOD_ID: ComboBox<'_> = ComboBox::new("ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERID");
    const TABLE_ROWS: ComboBox<'_> = ComboBox::new("ZCMW2100.ID_0001:VIW_MODULES.ROWS");
    const TABSTRIP: TabStrip<'_> = TabStrip::new("ZCMW2100.ID_0001:VIW_MAIN.MODULE_TABSTRIP");
    const BUTTON_EDU: Button<'_> = Button::new("ZCMW2100.ID_0001:VIW_MAIN.BUTTON_EDU");
    pub async fn new() -> Result<CourseSchedule, WDClientError> {
        Ok(CourseSchedule(BasicUSaintApplication::new(Self::APP_NAME).await?))
    }

    pub async fn select_period(&mut self, year: u32, period: PeriodType) -> Result<(), WDClientError> {
        self.event_queue().extend([
            Self::PERIOD_YEAR.select(year.to_string().as_str(), false),
            BasicUSaintApplication::SSR_FORM.request(false, "", "", false, false)
            ]);
        self.send_event().await?;
        self.event_queue().extend([
            Self::PERIOD_ID.select(period.to_string().as_str(), false),
            BasicUSaintApplication::SSR_FORM.request(false, "", "", false, false)
        ]);
        self.send_event().await
    }

    pub async fn select_rows(&mut self, row: u32) -> Result<(), WDClientError> {
        self.event_queue().extend([
            Self::TABLE_ROWS.select("500", false),
            BasicUSaintApplication::SSR_FORM.request(false, "", "", false, false)
        ]);
        self.send_event().await
    }

    async fn search_edu(&mut self) -> Result<(), WDClientError> {
        self.event_queue().extend([
            Self::TABSTRIP.tab_select("ZCMW2100.ID_0001:VIW_MAIN.TAB_EDU", 4, 0),
            BasicUSaintApplication::SSR_FORM.request(false, "", "", false, false)
        ]);
        self.send_event().await?;
        self.event_queue().extend([
            Self::BUTTON_EDU.press(),
            BasicUSaintApplication::SSR_FORM.request(false, "", "", false, false)
        ]);
        self.send_event().await
    }

    pub async fn load_edu(&mut self) -> Result<(), WDClientError> {
        self.search_edu().await?;
        Ok(())
    }
}

pub enum PeriodType {
    One,
    Summer,
    Two,
    Winter
}

impl ToString for PeriodType {
    fn to_string(&self) -> String {
        match self {
            PeriodType::One => "090",
            PeriodType::Summer => "091",
            PeriodType::Two => "092",
            PeriodType::Winter => "093"
        }.to_owned()
    }
}