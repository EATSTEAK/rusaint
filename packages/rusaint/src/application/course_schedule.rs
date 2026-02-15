use crate::application::course_schedule::model::LectureDetail;
use crate::application::course_schedule::utils::{
    combo_box_items, select_lv1, select_lv2, select_tab,
};
use crate::application::utils::sap_table::try_table_into_with_scroll;
use crate::application::utils::semester::get_selected_semester;
use crate::client::{USaintApplication, USaintClient};
use crate::{
    ApplicationError, RusaintError,
    application::course_schedule::model::{Lecture, LectureCategory},
    model::SemesterType,
};
use scraper::Selector;
use wdpe::command::WebDynproCommandExecutor;
use wdpe::element::definition::ElementDefinition as _;
use wdpe::element::layout::tab_strip::item::TabStripItem;
use wdpe::element::parser::ElementParser;
use wdpe::{
    body::Body,
    command::element::{complex::SapTableBodyCommand, selection::ComboBoxSelectEventCommand},
    define_elements,
    element::{
        Element, ElementDefWrapper, ElementWrapper,
        complex::{
            SapTable,
            sap_table::cell::{SapTableCell, SapTableCellWrapper},
        },
        layout::{PopupWindow, TabStrip},
        selection::ComboBox,
    },
    error::{ElementError, WebDynproError},
    event::Event,
};

/// [강의시간표](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW2100)
#[derive(Debug)]
pub struct CourseScheduleApplication {
    client: USaintClient,
}

impl USaintApplication for CourseScheduleApplication {
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
impl<'app> CourseScheduleApplication {
    // 메인 요소
    define_elements! {
        PERIOD_YEAR: ComboBox<'app> = "ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERYR";
        PERIOD_ID: ComboBox<'app> = "ZCMW_PERIOD_RE.ID_A61C4ED604A2BFC2A8F6C6038DE6AF18:VIW_MAIN.PERID";
        TABLE_ROWS: ComboBox<'app> = "ZCMW2100.ID_0001:VIW_MODULES.ROWS";
        TABSTRIP: TabStrip<'app> = "ZCMW2100.ID_0001:VIW_MAIN.MODULE_TABSTRIP";
        MAIN_TABLE: SapTable<'app> = "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.SALV_WD_UIE_TABLE";
    }

