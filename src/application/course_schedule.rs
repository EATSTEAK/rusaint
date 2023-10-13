use crate::{
    define_elements,
    model::SemesterType,
    webdynpro::{
        element::{action::Button, complex::SapTable, layout::TabStrip, selection::ComboBox},
        error::WebDynproError,
    },
};

use super::USaintApplication;

pub struct CourseSchedule(USaintApplication);

#[allow(unused)]
impl<'a> CourseSchedule {
    const APP_NAME: &str = "ZCMW2100";

    define_elements! {
        PERIOD_YEAR: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERYR";
        PERIOD_ID: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERID";
        TABLE_ROWS: ComboBox<'a> = "ZCMW2100.ID_0001:VIW_MODULES.ROWS";
        TABSTRIP: TabStrip<'a> = "ZCMW2100.ID_0001:VIW_MAIN.MODULE_TABSTRIP";
        BUTTON_EDU: Button<'a> = "ZCMW2100.ID_0001:VIW_MAIN.BUTTON_EDU";
        MAIN_TABLE: SapTable<'a> = "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.SALV_WD_UIE_TABLE";
    }

    pub async fn new() -> Result<CourseSchedule, WebDynproError> {
        Ok(CourseSchedule(
            USaintApplication::new(Self::APP_NAME).await?,
        ))
    }

    fn semester_to_key(period: SemesterType) -> &'static str {
        match period {
            SemesterType::One => "090",
            SemesterType::Summer => "091",
            SemesterType::Two => "092",
            SemesterType::Winter => "093",
        }
    }

    pub async fn select_period(
        &mut self,
        year: &str,
        period: SemesterType,
    ) -> Result<(), WebDynproError> {
        let events = {
            let body = self.body();
            let period_year = Self::PERIOD_YEAR.from_body(body)?;
            let period_id = Self::PERIOD_ID.from_body(body)?;
            vec![
                period_year.select(year, false)?,
                period_id.select(Self::semester_to_key(period), false)?,
            ]
        };
        self.send_events(events).await
    }

    pub async fn select_rows(&mut self, row: u32) -> Result<(), WebDynproError> {
        let events = {
            let body = self.body();
            let table_rows = Self::TABLE_ROWS.from_body(body)?;
            vec![table_rows.select(row.to_string().as_str(), false)?]
        };
        self.send_events(events).await
    }

    pub async fn select_edu(&mut self) -> Result<(), WebDynproError> {
        let events = {
            let body = self.body();
            let tab_strip = Self::TABSTRIP.from_body(body)?;
            vec![tab_strip.tab_select("ZCMW2100.ID_0001:VIW_MAIN.TAB_EDU", 4, 0)?]
        };
        self.send_events(events).await
    }

    async fn search_edu(&mut self) -> Result<(), WebDynproError> {
        let events = {
            let body = self.body();
            let button_edu = Self::BUTTON_EDU.from_body(body)?;
            vec![button_edu.press()?]
        };
        self.send_events(events).await
    }

    pub async fn load_edu(&mut self) -> Result<(), WebDynproError> {
        self.select_edu().await?;
        self.search_edu().await?;
        Ok(())
    }

    pub fn read_edu_raw(&self) -> Result<SapTable, WebDynproError> {
        let body = self.body();
        let main_table = Self::MAIN_TABLE.from_body(body)?;
        Ok(main_table)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        application::course_schedule::CourseSchedule,
        webdynpro::element::{
            complex::sap_table::cell::{SapTableCell, SapTableCellWrapper},
            selection::list_box::{item::ListBoxItemWrapper, ListBoxWrapper},
            ElementWrapper,
        },
    };

    #[tokio::test]
    async fn examine_elements() {
        let app = CourseSchedule::new().await.unwrap();
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
        let app = CourseSchedule::new().await.unwrap();
        let period_id_combobox = CourseSchedule::PERIOD_ID.from_body(app.body()).unwrap();
        let listbox = period_id_combobox.item_list_box(app.body()).unwrap();
        match listbox {
            ListBoxWrapper::ListBoxPopup(listbox) => {
                for item in listbox.items() {
                    match item {
                        ListBoxItemWrapper::Item(item) => {
                            println!("value: {:?}, key: {:?}", item.value1(), item.key());
                        }
                        ListBoxItemWrapper::ActionItem(item) => {
                            println!("title: {:?}, text: {:?}", item.title(), item.text());
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
        app.load_edu().await.unwrap();
        let table = app.read_edu_raw().unwrap();
        if let Some(table) = table.table() {
            for row in table.with_header() {
                print!("row: ");
                for col in row.iter() {
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
