use std::ops::{Deref, DerefMut};

use scraper::Selector;

use crate::webdynpro::{
    element::{button::Button, combo_box::ComboBox, tab_strip::TabStrip, ElementDef},
    error::ClientError,
};

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
    const PERIOD_YEAR: ElementDef<'_, ComboBox<'_>> =
        ElementDef::new("ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERYR");
    const PERIOD_ID: ElementDef<'_, ComboBox<'_>> =
        ElementDef::new("ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERID");
    const TABLE_ROWS: ElementDef<'_, ComboBox<'_>> =
        ElementDef::new("ZCMW2100.ID_0001:VIW_MODULES.ROWS");
    const TABSTRIP: ElementDef<'_, TabStrip<'_>> =
        ElementDef::new("ZCMW2100.ID_0001:VIW_MAIN.MODULE_TABSTRIP");
    const BUTTON_EDU: ElementDef<'_, Button<'_>> =
        ElementDef::new("ZCMW2100.ID_0001:VIW_MAIN.BUTTON_EDU");
    pub async fn new() -> Result<CourseSchedule, ClientError> {
        Ok(CourseSchedule(
            BasicUSaintApplication::new(Self::APP_NAME).await?,
        ))
    }

    pub async fn select_period(
        &mut self,
        year: u32,
        period: PeriodType,
    ) -> Result<(), ClientError> {
        let body = self.body();
        let period_year = Self::PERIOD_YEAR.elem(body)?;
        let period_id = Self::PERIOD_ID.elem(body)?;
        self.send_events(vec![
            period_year.select(year.to_string().as_str(), false)?,
            period_id.select(period.to_string().as_str(), false)?,
        ])
        .await
    }

    pub async fn select_rows(&mut self, row: u32) -> Result<(), ClientError> {
        let body = self.body();
        let table_rows = Self::TABLE_ROWS.elem(body)?;
        self.send_events(vec![table_rows.select(row.to_string().as_str(), false)?])
            .await
    }

    pub async fn select_edu(&mut self) -> Result<(), ClientError> {
        let body = self.body();
        let tab_strip = Self::TABSTRIP.elem(body)?;
        println!("Parsed TabStrip");
        self.send_events(vec![tab_strip.tab_select(
            "ZCMW2100.ID_0001:VIW_MAIN.TAB_EDU",
            4,
            0,
        )?])
        .await
    }

    async fn search_edu(&mut self) -> Result<(), ClientError> {
        let body = self.body();
        let button_edu = Self::BUTTON_EDU.elem(body)?;
        self.send_events(vec![button_edu.press()?]).await
    }

    pub async fn load_edu(&mut self) -> Result<(), ClientError> {
        self.select_edu().await?;
        let html = self.body().document();
        let selector = Selector::parse(r#"[ct="B"]"#).unwrap();
        let elements: Vec<Option<&str>> = html.select(&selector).map(|elref| {
            elref.value().id()
        }).collect();
        self.search_edu().await?;
        Ok(())
    }
}

pub enum PeriodType {
    One,
    Summer,
    Two,
    Winter,
}

impl ToString for PeriodType {
    fn to_string(&self) -> String {
        match self {
            PeriodType::One => "090",
            PeriodType::Summer => "091",
            PeriodType::Two => "092",
            PeriodType::Winter => "093",
        }
        .to_owned()
    }
}
