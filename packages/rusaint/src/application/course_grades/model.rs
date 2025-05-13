use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use serde::{
    Deserialize, Deserializer, Serialize,
    de::{IntoDeserializer, value::MapDeserializer},
};

use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::{
    element::{complex::sap_table::FromSapTable, definition::ElementDefinition},
    error::{ElementError, WebDynproError},
};
use crate::{
    application::utils::de_with::{
        deserialize_empty, deserialize_f32_string, deserialize_semester_type,
        deserialize_u32_string,
    },
    model::SemesterType,
};

/// 전체 성적(학적부, 증명)
#[derive(Serialize, Deserialize, Debug)]
#[allow(unused)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct GradeSummary {
    /// 신청학점
    attempted_credits: f32,
    /// 취득학점
    earned_credits: f32,
    /// 평점계
    grade_points_sum: f32,
    /// 평점평균
    grade_points_average: f32,
    /// 산술평균
    arithmetic_mean: f32,
    /// P/F 학점계
    pf_earned_credits: f32,
}
impl GradeSummary {
    pub(crate) fn new(
        attempted_credits: f32,
        earned_credits: f32,
        gpa: f32,
        cgpa: f32,
        avg: f32,
        pf_earned_credits: f32,
    ) -> GradeSummary {
        GradeSummary {
            attempted_credits,
            earned_credits,
            grade_points_sum: gpa,
            grade_points_average: cgpa,
            arithmetic_mean: avg,
            pf_earned_credits,
        }
    }

    /// 신청학점
    pub fn attempted_credits(&self) -> f32 {
        self.attempted_credits
    }

    /// 취득학점
    pub fn earned_credits(&self) -> f32 {
        self.earned_credits
    }

    /// 평점계
    pub fn grade_points_sum(&self) -> f32 {
        self.grade_points_sum
    }

    /// 평점평균
    pub fn grade_points_average(&self) -> f32 {
        self.grade_points_average
    }

    /// 산술평균
    pub fn arithmetic_mean(&self) -> f32 {
        self.arithmetic_mean
    }

    /// P/F 학점계
    pub fn pf_earned_credits(&self) -> f32 {
        self.pf_earned_credits
    }
}

/// 학기별 성적
#[derive(Debug, Serialize, Deserialize)]
#[allow(unused)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct SemesterGrade {
    /// 학년도
    #[serde(
        rename(deserialize = "학년도"),
        deserialize_with = "deserialize_u32_string"
    )]
    year: u32,
    /// 학기
    #[serde(
        rename(deserialize = "학기"),
        deserialize_with = "deserialize_semester_type"
    )]
    semester: SemesterType,
    /// 신청학점
    #[serde(
        rename(deserialize = "신청학점"),
        deserialize_with = "deserialize_f32_string"
    )]
    attempted_credits: f32,
    /// 취득학점
    #[serde(
        rename(deserialize = "취득학점"),
        deserialize_with = "deserialize_f32_string"
    )]
    earned_credits: f32,
    /// P/F학점
    #[serde(
        rename(deserialize = "P/F학점"),
        deserialize_with = "deserialize_f32_string"
    )]
    pf_earned_credits: f32,
    /// 평점평균
    #[serde(
        rename(deserialize = "평점평균"),
        deserialize_with = "deserialize_f32_string"
    )]
    grade_points_average: f32,
    /// 평점계
    #[serde(
        rename(deserialize = "평점계"),
        deserialize_with = "deserialize_f32_string"
    )]
    grade_points_sum: f32,
    /// 산술평균
    #[serde(
        rename(deserialize = "산술평균"),
        deserialize_with = "deserialize_f32_string"
    )]
    arithmetic_mean: f32,
    /// 학기별석차
    #[serde(
        rename(deserialize = "학기별석차"),
        deserialize_with = "deserialize_rank"
    )]
    semester_rank: (u32, u32),
    /// 전체석차
    #[serde(
        rename(deserialize = "전체석차"),
        deserialize_with = "deserialize_rank"
    )]
    general_rank: (u32, u32),
    /// 학사경고
    #[serde(
        rename(deserialize = "학사경고"),
        default = "return_false",
        deserialize_with = "deserialize_empty"
    )]
    academic_probation: bool,
    /// 상담여부
    #[serde(
        rename(deserialize = "상담여부"),
        deserialize_with = "deserialize_empty"
    )]
    consult: bool,
    /// 유급
    #[serde(rename(deserialize = "유급"), deserialize_with = "deserialize_empty")]
    flunked: bool,
}

fn return_false() -> bool {
    false
}

fn deserialize_rank<'de, D: Deserializer<'de>>(deserializer: D) -> Result<(u32, u32), D::Error> {
    let value = String::deserialize(deserializer)?;
    let mut spl = value.split("/");
    let first: u32 = spl
        .next()
        .ok_or_else(|| serde::de::Error::custom("input rank is invalid"))?
        .parse()
        .map_err(|_e| serde::de::Error::custom("input rank is not a number"))?;
    let second: u32 = spl
        .next()
        .ok_or_else(|| serde::de::Error::custom("input rank is invalid"))?
        .parse()
        .map_err(|_e| serde::de::Error::custom("input rank is not a number"))?;
    Ok((first, second))
}

