use crate::webdynpro::{
    client::EventProcessResult,
    command::{WebDynproCommand, WebDynproReadCommand},
    element::{
        complex::{sap_table::SapTableBody, SapTableDef, SapTableLSData},
        definition::ElementDefinition,
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
    type Result = EventProcessResult;

    async fn dispatch(
        &self,
        client: &mut crate::webdynpro::client::WebDynproClient,
    ) -> Result<Self::Result, WebDynproError> {
        let event = (&self.element_def)
            .from_body(client.body())?
            .vertical_scroll(
                self.first_visible_item_index,
                &self.cell_id,
                &self.access_type,
                self.selection_follow_focus,
                self.shift,
                self.ctrl,
                self.alt,
            )?;
        client.process_event(false, event).await
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

impl WebDynproReadCommand for ReadSapTableLSDataCommand {
    fn read(
        &self,
        body: &crate::webdynpro::client::body::Body,
    ) -> Result<Self::Result, WebDynproError> {
        let lsdata = self.element_def.from_body(body)?.lsdata().clone();
        Ok(lsdata)
    }
}

impl WebDynproCommand for ReadSapTableLSDataCommand {
    type Result = SapTableLSData;

    async fn dispatch(
        &self,
        client: &mut crate::webdynpro::client::WebDynproClient,
    ) -> Result<Self::Result, WebDynproError> {
        self.read(client.body())
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

impl WebDynproReadCommand for ReadSapTableBodyCommand {
    fn read(
        &self,
        body: &crate::webdynpro::client::body::Body,
    ) -> Result<Self::Result, WebDynproError> {
        let body = self.element_def.from_body(body)?.table()?.clone();
        Ok(body)
    }
}

impl WebDynproCommand for ReadSapTableBodyCommand {
    type Result = SapTableBody;

    async fn dispatch(
        &self,
        client: &mut crate::webdynpro::client::WebDynproClient,
    ) -> Result<Self::Result, WebDynproError> {
        self.read(client.body())
    }
}
