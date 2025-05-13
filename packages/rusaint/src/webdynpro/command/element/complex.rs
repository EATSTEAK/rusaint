use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::event::Event;
use crate::webdynpro::{
    command::WebDynproCommand,
    element::{
        Element,
        complex::{SapTableDef, SapTableLSData, sap_table::SapTableBody},
    },
    error::WebDynproError,
};

/// 주어진 [`SapTable`](crate::webdynpro::element::complex::SapTable)의 상하 스크롤을 수행하는 이벤트를 반환
pub struct SapTableVerticalScrollEventCommand {
    element_def: SapTableDef,
    first_visible_item_index: u32,
    cell_id: String,
    access_type: String,
    selection_follow_focus: bool,
    shift: bool,
    ctrl: bool,
    alt: bool,
}

impl SapTableVerticalScrollEventCommand {
    /// 새로운 명령 객체를 생성합니다.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        element_def: SapTableDef,
        first_visible_item_index: u32,
        cell_id: &str,
        access_type: &str,
        selection_follow_focus: bool,
        shift: bool,
        ctrl: bool,
        alt: bool,
    ) -> Self {
        Self {
            element_def,
            first_visible_item_index,
            cell_id: cell_id.to_string(),
            access_type: access_type.to_string(),
            selection_follow_focus,
            shift,
            ctrl,
            alt,
        }
    }
}

impl WebDynproCommand for SapTableVerticalScrollEventCommand {
    type Result = Event;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        parser.element_from_def(&self.element_def)?.vertical_scroll(
            self.first_visible_item_index,
            &self.cell_id,
            &self.access_type,
            self.selection_follow_focus,
            self.shift,
            self.ctrl,
            self.alt,
        )
    }
}

/// [`SapTableLSData`]를 반환
pub struct SapTableLSDataCommand {
    element_def: SapTableDef,
}

impl SapTableLSDataCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: SapTableDef) -> SapTableLSDataCommand {
        Self { element_def }
    }
}

impl WebDynproCommand for SapTableLSDataCommand {
    type Result = SapTableLSData;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        let lsdata = parser.element_from_def(&self.element_def)?.lsdata().clone();
        Ok(lsdata)
    }
}

/// [`SapTableBody`]를 반환
pub struct SapTableBodyCommand {
    element_def: SapTableDef,
}

impl SapTableBodyCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: SapTableDef) -> SapTableBodyCommand {
        Self { element_def }
    }
}

impl WebDynproCommand for SapTableBodyCommand {
    type Result = SapTableBody;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        let body = parser.element_from_def(&self.element_def)?.table()?.clone();
        Ok(body)
    }
}
