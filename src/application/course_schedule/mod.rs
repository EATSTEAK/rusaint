use crate::{
    application::course_schedule::model::{Lecture, LectureCategory},
    define_elements,
    model::SemesterType,
    webdynpro::{
        client::body::Body,
        command::element::{
            action::ButtonPressCommand, layout::TabStripTabSelectCommand,
            selection::ComboBoxSelectCommand,
        },
        element::{
            action::Button,
            complex::SapTable,
            definition::ElementDefinition,
            layout::{tab_strip::item::TabStripItem, TabStrip},
            selection::ComboBox,
            unknown::Unknown,
        },
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
    // 메인 요소
    define_elements! {
        PERIOD_YEAR: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERYR";
        PERIOD_ID: ComboBox<'a> = "ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERID";
        TABLE_ROWS: ComboBox<'a> = "ZCMW2100.ID_0001:VIW_MODULES.ROWS";
        TABSTRIP: TabStrip<'a> = "ZCMW2100.ID_0001:VIW_MAIN.MODULE_TABSTRIP";
        MAIN_TABLE: SapTable<'a> = "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.SALV_WD_UIE_TABLE";
    }

    // 학부전공별
    define_elements! {
        TAB_OTHERS: TabStripItem<'a> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_OTHERS";
        OTHERS_DDK_LV3: ComboBox<'a> = "ZCMW2100.ID_0001:VIW_TAB_OTHERS.DDK_LV3";
        OTHERS_DDK_LV4: ComboBox<'a> = "ZCMW2100.ID_0001:VIW_TAB_OTHERS.DDK_LV4";
        OTHERS_DDK_LV5: ComboBox<'a> = "ZCMW2100.ID_0001:VIW_TAB_OTHERS.DDK_LV5";
        SEARCH_OTHERS: Button<'a> = "ZCMW2100.ID_0001:VIW_TAB_OTHERS.BUTTON";
    }

    // 교양필수
    define_elements! {
        TAB_GENERAL_REQ: TabStripItem<'a> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_GENERAL_REQ";
        GENERAL_REQ_TYPE: ComboBox<'a> = "ZCMW2100.ID_0001:VIW_TAB_GENERAL_REQ.SM_OBJID";
        SEARCH_GENERAL_REQ: Button<'a> = "ZCMW2100.ID_0001:VIW_TAB_GENERAL_REQ.BUTTON_SEARCH";
    }

    // 교양선택
    define_elements! {
        TAB_GENERAL_OPT: TabStripItem<'a> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_GENERAL_OPT";
        GENERAL_OPT_DISCIPLINES: ComboBox<'a> = "ZCMW2100.ID_0001:VIW_TAB_GENERAL_OPT.DISCIPLINES";
        SEARCH_GENERAL_OPT: Button<'a> = "ZCMW2100.ID_0001:VIW_TAB_GENERAL_OPT.BUTTON_SEARCH";
    }

    // 채플
    define_elements! {
        TAB_CHAPEL: TabStripItem<'a> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_CHAPEL_REQ";
        CHAPEL_TYPE: ComboBox<'a> = "ZCMW2100.ID_0001:VIW_TAB_CHAPEL_REQ.SM_OBJID";
        SEARCH_CHAPEL: Button<'a> = "ZCMW2100.ID_0001:VIW_TAB_CHAPEL_REQ.BUTTON_SEARCH";
    }

    // 교직
    define_elements! {
        TAB_EDU: TabStripItem<'a> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_EDU";
        SEARCH_EDU: Button<'a> = "ZCMW2100.ID_0001:VIW_MAIN.BUTTON_EDU";
    }

    // 평생교육사
    define_elements! {
        TAB_LIFELONG: TabStripItem<'a> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_LIFELONG";
        SEARCH_LIFELONG: Button<'a> = "ZCMW2100.ID_0001:VIW_MAIN.BUTTON_LIFELONG";
    }

    // 일반선택
    define_elements! {
        TAB_ROTC_CYBER: TabStripItem<'a> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_ROTC_CYBER";
        SEARCH_ROTC_CYBER: Button<'a> = "ZCMW2100.ID_0001:VIW_MAIN.BUTTON_ROTC_CYBER";
    }

    // 대학원
    define_elements! {
        TAB_GRADUATE: TabStripItem<'a> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_GRADUATE";
        GRADUATE_DDK_LV3: ComboBox<'a> = "ZCMW2100.ID_0001:VIW_TAB_GRADUATE.DDK_LV3";
        GRADUATE_DDK_LV4: ComboBox<'a> = "ZCMW2100.ID_0001:VIW_TAB_GRADUATE.DDK_LV4";
        SEARCH_GRADUATE: Button<'a> = "ZCMW2100.ID_0001:VIW_TAB_GRADUATE.BUTTON";
    }

    // 연계전공
    define_elements! {
        TAB_YOMA: TabStripItem<'a> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_YOMA";
        COMBO_YOMA: ComboBox<'a> = "ZCMW2100.ID_0001:VIW_TAB_YOMA.CONNECT_MAJO";
        SEARCH_YOMA: Button<'a> = "ZCMW2100.ID_0001:VIW_TAB_YOMA.BUTTON_SEARCH";
    }

    // 융합전공
    define_elements! {
        TAB_UNMA: TabStripItem<'a> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_UNMA";
        COMBO_UNMA: ComboBox<'a> = "ZCMW2100.ID_0001:VIW_TAB_UNMA.CG_OBJID";
        SEARCH_UNMA: Button<'a> = "ZCMW2100.ID_0001:VIW_TAB_UNMA.BUTTON_SEARCH";
    }

    // 교수명검색
    define_elements! {
        TAB_PROFESSOR: TabStripItem<'a> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_PROFESSOR";
        COMBO_PROFESSOR: Unknown<'a> = "ZCMW2100.ID_0001:VIW_TAB_PROFESSOR.PROFESSOR"; // TODO: implement ComboBoxString
        SEARCH_PROFESSOR: Button<'a> = "ZCMW2100.ID_0001:VIW_TAB_PROFESSOR.BUTTON_SEARCH";
    }

    // 과목검색
    define_elements! {
        TAB_SEARCH: TabStripItem<'a> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_SEARCH";
        COMBO_SEARCH: Unknown<'a> = "ZCMW2100.ID_0001:VIW_TAB_SEARCH.SEARCH_TEXT";
        SEARCH_SEARCH: Button<'a> = "ZCMW2100.ID_0001:VIW_TAB_SEARCH.BUTTON_SEARCH";
    }

    // 타전공인정과목
    define_elements! {
        TAB_OTHER_GC: TabStripItem<'a> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_OTHER_GC";
        OTHER_GC_DDK_LV3: ComboBox<'a> = "ZCMW2100.ID_0001:VIW_TAB_OTHER_GC.DDK_LV3";
        OTHER_GC_DDK_LV4: ComboBox<'a> = "ZCMW2100.ID_0001:VIW_TAB_OTHER_GC.DDK_LV4";
        OTHER_GC_DDK_LV5: ComboBox<'a> = "ZCMW2100.ID_0001:VIW_TAB_OTHER_GC.DDK_LV5";
        SEARCH_OTHER_GC: Button<'a> = "ZCMW2100.ID_0001:VIW_TAB_OTHER_GC.BTN_OTHER_GC";
    }

    // 듀얼리스팅과목
    define_elements! {
        TAB_DUALLT_SM: TabStripItem<'a> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_DUALLT_SM";
        SEARCH_DUALLT: Button<'a> = "ZCMW2100.ID_0001:VIW_TAB_DUALLT_SM.BTN_DUALLT_SM";
    }

    // 숭실사이버대
    define_elements! {
        TAB_CYBER: TabStripItem<'a> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_CYBER";
        SEARCH_CYBER: Button<'a> = "ZCMW2100.ID_0001:VIW_MAIN.BTN_CYBER";
    }

    fn semester_to_key(period: SemesterType) -> &'static str {
        match period {
            SemesterType::One => "090",
            SemesterType::Summer => "091",
            SemesterType::Two => "092",
            SemesterType::Winter => "093",
        }
    }

    async fn select_period(
        &mut self,
        year: &str,
        period: SemesterType,
    ) -> Result<(), WebDynproError> {
        self.client
            .send(ComboBoxSelectCommand::new(Self::PERIOD_YEAR, year, false))
            .await?;
        self.client
            .send(ComboBoxSelectCommand::new(
                Self::PERIOD_ID,
                Self::semester_to_key(period),
                false,
            ))
            .await?;
        Ok(())
    }

    async fn select_rows(&mut self, row: u32) -> Result<(), WebDynproError> {
        self.client
            .send(ComboBoxSelectCommand::new(
                Self::TABLE_ROWS,
                row.to_string().as_str(),
                false,
            ))
            .await?;
        Ok(())
    }

    pub async fn find_lectures(
        &mut self,
        year: &str,
        period: SemesterType,
        lecture_category: LectureCategory,
    ) -> Result<impl Iterator<Item = Lecture>, WebDynproError> {
        unimplemented!();
        Ok(Vec::<Lecture>::with_capacity(0).into_iter())
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
            definition::ElementDefinition,
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
            let elem = ElementWrapper::dyn_element(elem_ref);
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
}

pub mod model;
