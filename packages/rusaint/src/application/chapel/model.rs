use std::collections::HashMap;

use serde::{
    Deserialize, Serialize,
    de::{IntoDeserializer, value::MapDeserializer},
};

use crate::application::utils::de_with::{deserialize_semester_type, deserialize_u32_string};
use crate::webdynpro::command::WebDynproCommandExecutor;
use crate::webdynpro::element::parser::ElementParser;
use crate::{
    RusaintError, define_elements,
    error::ApplicationError,
    model::SemesterType,
    webdynpro::{
        command::element::complex::SapTableBodyCommand,
        element::{
            ElementDefWrapper,
            complex::{
                SapTable,
                sap_table::{
                    FromSapTable,
                    cell::{SapTableCell, SapTableCellWrapper},
                },
            },
            definition::ElementDefinition,
        },
        error::{ElementError, WebDynproError},
    },
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// 학기별 채플 정보
pub struct ChapelInformation {
    year: u32,
    semester: SemesterType,
    general_information: GeneralChapelInformation,
    attendances: Vec<ChapelAttendance>,
    absence_requests: Vec<ChapelAbsenceRequest>,
}

impl ChapelInformation {
    pub(crate) fn new(
        year: u32,
        semester: SemesterType,
        general_information: GeneralChapelInformation,
        attendances: Vec<ChapelAttendance>,
        absence_requests: Vec<ChapelAbsenceRequest>,
    ) -> Self {
        Self {
            year,
            semester,
            general_information,
            attendances,
            absence_requests,
        }
    }

    /// 해당 채플 정보의 학년도를 반환합니다.
    pub fn year(&self) -> u32 {
        self.year
    }

    /// 해당 채플 정보의 학기를 반환합니다.
    pub fn semester(&self) -> SemesterType {
        self.semester
    }

    /// 기본 채플 정보(좌석번호, 결석현황, 성적결과)를 반환합니다.
    pub fn general_information(&self) -> &GeneralChapelInformation {
        &self.general_information
    }

    /// 채플 출결 정보를 반환합니다.
    pub fn attendances(&self) -> &[ChapelAttendance] {
        &self.attendances
    }

    /// 채플 결석신청 정보를 반환합니다.
    pub fn absence_requests(&self) -> &[ChapelAbsenceRequest] {
        &self.absence_requests
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// 채플 기본 정보(좌석번호, 결석현황, 성적결과)
pub struct GeneralChapelInformation {
    #[serde(
        rename(deserialize = "분반"),
        deserialize_with = "deserialize_u32_string"
    )]
    division: u32,
    #[serde(rename(deserialize = "시간표"))]
    chapel_time: String,
    #[serde(rename(deserialize = "강의실"))]
    chapel_room: String,
    #[serde(
        rename(deserialize = "층수"),
        deserialize_with = "deserialize_u32_string"
    )]
    floor_level: u32,
    #[serde(rename(deserialize = "좌석번호"))]
    seat_number: String,
    #[serde(
        rename(deserialize = "결석일수"),
        deserialize_with = "deserialize_u32_string"
    )]
    absence_time: u32,
    #[serde(rename(deserialize = "성적"))]
    result: String,
    #[serde(rename(deserialize = "비고"))]
    note: String,
}

impl<'a> GeneralChapelInformation {
    define_elements! {
        TABLE: SapTable<'a> = "ZCMW3681.ID_0001:V_MAIN.TABLE";
    }

    pub(crate) fn with_parser(parser: &'a ElementParser) -> Result<Vec<Self>, RusaintError> {
        let table = parser.read(SapTableBodyCommand::new(Self::TABLE))?;
        let Some(first_row) = table.iter().next() else {
            return Err(ApplicationError::NoChapelInformation.into());
        };
        if let Some(Ok(SapTableCellWrapper::Normal(cell))) = first_row.iter_value(parser).next() {
            if let Some(ElementDefWrapper::TextView(tv_def)) = cell.content() {
                if let Ok(tv) = parser.element_from_def(&tv_def) {
                    if tv.text().contains("없습니다.") {
                        return Err(ApplicationError::NoChapelInformation.into());
                    }
                }
            }
        }
        Ok(table.try_table_into::<Self>(parser)?)
    }

    /// 분반 번호를 반환합니다.
    pub fn division(&self) -> u32 {
        self.division
    }

    /// 채플 시간을 반환합니다.
    pub fn chapel_time(&self) -> &str {
        &self.chapel_time
    }

    /// 채플 강의실을 반환합니다.
    pub fn chapel_room(&self) -> &str {
        &self.chapel_room
    }

    /// 층수를 반환합니다.
    pub fn floor_level(&self) -> u32 {
        self.floor_level
    }

    /// 좌석번호를 반환합니다.
    pub fn seat_number(&self) -> &str {
        &self.seat_number
    }

    /// 결석일수를 반환합니다.
    pub fn absence_time(&self) -> u32 {
        self.absence_time
    }

    /// 성적을 반환합니다.
    pub fn result(&self) -> &str {
        &self.result
    }

    /// 비고 내용을 반환합니다.
    pub fn note(&self) -> &str {
        &self.note
    }
}

