use serde::{Deserialize, Deserializer};

use crate::{define_elements, model::SemesterType, webdynpro::{client::body::Body, element::complex::SapTable, error::WebDynproError}};

#[derive(Clone, Debug)]
pub struct ChapelInformation {
    year: u32,
    semester: SemesterType,
    general_information: GeneralChapelInformation,
    attendances: Vec<ChapelAttendance>,
    absence_requests: Vec<ChapelAbsenceRequest>,
}

impl ChapelInformation {
    pub fn new(year: u32, semester: SemesterType, general_information: GeneralChapelInformation, attendances: Vec<ChapelAttendance>, absence_requests: Vec<ChapelAbsenceRequest>) -> Self {
        Self { year, semester, general_information, attendances, absence_requests }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct GeneralChapelInformation {
    #[serde(rename(deserialize = "분반"))]
    division: u32,
    #[serde(rename(deserialize = "시간표"))]
    chapel_time: String,
    #[serde(rename(deserialize = "강의실"))]
    chapel_room: String,
    #[serde(rename(deserialize = "층수"))]
    floor_level: u32,
    #[serde(rename(deserialize = "좌석번호"))]
    seat_number: String,
    #[serde(rename(deserialize = "결석일수"))]
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

    pub fn from_body(body: &'a Body) -> Result<Self, WebDynproError> {
        todo!()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChapelAttendance {
    #[serde(rename(deserialize = "분반"))]
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

    pub fn from_body(body: &'a Body) -> Result<Self, WebDynproError> {
        todo!()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChapelAbsenceRequest {
    #[serde(rename(deserialize = "학년도"))]
    year: u32,
    #[serde(rename(deserialize = "학기"), deserialize_with = "deserialize_semester_type")]
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
    status: String
}

pub fn deserialize_semester_type<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<SemesterType, D::Error> {
    let value = String::deserialize(deserializer)?;
    match value.trim() {
        "1 학기" => Ok(SemesterType::One),
        "여름 학기" => Ok(SemesterType::Summer),
        "2 학기" => Ok(SemesterType::Two),
        "겨울 학기" => Ok(SemesterType::Winter),
        _ => Err(serde::de::Error::custom("Unknown SemesterType varient"))
    }
}

impl<'a> ChapelAbsenceRequest {

    define_elements! {
        TABLE02_CP_CP: SapTable<'a> = "ZCMW3681.ID_0001:V_MAIN.TABLE02_CP_CP";
    }
    pub fn from_body(body: &'a Body) -> Result<Self, WebDynproError> {
        todo!()
    }
}