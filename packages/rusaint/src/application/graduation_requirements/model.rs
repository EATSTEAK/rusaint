use std::collections::HashMap;

use serde::{
    Deserialize, Deserializer, Serialize,
    de::{IntoDeserializer, value::MapDeserializer},
};

use crate::application::utils::de_with::deserialize_with_trim;
use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::{
    element::{complex::sap_table::FromSapTable, definition::ElementDefinition},
    error::ElementError,
};

#[derive(Debug)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Serialize, Deserialize)]
/// 졸업 학생 정보
pub struct GraduationStudent {
    number: u32,
    name: String,
    grade: u32,
    semester: u32,
    status: String,
    apply_year: u32,
    apply_type: String,
    department: String,
    majors: Vec<String>,
    audit_date: String,
    graduation_points: f32,
    completed_points: f32,
}

impl GraduationStudent {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        number: u32,
        name: &str,
        grade: u32,
        semester: u32,
        status: &str,
        apply_year: u32,
        apply_type: &str,
        department: &str,
        majors: Vec<String>,
        audit_date: &str,
        graduation_points: f32,
        completed_points: f32,
    ) -> Self {
        Self {
            number,
            name: name.to_owned(),
            grade,
            semester,
            status: status.to_owned(),
            apply_year,
            apply_type: apply_type.to_owned(),
            department: department.to_owned(),
            majors,
            audit_date: audit_date.to_owned(),
            graduation_points,
            completed_points,
        }
    }

    /// 학번
    pub fn number(&self) -> u32 {
        self.number
    }

    /// 성명
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 학년
    pub fn grade(&self) -> u32 {
        self.grade
    }

    /// 이수학기
    pub fn semester(&self) -> u32 {
        self.semester
    }

    /// 학적상태
    pub fn status(&self) -> &str {
        &self.status
    }

    /// 입학년도
    pub fn apply_year(&self) -> u32 {
        self.apply_year
    }

    /// 입학유형
    pub fn apply_type(&self) -> &str {
        &self.apply_type
    }

    /// 학부
    pub fn department(&self) -> &str {
        &self.department
    }

    /// 제1~4 전공
    pub fn majors(&self) -> &[String] {
        &self.majors
    }

    /// 졸업사정일자
    pub fn audit_date(&self) -> &str {
        &self.audit_date
    }

    /// 졸업학점
    pub fn graduation_points(&self) -> f32 {
        self.graduation_points
    }

    /// 인정학점
    pub fn completed_points(&self) -> f32 {
        self.completed_points
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// 전체 졸업 요건 정보
pub struct GraduationRequirements {
    is_graduatable: bool,
    requirements: HashMap<String, GraduationRequirement>,
}

impl GraduationRequirements {
    pub(super) fn new(
        is_graduatable: bool,
        requirements: HashMap<String, GraduationRequirement>,
    ) -> Self {
        Self {
            is_graduatable,
            requirements,
        }
    }

    /// 졸업사정결과
    pub fn is_graduatable(&self) -> bool {
        self.is_graduatable
    }

    /// 졸업요건
    pub fn requirements(&self) -> &HashMap<String, GraduationRequirement> {
        &self.requirements
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// 졸업 요건
pub struct GraduationRequirement {
    #[serde(
        rename(deserialize = "졸업요건"),
        deserialize_with = "deserialize_with_trim"
    )]
    name: String,
    #[serde(
        rename(deserialize = "기준값"),
        deserialize_with = "deserialize_option_u32_string"
    )]
    requirement: Option<u32>,
    #[serde(
        rename(deserialize = "계산값"),
        deserialize_with = "deserialize_option_f32_string"
    )]
    calcuation: Option<f32>,
    #[serde(
        rename(deserialize = "계산값 - 기준값"),
        deserialize_with = "deserialize_option_f32_string"
    )]
    difference: Option<f32>,
    #[serde(
        rename(deserialize = "결과"),
        deserialize_with = "deserialize_sufficiency"
    )]
    result: bool,
    #[serde(rename(deserialize = "이수구분"))]
    category: String,
    #[serde(
        rename(deserialize = "과목사용"),
        deserialize_with = "deserialize_lectures"
    )]
    lectures: Vec<String>,
}

impl GraduationRequirement {
    /// 졸업요건
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 기준값
    pub fn requirement(&self) -> Option<u32> {
        self.requirement
    }

    /// 계산값
    pub fn calcuation(&self) -> Option<f32> {
        self.calcuation
    }

    /// 계산값 - 기준값
    pub fn difference(&self) -> Option<f32> {
        self.difference
    }

    /// 결과
    pub fn result(&self) -> bool {
        self.result
    }

    /// 이수구분
    pub fn category(&self) -> &str {
        &self.category
    }

    /// 과목사용
    pub fn lectures(&self) -> &[String] {
        &self.lectures
    }
}

pub(crate) fn deserialize_option_u32_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<u32>, D::Error> {
    let value = String::deserialize(deserializer)?;
    if value.trim().is_empty() {
        return Ok(None);
    }
    value
        .trim()
        .parse()
        .map(Some)
        .map_err(serde::de::Error::custom)
}

pub(crate) fn deserialize_option_f32_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<f32>, D::Error> {
    let value = String::deserialize(deserializer)?;
    if value.trim().is_empty() {
        return Ok(None);
    }
    value
        .trim()
        .parse()
        .map(Some)
        .map_err(serde::de::Error::custom)
}

fn deserialize_sufficiency<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    let value = String::deserialize(deserializer)?;
    Ok(value.trim() == "충족")
}

fn deserialize_lectures<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<String>, D::Error> {
    let value = String::deserialize(deserializer)?;
    Ok(value
        .split(", ")
        .map(str::to_owned)
        .collect::<Vec<String>>())
}

impl<'body> FromSapTable<'body> for GraduationRequirement {
    fn from_table(
        header: &'body crate::webdynpro::element::complex::sap_table::SapTableHeader,
        row: &'body crate::webdynpro::element::complex::sap_table::SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, crate::webdynpro::error::WebDynproError> {
        let map_string = row.try_row_into::<HashMap<String, String>>(header, parser)?;
        let map_de: MapDeserializer<_, serde::de::value::Error> = map_string.into_deserializer();
        Ok(GraduationRequirement::deserialize(map_de).map_err(|e| {
            ElementError::InvalidContent {
                element: row.table_def().id().to_string(),
                content: e.to_string(),
            }
        })?)
    }
}