impl<'body> FromSapTable<'body> for GeneralChapelInformation {
    fn from_table(
        header: &'body crate::webdynpro::element::complex::sap_table::SapTableHeader,
        row: &'body crate::webdynpro::element::complex::sap_table::SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, WebDynproError> {
        let map_string = row.try_row_into::<HashMap<String, String>>(header, parser)?;
        let map_de: MapDeserializer<_, serde::de::value::Error> = map_string.into_deserializer();
        Ok(
            Self::deserialize(map_de).map_err(|e| ElementError::InvalidContent {
                element: row.table_def().id().to_string(),
                content: e.to_string(),
            })?,
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// 채플 수업별 출석정보
pub struct ChapelAttendance {
    #[serde(
        rename(deserialize = "분반"),
        deserialize_with = "deserialize_u32_string"
    )]
    division: u32,
    #[serde(rename(deserialize = "수업일자"))]
    class_date: String,
    #[serde(rename(deserialize = "강의구분"))]
    category: String,
    #[serde(rename(deserialize = "강사"))]
    instructor: String,
    #[serde(rename(deserialize = "소속"))]
    instructor_department: String,
    #[serde(rename(deserialize = "제목"))]
    title: String,
    #[serde(rename(deserialize = "출결상태"))]
    attendance: String,
    #[serde(rename(deserialize = "평가"))]
    result: String,
    #[serde(rename(deserialize = "비고"))]
    note: String,
}

impl<'a> ChapelAttendance {
    define_elements! {
        TABLE_A: SapTable<'a> = "ZCMW3681.ID_0001:V_MAIN.TABLE_A";
    }

    pub(crate) fn with_parser(parser: &'a ElementParser) -> Result<Vec<Self>, WebDynproError> {
        let table = parser.read(SapTableBodyCommand::new(Self::TABLE_A))?;
        let Some(first_row) = table.iter().next() else {
            return Ok(Vec::with_capacity(0));
        };
        if let Some(Ok(SapTableCellWrapper::Normal(cell))) = first_row.iter_value(parser).next() {
            if let Some(ElementDefWrapper::TextView(tv_def)) = cell.content() {
                if let Ok(tv) = parser.element_from_def(&tv_def) {
                    if tv.text().contains("채플 출결 상세내용") {
                        return Ok(Vec::with_capacity(0));
                    }
                }
            }
        }
        table.try_table_into::<Self>(parser)
    }

    /// 채플 분반 번호를 반환합니다.
    pub fn division(&self) -> u32 {
        self.division
    }

    /// 수업일자를 반환합니다.
    pub fn class_date(&self) -> &str {
        &self.class_date
    }

    /// 강의구분을 반환합니다.
    pub fn category(&self) -> &str {
        &self.category
    }

    /// 강사명을 반환합니다.
    pub fn instructor(&self) -> &str {
        &self.instructor
    }

    /// 강사의 소속을 반환합니다.
    pub fn instructor_department(&self) -> &str {
        &self.instructor_department
    }

    /// 강의 제목을 반환합니다.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// 출결상태를 반환합니다.
    pub fn attendance(&self) -> &str {
        &self.attendance
    }

    /// 평가 내용을 반환합니다.
    pub fn result(&self) -> &str {
        &self.result
    }

    /// 비고를 반환합니다.
    pub fn note(&self) -> &str {
        &self.note
    }
}

