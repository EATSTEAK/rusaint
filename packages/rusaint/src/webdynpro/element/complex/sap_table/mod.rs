use std::{borrow::Cow, cell::OnceCell, collections::HashMap};

use scraper::Selector;

use crate::webdynpro::{
    element::{Interactable, definition::ElementDefinition, macros::define_element_interactable},
    error::{BodyError, ElementError, WebDynproError},
    event::Event,
};

use self::property::AccessType;

define_element_interactable! {
    #[doc = "테이블"]
    SapTable<"ST", "SapTable"> {
        table: OnceCell<Option<SapTableBody>>,
    },
    #[doc = "[`SapTable`]의 정의"]
    SapTableDef,
    #[doc = "[`SapTable`] 내부 데이터"]
    SapTableLSData {
        title_text: String => "0",
        accessibility_description: String => "1",
        row_count: u32 => "2",
        col_count: u32 => "3",
    }
}

impl<'a> SapTable<'a> {
    /// HTML 엘리먼트로부터 새로운 [`SapTable`] 엘리먼트를 생성합니다.
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
            table: OnceCell::new(),
        }
    }

    /// 테이블 내부 컨텐츠를 반환합니다.
    pub fn table(&self) -> Result<&SapTableBody, WebDynproError> {
        self.table
            .get_or_init(|| self.parse_table().ok())
            .as_ref()
            .ok_or(WebDynproError::Element(ElementError::NoSuchContent {
                element: self.id.to_string(),
                content: "Table body".to_string(),
            }))
    }

    fn parse_table(&self) -> Result<SapTableBody, WebDynproError> {
        let def: SapTableDef = {
            if let Cow::Borrowed(id) = self.id {
                SapTableDef::new(id)
            } else {
                SapTableDef::new_dynamic(self.id.to_string())
            }
        };
        let element = self.element_ref;
        let tbody_selector = Selector::parse(
            format!(
                r#"[id="{}-contentTBody"]"#,
                element.value().id().ok_or(ElementError::NoSuchData {
                    element: self.id.clone().into_owned(),
                    field: "id".to_string()
                })?
            )
            .as_str(),
        )
        .or(Err(BodyError::InvalidSelector))?;
        let Some(tbody) = element.select(&tbody_selector).next() else {
            return Err(ElementError::NoSuchContent {
                element: self.id.clone().into_owned(),
                content: "Table body".to_string(),
            })?;
        };
        Ok(SapTableBody::new(def, tbody)?)
    }

    /// 테이블의 행을 선택하는 이벤트를 반환합니다.
    pub fn row_select(
        &self,
        row_index: i32,
        row_user_data: &str,
        cell_user_data: &str,
        access_type: AccessType,
        trigger_cell_id: &str,
    ) -> Result<Event, WebDynproError> {
        let parameters: HashMap<String, String> = HashMap::from([
            ("Id".to_string(), self.id.clone().to_string()),
            ("RowIndex".to_string(), format!("{row_index}")),
            ("RowUserData".to_string(), row_user_data.to_owned()),
            ("CellUserData".to_string(), cell_user_data.to_owned()),
            ("AccessType".to_string(), access_type.to_string()),
            ("TriggerCellId".to_string(), trigger_cell_id.to_owned()),
        ]);
        self.fire_event("RowSelect".to_string(), parameters)
    }

    /// 테이블의 내부 셀을 선택하는 이벤트를 반환합니다.
    #[allow(clippy::too_many_arguments)]
    pub fn cell_select(
        &self,
        cell_id: &str,
        cell_type: &str,
        row_index: i32,
        col_index: i32,
        row_user_data: &str,
        cell_user_data: &str,
        access_type: AccessType,
    ) -> Result<Event, WebDynproError> {
        let parameters: HashMap<String, String> = HashMap::from([
            ("Id".to_string(), self.id.clone().to_string()),
            ("CellId".to_string(), cell_id.to_owned()),
            ("CellType".to_string(), cell_type.to_owned()),
            ("RowIndex".to_string(), format!("{row_index}")),
            ("ColIndex".to_string(), format!("{col_index}")),
            ("RowUserData".to_string(), row_user_data.to_owned()),
            ("CellUserData".to_string(), cell_user_data.to_owned()),
            ("AccessType".to_string(), access_type.to_string()),
        ]);
        self.fire_event("CellSelect".to_string(), parameters)
    }

    /// 테이블을 상하로 스크롤하는 이벤트를 반환합니다.
    #[allow(clippy::too_many_arguments)]
    pub fn vertical_scroll(
        &self,
        first_visible_item_index: u32,
        cell_id: &str,
        access_type: &str,
        selection_follow_focus: bool,
        shift: bool,
        ctrl: bool,
        alt: bool,
    ) -> Result<Event, WebDynproError> {
        let parameters: HashMap<String, String> = HashMap::from([
            ("Id".to_string(), self.id.clone().to_string()),
            (
                "FirstVisibleItemIndex".to_string(),
                first_visible_item_index.to_string(),
            ),
            ("CellId".to_string(), cell_id.to_owned()),
            ("AccessType".to_string(), access_type.to_string()),
            (
                "SelectionFollowFocus".to_string(),
                selection_follow_focus.to_string(),
            ),
            ("Shift".to_string(), shift.to_string()),
            ("Ctrl".to_string(), ctrl.to_string()),
            ("Alt".to_string(), alt.to_string()),
        ]);
        self.fire_event("VerticalScroll".to_string(), parameters)
    }
}

mod body;
mod from_sap_table;
mod header;
mod row;

/// [`SapTable`] 내부 셀
pub mod cell;
/// [`SapTable`] 내부 데이터 프로퍼티
pub mod property;

pub use self::body::SapTableBody;
pub use self::from_sap_table::FromSapTable;
pub use self::header::SapTableHeader;
pub use self::row::SapTableRow;