impl SemesterGrade {
    /// 학년도
    pub fn year(&self) -> u32 {
        self.year
    }

    /// 학기
    pub fn semester(&self) -> SemesterType {
        self.semester
    }

    /// 취득학점
    pub fn earned_credits(&self) -> f32 {
        self.earned_credits
    }

    /// P/F학점
    pub fn pf_earned_credits(&self) -> f32 {
        self.pf_earned_credits
    }

    /// 평점평균
    pub fn grade_points_average(&self) -> f32 {
        self.grade_points_average
    }

    /// 평점계
    pub fn grade_points_sum(&self) -> f32 {
        self.grade_points_sum
    }

    /// 산술평균
    pub fn arithmetic_mean(&self) -> f32 {
        self.arithmetic_mean
    }

    /// 학기별석차
    pub fn semester_rank(&self) -> (u32, u32) {
        self.semester_rank
    }

    /// 전체석차
    pub fn general_rank(&self) -> (u32, u32) {
        self.general_rank
    }

    /// 학사경고
    pub fn academic_probation(&self) -> bool {
        self.academic_probation
    }

    /// 상담여부
    pub fn consult(&self) -> bool {
        self.consult
    }

    /// 유급
    pub fn flunked(&self) -> bool {
        self.flunked
    }
}

impl<'body> FromSapTable<'body> for SemesterGrade {
    fn from_table(
        header: &'body crate::webdynpro::element::complex::sap_table::SapTableHeader,
        row: &'body crate::webdynpro::element::complex::sap_table::SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, WebDynproError> {
        let map_string = row.try_row_into::<HashMap<String, String>>(header, parser)?;
        let map_de: MapDeserializer<_, serde::de::value::Error> = map_string.into_deserializer();
        Ok(
            SemesterGrade::deserialize(map_de).map_err(|e| ElementError::InvalidContent {
                element: row.table_def().id().to_string(),
                content: e.to_string(),
            })?,
        )
    }
}

/// 과목별 성적
#[derive(Debug, Serialize, Deserialize)]
#[allow(unused)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct ClassGrade {
    /// 이수학년도
    year: u32,
    /// 이수학기
    semester: SemesterType,
    /// 과목코드
    code: String,
    /// 과목명
    class_name: String,
    /// 과목학점
    grade_points: f32,
    /// 성적
    score: ClassScore,
    /// 등급
    rank: String,
    /// 교수명
    professor: String,
    /// 상세성적
    detail: Option<HashMap<String, f32>>,
}

impl ClassGrade {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        year: u32,
        semester: SemesterType,
        code: String,
        class_name: String,
        grade_points: f32,
        score: ClassScore,
        rank: String,
        professor: String,
        detail: Option<HashMap<String, f32>>,
    ) -> ClassGrade {
        ClassGrade {
            year,
            semester,
            code,
            class_name,
            grade_points,
            score,
            rank,
            professor,
            detail,
        }
    }

    /// 이수학년도
    pub fn year(&self) -> u32 {
        self.year
    }

    /// 이수학기
    pub fn semester(&self) -> SemesterType {
        self.semester
    }

    /// 과목코드
    pub fn code(&self) -> &str {
        self.code.as_ref()
    }

    /// 과목명
    pub fn class_name(&self) -> &str {
        self.class_name.as_ref()
    }

    /// 과목학점
    pub fn grade_points(&self) -> f32 {
        self.grade_points
    }

    /// 성적
    pub fn score(&self) -> ClassScore {
        self.score
    }

    /// 등급
    pub fn rank(&self) -> &str {
        self.rank.as_ref()
    }

    /// 교수명
    pub fn professor(&self) -> &str {
        self.professor.as_ref()
    }

    /// 상세성적
    pub fn detail(&self) -> Option<&HashMap<String, f32>> {
        self.detail.as_ref()
    }
}

/// 학위과정
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[allow(unused)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum CourseType {
    /// 박사과정
    Phd, // DR
    /// 석사과정
    Master, // MA
    /// 석박과정
    PhdIntergrated, // MP
    /// 연구과정
    Research, // RE
    /// 학사과정
    Bachelor, // UG
}

/// 과목 점수
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[allow(unused)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum ClassScore {
    /// P/F 과목의 Pass
    Pass,
    /// P/F 과목의 Failed
    Failed,
    /// 일반 과목의 점수
    Score(u32),
    /// 성적 없음
    Empty,
}

impl FromStr for ClassScore {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "P" => Self::Pass,
            "F" => Self::Failed,
            "" => Self::Empty,
            _ => Self::Score(s.parse::<u32>()?),
        })
    }
}
