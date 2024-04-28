use crate::{
    define_elements,
    model::SemesterType,
    webdynpro::{
        client::body::Body,
        element::{action::Button, complex::SapTable, layout::TabStrip, selection::ComboBox},
        error::WebDynproError,
    },
    RusaintError,
};

use super::{USaintApplication, USaintClient};

pub struct CourseSchedule {
    client: USaintClient,
}

impl USaintApplication for CourseSchedule {
    const APP_NAME: &'static str = "ZCMW2100";

    fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
        if client.name() != Self::APP_NAME {
            Err(RusaintError::InvalidClientError)
        } else {
            Ok(Self { client })
        }
    }
}

#[allow(unused)]
impl<'a> CourseSchedule {
    define_elements! {
        PERIOD_YEAR: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERYR";
        PERIOD_ID: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERID";
        TABLE_ROWS: ComboBox<'a> = "ZCMW2100.ID_0001:VIW_MODULES.ROWS";
        TABSTRIP: TabStrip<'a> = "ZCMW2100.ID_0001:VIW_MAIN.MODULE_TABSTRIP";
        BUTTON_EDU: Button<'a> = "ZCMW2100.ID_0001:VIW_MAIN.BUTTON_EDU";
        MAIN_TABLE: SapTable<'a> = "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.SALV_WD_UIE_TABLE";
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
            let body = self.client.body();
            let period_year = Self::PERIOD_YEAR.from_body(body)?;
            let period_id = Self::PERIOD_ID.from_body(body)?;
            vec![
                period_year.select(year, false)?,
                period_id.select(Self::semester_to_key(period), false)?,
            ]
        };
        for event in events {
            self.client.process_event(false, event).await?;
        }
        Ok(())
    }

    pub async fn select_rows(&mut self, row: u32) -> Result<(), WebDynproError> {
        let events = {
            let body = self.client.body();
            let table_rows = Self::TABLE_ROWS.from_body(body)?;
            table_rows.select(row.to_string().as_str(), false)?
        };
        self.client.process_event(false, events).await?;
        Ok(())
    }

    pub async fn select_edu(&mut self) -> Result<(), WebDynproError> {
        let events = {
            let body = self.client.body();
            let tab_strip = Self::TABSTRIP.from_body(body)?;
            tab_strip.tab_select("ZCMW2100.ID_0001:VIW_MAIN.TAB_EDU", 4, 0)?
        };
        self.client.process_event(false, events).await?;
        Ok(())
    }

    async fn search_edu(&mut self) -> Result<(), WebDynproError> {
        let events = {
            let body = self.client.body();
            let button_edu = Self::BUTTON_EDU.from_body(body)?;
            button_edu.press()?
        };
        self.client.process_event(false, events).await?;
        Ok(())
    }

    pub async fn load_edu(&mut self) -> Result<(), WebDynproError> {
        self.select_edu().await?;
        self.search_edu().await?;
        Ok(())
    }

    pub fn read_edu_raw(&self) -> Result<SapTable, WebDynproError> {
        let main_table = Self::MAIN_TABLE.from_body(self.client.body())?;
        Ok(main_table)
    }

    fn body(&self) -> &Body {
        self.client.body()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        application::{course_schedule::CourseSchedule, USaintClientBuilder},
        webdynpro::element::{
            complex::sap_table::cell::{SapTableCell, SapTableCellWrapper},
            selection::list_box::{item::ListBoxItemWrapper, ListBoxWrapper},
            ElementWrapper,
        },
    };

    #[tokio::test]
    async fn examine_elements() {
        let app = USaintClientBuilder::new()
            .build_into::<CourseSchedule>()
            .await
            .unwrap();
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
        let app = USaintClientBuilder::new()
            .build_into::<CourseSchedule>()
            .await
            .unwrap();
        let period_id_combobox = CourseSchedule::PERIOD_ID.from_body(app.body()).unwrap();
        let listbox = period_id_combobox.item_list_box(app.body()).unwrap();
        match listbox {
            ListBoxWrapper::ListBoxPopup(listbox) => {
                for item in listbox.list_box().items() {
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
        let mut app = USaintClientBuilder::new()
            .build_into::<CourseSchedule>()
            .await
            .unwrap();
        app.load_edu().await.unwrap();
        let table = app.read_edu_raw().unwrap();
        if let Ok(table) = table.table() {
            for row in table.with_header() {
                print!("row: ");
                for col in row.iter_value(app.body()) {
                    match col {
                        Ok(SapTableCellWrapper::Header(cell)) => {
                            let content = cell.content();
                            print!("Header: ");
                            if let Some(elem) = content {
                                print!("{:?}, ", elem);
                            }
                        }
                        Ok(SapTableCellWrapper::Normal(cell)) => {
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