    fn semester_to_key(semester: SemesterType) -> &'static str {
        match semester {
            SemesterType::One => "090",
            SemesterType::Summer => "091",
            SemesterType::Two => "092",
            SemesterType::Winter => "093",
        }
    }

    async fn select_semester(
        &mut self,
        parser: &ElementParser,
        year: &str,
        semester: SemesterType,
    ) -> Result<(), WebDynproError> {
        let year_select_event = parser.read(ComboBoxSelectEventCommand::new(
            Self::PERIOD_YEAR,
            year,
            false,
        ))?;
        self.client.process_event(false, year_select_event).await?;
        let semester_select_event = parser.read(ComboBoxSelectEventCommand::new(
            Self::PERIOD_ID,
            Self::semester_to_key(semester),
            false,
        ))?;
        self.client
            .process_event(false, semester_select_event)
            .await?;
        Ok(())
    }

    async fn select_rows(
        &mut self,
        parser: &ElementParser,
        row: u32,
    ) -> Result<(), WebDynproError> {
        let event = parser.read(ComboBoxSelectEventCommand::new(
            Self::TABLE_ROWS,
            row.to_string().as_str(),
            false,
        ))?;
        self.client.process_event(false, event).await?;
        Ok(())
    }

    fn body(&self) -> &Body {
        self.client.body()
    }

    /// 현재 페이지에 선택된 년도와 학기를 가져옵니다. 최초 로드 시 현재 학기를 가져올 가능성이 있습니다.
    /// 하지만 이 애플리케이션의 다른 함수를 호출하여 한번 정보를 가져왔다면 마지막으로 가져온 정보의 학기가 반환되므로 주의하여야 하며, 신뢰할 수 있는 현재 학기의 원천으로 사용되어서는 안됩니다.
    pub fn get_selected_semester(&self) -> Result<(u32, SemesterType), RusaintError> {
        Ok(get_selected_semester(
            &self.client,
            &Self::PERIOD_YEAR,
            &Self::PERIOD_ID,
        )?)
    }

    define_elements! {
        TAB_OTHERS: TabStripItem<'app> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_OTHERS";
        OTHERS_DDK_LV3: ComboBox<'app> = "ZCMW2100.ID_0001:VIW_TAB_OTHERS.DDK_LV3";
        OTHERS_DDK_LV4: ComboBox<'app> = "ZCMW2100.ID_0001:VIW_TAB_OTHERS.DDK_LV4";
        OTHERS_DDK_LV5: ComboBox<'app> = "ZCMW2100.ID_0001:VIW_TAB_OTHERS.DDK_LV5";
    }

    /// 선택한 학기 기준 단과대 목록을 가져옵니다.
    pub async fn collages(
        &mut self,
        year: u32,
        semester: SemesterType,
    ) -> Result<Vec<String>, RusaintError> {
        self.select_semester(
            &ElementParser::new(self.client.body()),
            &format!("{year}"),
            semester,
        )
        .await;
        select_tab(&mut self.client, Self::TAB_OTHERS, 0).await?;
        Ok(combo_box_items(&mut self.client, Self::OTHERS_DDK_LV3)?)
    }

    /// 선택한 학기 기준 주어진 단과대의 학과(부) 목록을 가져옵니다.
    pub async fn departments(
        &mut self,
        year: u32,
        semester: SemesterType,
        collage: &str,
    ) -> Result<Vec<String>, RusaintError> {
        self.select_semester(
            &ElementParser::new(self.client.body()),
            &format!("{year}"),
            semester,
        )
        .await;
        select_tab(&mut self.client, Self::TAB_OTHERS, 0).await?;
        select_lv1(&mut self.client, Self::OTHERS_DDK_LV3, collage).await?;
        Ok(combo_box_items(&mut self.client, Self::OTHERS_DDK_LV4)?)
    }

    /// 선택한 학과(부)의 전공 목록을 가져옵니다.
    pub async fn majors(
        &mut self,
        year: u32,
        semester: SemesterType,
        collage: &str,
        department: &str,
    ) -> Result<Vec<String>, RusaintError> {
        self.select_semester(
            &ElementParser::new(self.client.body()),
            &format!("{year}"),
            semester,
        )
        .await;
        select_tab(&mut self.client, Self::TAB_OTHERS, 0).await?;
        select_lv2(
            &mut self.client,
            Self::OTHERS_DDK_LV3,
            Self::OTHERS_DDK_LV4,
            collage,
            department,
        )
        .await?;
        Ok(combo_box_items(&mut self.client, Self::OTHERS_DDK_LV5)?)
    }

    /// 선택한 학기의 교양필수 과목명 목록을 가져옵니다.
    pub async fn required_electives(
        &mut self,
        year: u32,
        semester: SemesterType,
    ) -> Result<Vec<String>, RusaintError> {
        define_elements! {
            TAB_GENERAL_REQ: TabStripItem<'_> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_GENERAL_REQ";
            GENERAL_REQ_TYPE: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_GENERAL_REQ.SM_OBJID";
        }
        self.select_semester(
            &ElementParser::new(self.client.body()),
            &format!("{year}"),
            semester,
        )
        .await;
        select_tab(&mut self.client, TAB_GENERAL_REQ, 1).await?;
        Ok(combo_box_items(&mut self.client, GENERAL_REQ_TYPE)?)
    }

    /// 선택한 학기의 교양선택 분야 목록을 가져옵니다.
    pub async fn optional_elective_categories(
        &mut self,
        year: u32,
        semester: SemesterType,
    ) -> Result<Vec<String>, RusaintError> {
        define_elements! {
            TAB_GENERAL_OPT: TabStripItem<'_> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_GENERAL_OPT";
            GENERAL_OPT_DISCIPLINES: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_GENERAL_OPT.DISCIPLINES";
        }
        self.select_semester(
            &ElementParser::new(self.client.body()),
            &format!("{year}"),
            semester,
        )
        .await;
        select_tab(&mut self.client, TAB_GENERAL_OPT, 2).await?;
        Ok(combo_box_items(&mut self.client, GENERAL_OPT_DISCIPLINES)?)
    }

    /// 선택한 학기의 채플 과목 분류 목록을 가져옵니다.
    pub async fn chapel_categories(
        &mut self,
        year: u32,
        semester: SemesterType,
    ) -> Result<Vec<String>, RusaintError> {
        define_elements! {
            TAB_CHAPEL: TabStripItem<'_> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_CHAPEL_REQ";
            CHAPEL_TYPE: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_CHAPEL_REQ.SM_OBJID";
        }
        self.select_semester(
            &ElementParser::new(self.client.body()),
            &format!("{year}"),
            semester,
        )
        .await;
        select_tab(&mut self.client, TAB_CHAPEL, 3).await?;
        Ok(combo_box_items(&mut self.client, CHAPEL_TYPE)?)
    }

    define_elements! {
        TAB_GRADUATE: TabStripItem<'app> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_GRADUATE";
        GRADUATE_DDK_LV3: ComboBox<'app> = "ZCMW2100.ID_0001:VIW_TAB_GRADUATE.DDK_LV3";
        GRADUATE_DDK_LV4: ComboBox<'app> = "ZCMW2100.ID_0001:VIW_TAB_GRADUATE.DDK_LV4";
    }

    /// 선택한 학기의 대학원 단과대학 목록을 가져옵니다.
    pub async fn graduated_collages(
        &mut self,
        year: u32,
        semester: SemesterType,
    ) -> Result<Vec<String>, RusaintError> {
        self.select_semester(
            &ElementParser::new(self.client.body()),
            &format!("{year}"),
            semester,
        )
        .await;
        select_tab(&mut self.client, Self::TAB_GRADUATE, 5).await?;
        Ok(combo_box_items(&mut self.client, Self::GRADUATE_DDK_LV3)?)
    }

    /// 선택한 학기의 주어진 대학원 단과대의 학과 목록을 가져옵니다.
    pub async fn graduated_departments(
        &mut self,
        year: u32,
        semester: SemesterType,
        collage: &str,
    ) -> Result<Vec<String>, RusaintError> {
        self.select_semester(
            &ElementParser::new(self.client.body()),
            &format!("{year}"),
            semester,
        )
        .await;
        select_tab(&mut self.client, Self::TAB_GRADUATE, 5).await?;
        select_lv1(&mut self.client, Self::GRADUATE_DDK_LV3, collage).await?;
        Ok(combo_box_items(&mut self.client, Self::GRADUATE_DDK_LV4)?)
    }

    /// 선택한 학기의 연계전공 목록을 가져옵니다.
    pub async fn connected_majors(
        &mut self,
        year: u32,
        semester: SemesterType,
    ) -> Result<Vec<String>, RusaintError> {
        define_elements! {
            TAB_YOMA: TabStripItem<'_> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_YOMA";
            COMBO_YOMA: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_YOMA.CONNECT_MAJO";
        }
        self.select_semester(
            &ElementParser::new(self.client.body()),
            &format!("{year}"),
            semester,
        )
        .await;
        select_tab(&mut self.client, TAB_YOMA, 8).await?;
        Ok(combo_box_items(&mut self.client, COMBO_YOMA)?)
    }

    /// 선택한 학기의 융합전공 목록을 가져옵니다.
    pub async fn united_majors(
        &mut self,
        year: u32,
        semester: SemesterType,
    ) -> Result<Vec<String>, RusaintError> {
        define_elements! {
            TAB_UNMA: TabStripItem<'_> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_UNMA";
            COMBO_UNMA: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_UNMA.CG_OBJID";
        }
        self.select_semester(
            &ElementParser::new(self.client.body()),
            &format!("{year}"),
            semester,
        )
        .await;
        select_tab(&mut self.client, TAB_UNMA, 9).await?;
        Ok(combo_box_items(&mut self.client, COMBO_UNMA)?)
    }

    /// 학기, 학년도, 강의 분류를 통해 강의를 찾습니다.
    pub async fn find_lectures(
        &mut self,
        year: u32,
        semester: SemesterType,
        lecture_category: &LectureCategory,
    ) -> Result<impl Iterator<Item = Lecture>, RusaintError> {
        {
            let parser = ElementParser::new(self.body());
            let year_str = format!("{year}");
            self.select_rows(&parser, 500).await?;
            self.select_semester(&parser, &year_str, semester).await?;
        }
        lecture_category.request_query(&mut self.client).await?;
        let parser = ElementParser::new(self.body());
        let table = parser.read(SapTableBodyCommand::new(Self::MAIN_TABLE))?;
        let Some(first_row) = table.iter().next() else {
            return Err(ApplicationError::NoLectureResult.into());
        };
        if let Some(Ok(SapTableCellWrapper::Normal(cell))) = first_row.iter_value(&parser).next()
            && let Some(ElementDefWrapper::TextView(tv_def)) = cell.content()
            && let Ok(tv) = parser.element_from_def(&tv_def)
            && tv.text().contains("없습니다.")
        {
            return Err(ApplicationError::NoLectureResult.into());
        }
        let lectures =
            try_table_into_with_scroll::<Lecture>(&mut self.client, parser, Self::MAIN_TABLE)
                .await?;

        Ok(lectures.into_iter())
    }

    /// 현재 페이지에 로드된 강의들을 가져옵니다. `find_lectures` 함수를 호출하여 강의를 검색한 이후에 사용되어야 하며, 검색한 강의들에 대한 추가 정보를 가져오고자 할 때 사용할 수 있습니다.
    /// NOTE: 이 함수는 스크롤을 수행하지 않으므로, find_lectures 함수가 너무 많은 강의(500줄 초과)를 반환한 경우, 예상대로 동작하지 않을 수 있습니다.
    pub fn loaded_lectures(&self) -> Result<impl Iterator<Item = Lecture>, RusaintError> {
        let parser = ElementParser::new(self.body());
        let table = parser.read(SapTableBodyCommand::new(Self::MAIN_TABLE))?;
        let lectures = table.try_table_into::<Lecture>(&parser)?;
        Ok(lectures.into_iter())
    }

    /// 주어진 과목번호에 해당하는 강의의 상세 정보를 가져옵니다.
    /// `find_lectures` 함수를 먼저 호출하여 강의를 검색한 이후에 사용되어야 합니다.
    pub async fn lecture_detail(&mut self, code: &str) -> Result<LectureDetail, RusaintError> {
        let parser = ElementParser::new(self.body());
        let table = parser.read(SapTableBodyCommand::new(Self::MAIN_TABLE))?;

        // Find the column index for "과목번호" from header
        let header = table
            .header()
            .ok_or(WebDynproError::from(ElementError::NoSuchContent {
                element: Self::MAIN_TABLE.id().to_string(),
                content: "Header of table".to_string(),
            }))?;
        let titles = header.titles(&parser)?;
        let code_col_idx =
            titles
                .iter()
                .position(|t| t == "과목번호")
                .ok_or(WebDynproError::from(ElementError::NoSuchContent {
                    element: Self::MAIN_TABLE.id().to_string(),
                    content: "과목번호 column".to_string(),
                }))?;

        // Find the row with matching code and get the Link's activate event
        let activate_event = table
            .iter()
            .find_map(|row| {
                let cell_wrapper =
                    SapTableCellWrapper::from_def(&row[code_col_idx], &parser).ok()?;
                if let Some(ElementDefWrapper::Link(link_def)) = cell_wrapper.content() {
                    let link = parser.element_from_def(&link_def).ok()?;
                    if link.text() == code {
                        link.activate(false, false).ok()
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .ok_or(WebDynproError::from(ElementError::NoSuchData {
                element: Self::MAIN_TABLE.id().to_string(),
                field: format!("lecture with code {code}"),
            }))?;

        // Send the activate event to open the detail popup
        self.client.process_event(false, activate_event).await?;

        // Parse the detail from the popup
        let parser = ElementParser::new(self.body());
        let detail = LectureDetail::with_parser(&parser)?;

        // Close the popup
        self.close_popups().await?;

        Ok(detail)
    }

    async fn close_popups(&mut self) -> Result<(), WebDynproError> {
        let popup_selector =
            Selector::parse(format!(r#"[ct="{}"]"#, PopupWindow::CONTROL_ID).as_str()).unwrap();
        fn make_close_event(body: &Body, selector: &Selector) -> Option<Event> {
            let parser = ElementParser::new(body);
            let mut popup_iter = parser.document().select(selector);
            popup_iter.next().and_then(|elem| {
                let elem_wrapped = ElementWrapper::from_ref(elem).ok()?;
                if let ElementWrapper::PopupWindow(popup) = elem_wrapped {
                    popup.close().ok()
                } else {
                    None
                }
            })
        }
        while let Some(event) = make_close_event(self.body(), &popup_selector) {
            self.client.process_event(false, event).await?;
        }
        Ok(())
    }

    /// 페이지를 새로고침합니다.
    pub async fn reload(&mut self) -> Result<(), RusaintError> {
        self.client.reload().await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {}

/// 강의시간표 애플리케이션에서 사용하는 데이터 모델
pub mod model;
mod utils;