impl<'body> FromSapTable<'body> for ChapelAttendance {
    fn from_table(
        header: &'body crate::webdynpro::element::complex::sap_table::SapTableHeader,
        row: &'body crate::webdynpro::element::complex::sap_table::SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, WebDynproError> {
        let map_string = row.try_row_into::<HashMap<String, String>>(header, parser)?;
        let map_de: MapDeserializer<_, serde::de::value::Error> = map_string.into_deserializer();
        Ok(
            Self::deserialize(map_de).map_err(|e| ElementError::InvalidContent {
                element: row.table_def().id().to_string(),
                content: e.to_string(),
            })?,
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// 채플 결석신청 정보
pub struct ChapelAbsenceRequest {
    #[serde(
        rename(deserialize = "학년도"),
        deserialize_with = "deserialize_u32_string"
    )]
    year: u32,
    #[serde(
        rename(deserialize = "학기"),
        deserialize_with = "deserialize_semester_type"
    )]
    semester: SemesterType,
    #[serde(rename(deserialize = "결석구분상세"))]
    absence_detail: String,
    #[serde(rename(deserialize = "결석시작일자"))]
    absence_start: String,
    #[serde(rename(deserialize = "결석종료일자"))]
    absence_end: String,
    #[serde(rename(deserialize = "결석사유(국문)"))]
    absence_reason_kr: String,
    #[serde(rename(deserialize = "결석사유(영문)"))]
    absence_reason_en: String,
    #[serde(rename(deserialize = "신청일자"))]
    application_date: String,
    #[serde(rename(deserialize = "승인일자"))]
    approval_date: String,
    #[serde(rename(deserialize = "거부사유"))]
    denial_reason: String,
    #[serde(rename(deserialize = "상태"))]
    status: String,
}

impl<'a> ChapelAbsenceRequest {
    define_elements! {
        TABLE02_CP_CP: SapTable<'a> = "ZCMW3681.ID_0001:V_MAIN.TABLE02_CP_CP";
    }
    pub(crate) fn with_parser(parser: &'a ElementParser) -> Result<Vec<Self>, RusaintError> {
        let table = parser.read(SapTableBodyCommand::new(Self::TABLE02_CP_CP))?;
        let Some(first_row) = table.iter().next() else {
            return Ok(Vec::with_capacity(0));
        };
        if let Some(Ok(SapTableCellWrapper::Normal(cell))) = first_row.iter_value(parser).next() {
            if let Some(ElementDefWrapper::TextView(tv_def)) = cell.content() {
                if let Ok(tv) = parser.element_from_def(&tv_def) {
                    if tv.text().contains("없습니다.") {
                        return Ok(Vec::with_capacity(0));
                    }
                }
            }
        }

        Ok(table.try_table_into::<Self>(parser)?)
    }

    /// 신청 학년도를 반환합니다.
    pub fn year(&self) -> u32 {
        self.year
    }

    /// 신청 학기를 반환합니다.
    pub fn semester(&self) -> SemesterType {
        self.semester
    }

    /// 결석구분상세를 반환합니다.
    pub fn absence_detail(&self) -> &str {
        &self.absence_detail
    }

    /// 결석시작일자를 반환합니다.
    pub fn absence_start(&self) -> &str {
        &self.absence_start
    }

    /// 결석종료일자를 반환합니다.
    pub fn absence_end(&self) -> &str {
        &self.absence_end
    }

    /// 국문 결석사유를 반환합니다.
    pub fn absence_reason_kr(&self) -> &str {
        &self.absence_reason_kr
    }

    /// 영문 결석사유를 반환합니다.
    pub fn absence_reason_en(&self) -> &str {
        &self.absence_reason_en
    }

    /// 신청일자를 반환합니다.
    pub fn application_date(&self) -> &str {
        &self.application_date
    }

    /// 승인일자를 반환합니다.
    pub fn approval_date(&self) -> &str {
        &self.approval_date
    }

    /// 거부사유를 반환합니다.
    pub fn denial_reason(&self) -> &str {
        &self.denial_reason
    }

    /// 요청 상태를 반환합니다.
    pub fn status(&self) -> &str {
        &self.status
    }
}

impl<'body> FromSapTable<'body> for ChapelAbsenceRequest {
    fn from_table(
        header: &'body crate::webdynpro::element::complex::sap_table::SapTableHeader,
        row: &'body crate::webdynpro::element::complex::sap_table::SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, WebDynproError> {
        let map_string = row.try_row_into::<HashMap<String, String>>(header, parser)?;
        let map_de: MapDeserializer<_, serde::de::value::Error> = map_string.into_deserializer();
        Ok(
            Self::deserialize(map_de).map_err(|e| ElementError::InvalidContent {
                element: row.table_def().id().to_string(),
                content: e.to_string(),
            })?,
        )
    }
}
