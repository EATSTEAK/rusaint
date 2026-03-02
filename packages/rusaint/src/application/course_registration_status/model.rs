use std::collections::HashMap;

use serde::{
    Deserialize, Serialize,
    de::{IntoDeserializer, value::MapDeserializer},
};

use crate::application::utils::de_with::deserialize_optional_string;
use wdpe::{
    element::{
        complex::sap_table::FromSapTable, definition::ElementDefinition as _, parser::ElementParser,
    },
    error::{ElementError, WebDynproError},
};

/// 수강신청한 과목 정보
#[allow(unused)]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct RegisteredLecture {
    /// 계획
    #[serde(
        rename(deserialize = "계획"),
        default,
        deserialize_with = "deserialize_optional_string"
    )]
    syllabus: Option<String>,
    /// 이수구분
    #[serde(rename(deserialize = "이수구분"))]
    category: String,
    /// 다전공구분
    #[serde(
        rename(deserialize = "다전공구분"),
        default,
        deserialize_with = "deserialize_optional_string"
    )]
    sub_category: Option<String>,
    /// 공학인증
    #[serde(
        rename(deserialize = "공학인증"),
        default,
        deserialize_with = "deserialize_optional_string"
    )]
    abeek_info: Option<String>,
    /// 교과영역
    #[serde(
        rename(deserialize = "교과영역"),
        default,
        deserialize_with = "deserialize_optional_string"
    )]
    field: Option<String>,
    /// 과목번호
    #[serde(rename(deserialize = "과목번호"))]
    code: String,
    /// 과목명
    #[serde(rename(deserialize = "과목명"))]
    name: String,
    /// 분반
    #[serde(
        rename(deserialize = "분반"),
        default,
        deserialize_with = "deserialize_optional_string"
    )]
    division: Option<String>,
    /// 교수명
    #[serde(rename(deserialize = "교수명"))]
    professor: String,
    /// 시간/학점(설계)
    #[serde(rename(deserialize = "시간/학점(설계)"))]
    time_points: String,
    /// 요일/시간(강의실)
    #[serde(rename(deserialize = "요일/시간(강의실)"))]
    schedule_room: String,
    /// 과정
    #[serde(rename(deserialize = "과정"))]
    target: String,
    /// 수강 신청일
    #[serde(rename(deserialize = "수강 신청일"))]
    register_date: String,
    /// 비고
    #[serde(rename(deserialize = "비고"))]
    remarks: String,
}

impl<'body> FromSapTable<'body> for RegisteredLecture {
    fn from_table(
        header: Option<&'body wdpe::element::complex::sap_table::SapTableHeader>,
        row: &'body wdpe::element::complex::sap_table::SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, WebDynproError> {
        let map_string = row.try_row_into::<HashMap<String, String>>(header, parser)?;
        let map_de: MapDeserializer<_, serde::de::value::Error> = map_string.into_deserializer();
        Ok(
            RegisteredLecture::deserialize(map_de).map_err(|e| ElementError::InvalidContent {
                element: row.table_def().id().to_string(),
                content: e.to_string(),
            })?,
        )
    }
}
