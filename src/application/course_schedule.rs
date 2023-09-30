use anyhow::Result;
use std::ops::{Deref, DerefMut};

use crate::webdynpro::element::{
    button::Button, combo_box::ComboBox, element_ref, sap_table::SapTable, tab_strip::TabStrip,
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

impl<'a> CourseSchedule {
    const APP_NAME: &str = "ZCMW2100";

    element_ref! {
        PERIOD_YEAR: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERYR",
        PERIOD_ID: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERID",
        TABLE_ROWS: ComboBox<'a> = "ZCMW2100.ID_0001:VIW_MODULES.ROWS",
        TABSTRIP: TabStrip<'a> = "ZCMW2100.ID_0001:VIW_MAIN.MODULE_TABSTRIP",
        BUTTON_EDU: Button<'a> = "ZCMW2100.ID_0001:VIW_MAIN.BUTTON_EDU",
        MAIN_TABLE: SapTable<'a> = "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.SALV_WD_UIE_TABLE"
    }

    pub async fn new() -> Result<CourseSchedule> {
        Ok(CourseSchedule(
            BasicUSaintApplication::new(Self::APP_NAME).await?,
        ))
    }

    pub async fn select_period(&mut self, year: u32, period: PeriodType) -> Result<()> {
        let events = {
            let body = self.body();
            let period_year = Self::PERIOD_YEAR.from_body(body)?;
            let period_id = Self::PERIOD_ID.from_body(body)?;
            vec![
                period_year.select(year.to_string().as_str(), false)?,
                period_id.select(period.to_string().as_str(), false)?,
            ]
        };
        self.send_events(events).await
    }

    pub async fn select_rows(&mut self, row: u32) -> Result<()> {
        let events = {
            let body = self.body();
            let table_rows = Self::TABLE_ROWS.from_body(body)?;
            vec![table_rows.select(row.to_string().as_str(), false)?]
        };
        self.send_events(events).await
    }

    pub async fn select_edu(&mut self) -> Result<()> {
        let events = {
            let body = self.body();
            let tab_strip = Self::TABSTRIP.from_body(body)?;
            vec![tab_strip.tab_select("ZCMW2100.ID_0001:VIW_MAIN.TAB_EDU", 4, 0)?]
        };
        self.send_events(events).await
    }

    async fn search_edu(&mut self) -> Result<()> {
        let events = {
            let body = self.body();
            let button_edu = Self::BUTTON_EDU.from_body(body)?;
            vec![button_edu.press()?]
        };
        self.send_events(events).await
    }

    pub async fn load_edu(&mut self) -> Result<()> {
        self.select_edu().await?;
        self.search_edu().await?;
        Ok(())
    }

    pub fn read_edu_raw(&self) -> Result<SapTable> {
        let body = self.body();
        let main_table = Self::MAIN_TABLE.from_body(body)?;
        Ok(main_table)
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

#[cfg(test)]
mod test {
    use crate::{
        application::course_schedule::CourseSchedule,
        webdynpro::element::{
            list_box::{ListBoxItemWrapper, ListBoxWrapper},
            sap_table::cell::{SapTableCell, SapTableCellWrapper},
            ElementWrapper,
        },
    };

    #[tokio::test]
    async fn initial_load() {
        let mut app = CourseSchedule::new().await.unwrap();
        app.load_placeholder().await.unwrap();
    }

    #[tokio::test]
    async fn examine_elements() {
        let mut app = CourseSchedule::new().await.unwrap();
        app.load_placeholder().await.unwrap();
        let ct_selector = scraper::Selector::parse("[ct]").unwrap();
        for elem_ref in app.body().document().select(&ct_selector) {
            let elem = ElementWrapper::dyn_elem(elem_ref);
            if let Ok(elem) = elem {
                println!("{:?}", elem);
            }
        }
    }

    #[tokio::test]
    async fn combobox_items() {
        let mut app = CourseSchedule::new().await.unwrap();
        app.load_placeholder().await.unwrap();
        let period_id_combobox = CourseSchedule::PERIOD_ID.from_body(app.body()).unwrap();
        let listbox = period_id_combobox.item_list_box(app.body()).unwrap();
        match listbox {
            ListBoxWrapper::ListBoxPopup(listbox) => {
                for item in listbox.items() {
                    match item {
                        ListBoxItemWrapper::Item(item) => {
                            println!("{:?}", item);
                        }
                        ListBoxItemWrapper::ActionItem(item) => {
                            println!("{:?}", item);
                        }
                    }
                }
            }
            _ => {
                panic!("Unknown Listbox type {:?}", listbox);
            }
        }
        assert!(true);
    }

    #[tokio::test]
    async fn table_test() {
        let mut app = CourseSchedule::new().await.unwrap();
        app.load_placeholder().await.unwrap();
        app.load_edu().await.unwrap();
        let table = app.read_edu_raw().unwrap();
        if let Some(table) = table.table() {
            for row in table {
                print!("row: ");
                for col in row {
                    match col {
                        SapTableCellWrapper::Header(cell) => {
                            let content = cell.content();
                            print!("Header: ");
                            if let Some(elem) = content {
                                print!("{:?}, ", elem);
                            }
                        }
                        SapTableCellWrapper::Normal(cell) => {
                            let content = cell.content();
                            if let Some(elem) = content {
                                print!("{:?}, ", elem);
                            }
                        }
                        _ => {
                            print!("{:?}, ", col);
                        }
                    }
                }
                println!("");
            }
        }
        assert!(true);
    }
}
