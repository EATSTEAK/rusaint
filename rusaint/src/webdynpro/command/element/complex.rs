use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::event::Event;
use crate::webdynpro::{
    command::WebDynproCommand,
    element::{
        complex::{sap_table::SapTableBody, SapTableDef, SapTableLSData},
        Element,
    },
    error::WebDynproError,
};

/// 주어진 [`SapTable`](crate::webdynpro::element::complex::SapTable)의 상하 스크롤을 수행
pub struct SapTableVerticalScrollCommand {
    element_def: SapTableDef,
    first_visible_item_index: u32,
    cell_id: String,
    access_type: String,
    selection_follow_focus: bool,
    shift: bool,
    ctrl: bool,
    alt: bool,
}

impl SapTableVerticalScrollCommand {
    /// 새로운 명령 객체를 생성합니다.
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

impl WebDynproCommand for SapTableVerticalScrollCommand {
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
pub struct ReadSapTableLSDataCommand {
    element_def: SapTableDef,
}

impl ReadSapTableLSDataCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: SapTableDef) -> ReadSapTableLSDataCommand {
        Self { element_def }
    }
}

impl WebDynproCommand for ReadSapTableLSDataCommand {
    type Result = SapTableLSData;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        let lsdata = parser.element_from_def(&self.element_def)?.lsdata().clone();
        Ok(lsdata)
    }
}

/// [`SapTableBody`]를 반환
pub struct ReadSapTableBodyCommand {
    element_def: SapTableDef,
}

impl ReadSapTableBodyCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: SapTableDef) -> ReadSapTableBodyCommand {
        Self { element_def }
    }
}

impl WebDynproCommand for ReadSapTableBodyCommand {
    type Result = SapTableBody;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        let body = parser.element_from_def(&self.element_def)?.table()?.clone();
        Ok(body)
    }
}
