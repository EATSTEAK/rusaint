use std::collections::HashMap;

use serde::{
    Deserialize, Serialize,
    de::{IntoDeserializer, value::MapDeserializer},
};

use crate::application::utils::de_with::{
    deserialize_f32_string, deserialize_semester_type, deserialize_u32_string,
};
use crate::webdynpro::element::parser::ElementParser;
use crate::{
    model::SemesterType,
    webdynpro::{
        element::{complex::sap_table::FromSapTable, definition::ElementDefinition},
        error::{ElementError, WebDynproError},
    },
};

/// 강의평가 결과
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct LectureAssessmentResult {
    #[serde(rename(deserialize = "년도"))]
    year: String,
    #[serde(
        rename(deserialize = "학기"),
        deserialize_with = "deserialize_semester_type"
    )]
    semester: SemesterType,
    #[serde(
        rename(deserialize = "과목코드"),
        deserialize_with = "deserialize_u32_string"
    )]
    lecture_code: u32,
    #[serde(rename(deserialize = "과목명"))]
    lecture_name: String,
    #[serde(
        rename(deserialize = "학점"),
        deserialize_with = "deserialize_f32_string"
    )]
    points: f32,
    #[serde(rename(deserialize = "교수명"))]
    professor: String,
    #[serde(rename(deserialize = "소속대학"))]
    collage: String,
    #[serde(rename(deserialize = "소속학과"))]
    department: String,
    #[serde(rename(deserialize = "직위명"))]
    position: String,
    #[serde(
        rename(deserialize = "점수"),
        deserialize_with = "deserialize_f32_string"
    )]
    score: f32,
}

impl LectureAssessmentResult {
    /// 강의 학년도를 반환합니다.
    pub fn year(&self) -> &str {
        &self.year
    }

    /// 강의 학기를 반환합니다.
    pub fn semester(&self) -> SemesterType {
        self.semester
    }

    /// 과목 코드를 반환합니다.
    pub fn lecture_code(&self) -> u32 {
        self.lecture_code
    }

    /// 과목명을 반환합니다.
    pub fn lecture_name(&self) -> &str {
        &self.lecture_name
    }

    /// 강의의 학점을 반환합니다.
    pub fn points(&self) -> f32 {
        self.points
    }

    /// 강의 담당 교수명을 반환합니다.
    pub fn professor(&self) -> &str {
        &self.professor
    }

    /// 교수(강사)의 단과대학을 반환합니다.
    pub fn collage(&self) -> &str {
        &self.collage
    }

    /// 교수(강사)의 학과(부)를 반환합니다.
    pub fn department(&self) -> &str {
        &self.department
    }

    /// 교수(강사)의 직책을 반환합니다.
    pub fn position(&self) -> &str {
        &self.position
    }

    /// 강의평가 점수를 반환합니다.
    pub fn score(&self) -> f32 {
        self.score
    }
}

impl<'body> FromSapTable<'body> for LectureAssessmentResult {
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
