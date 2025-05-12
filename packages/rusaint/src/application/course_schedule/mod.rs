use super::{USaintApplication, USaintClient};
use crate::application::course_schedule::utils::{
    combo_box_items, select_lv1, select_lv2, select_tab,
};
use crate::application::utils::sap_table::try_table_into_with_scroll;
use crate::application::utils::semester::get_selected_semester;
use crate::webdynpro::command::WebDynproCommandExecutor;
use crate::webdynpro::element::layout::tab_strip::item::TabStripItem;
use crate::webdynpro::element::parser::ElementParser;
use crate::{
    ApplicationError, RusaintError,
    application::course_schedule::model::{Lecture, LectureCategory},
    define_elements,
    model::SemesterType,
    webdynpro::{
        client::body::Body,
        command::element::{complex::SapTableBodyCommand, selection::ComboBoxSelectEventCommand},
        element::{
            ElementDefWrapper,
            complex::{
                SapTable,
                sap_table::cell::{SapTableCell, SapTableCellWrapper},
            },
            layout::TabStrip,
            selection::ComboBox,
        },
        error::WebDynproError,
    },
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
        select_tab(&mut self.client.0, Self::TAB_OTHERS, 0).await?;
        Ok(combo_box_items(&mut self.client.0, Self::OTHERS_DDK_LV3)?)
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
        select_tab(&mut self.client.0, Self::TAB_OTHERS, 0).await?;
        select_lv1(&mut self.client.0, Self::OTHERS_DDK_LV3, collage).await?;
        Ok(combo_box_items(&mut self.client.0, Self::OTHERS_DDK_LV4)?)
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
        select_tab(&mut self.client.0, Self::TAB_OTHERS, 0).await?;
        select_lv2(
            &mut self.client.0,
            Self::OTHERS_DDK_LV3,
            Self::OTHERS_DDK_LV4,
            collage,
            department,
        )
        .await?;
        Ok(combo_box_items(&mut self.client.0, Self::OTHERS_DDK_LV5)?)
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
        select_tab(&mut self.client.0, TAB_GENERAL_REQ, 1).await?;
        Ok(combo_box_items(&mut self.client.0, GENERAL_REQ_TYPE)?)
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
        select_tab(&mut self.client.0, TAB_GENERAL_OPT, 2).await?;
        Ok(combo_box_items(
            &mut self.client.0,
            GENERAL_OPT_DISCIPLINES,
        )?)
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
        select_tab(&mut self.client.0, TAB_CHAPEL, 3).await?;
        Ok(combo_box_items(&mut self.client.0, CHAPEL_TYPE)?)
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
        select_tab(&mut self.client.0, Self::TAB_GRADUATE, 5).await?;
        Ok(combo_box_items(&mut self.client.0, Self::GRADUATE_DDK_LV3)?)
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
        select_tab(&mut self.client.0, Self::TAB_GRADUATE, 5).await?;
        select_lv1(&mut self.client.0, Self::GRADUATE_DDK_LV3, collage).await?;
        Ok(combo_box_items(&mut self.client.0, Self::GRADUATE_DDK_LV4)?)
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
        select_tab(&mut self.client.0, TAB_YOMA, 8).await?;
        Ok(combo_box_items(&mut self.client.0, COMBO_YOMA)?)
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
        select_tab(&mut self.client.0, TAB_UNMA, 9).await?;
        Ok(combo_box_items(&mut self.client.0, COMBO_UNMA)?)
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
        lecture_category.request_query(&mut self.client.0).await?;
        let parser = ElementParser::new(self.body());
        let table = parser.read(SapTableBodyCommand::new(Self::MAIN_TABLE))?;
        let Some(first_row) = table.iter().next() else {
            return Err(ApplicationError::NoLectureResult.into());
        };
        if let Some(Ok(SapTableCellWrapper::Normal(cell))) = first_row.iter_value(&parser).next() {
            if let Some(ElementDefWrapper::TextView(tv_def)) = cell.content() {
                if let Ok(tv) = parser.element_from_def(&tv_def) {
                    if tv.text().contains("없습니다.") {
                        return Err(ApplicationError::NoLectureResult.into());
                    }
                }
            }
        }
        let lectures =
            try_table_into_with_scroll::<Lecture>(&mut self.client, parser, Self::MAIN_TABLE)
                .await?;
        Ok(lectures.into_iter())
    }

    // TO-DO: 카테고리 별 선택지 가져오기 기능
}

#[cfg(test)]
mod test {}

/// 강의시간표 애플리케이션에서 사용하는 데이터 모델
pub mod model;
mod utils;
