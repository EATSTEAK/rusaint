use ozra::types::{DataSet, FieldValue};
use serde::{Deserialize, Serialize};

use crate::{ApplicationError, RusaintError};

/// OZ `ET_BOOKED` 데이터셋 기준 수강신청 과목 정보
#[allow(unused)]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct RegisteredLecture {
    /// 계획
    pub syllabus: Option<String>,
    /// 이수구분
    pub category: String,
    /// 다전공구분
    pub sub_category: Option<String>,
    /// 공학인증
    pub abeek_info: Option<String>,
    /// 교과영역
    pub field: Option<String>,
    /// 과목번호 (`SE_SHORT`)
    pub code: String,
    /// 과목명
    pub name: String,
    /// 분반
    pub division: Option<String>,
    /// 교수명
    pub professor: String,
    /// 시간/학점(설계)
    pub time_points: String,
    /// 강의시간(강의실)
    pub schedule_room: String,
    /// 원본 과목 ID (`SM_OBJID`)
    pub sm_objid: String,
    /// 원본 분반 ID (`SE_OBJID`)
    pub se_objid: String,
    /// 전체 과목명
    pub full_name: String,
    /// 다전공 정보
    pub multi_major_info: String,
    /// 과정 코드 (`PROGC_VAR`)
    pub program_code: String,
    /// 과정명 (`PROGC_VART`)
    pub program_title: String,
    /// 수강신청일
    pub registration_date: String,
    /// 수강신청시각
    pub registration_time: String,
    /// 비고 (`REMARK`)
    pub remark: String,
}

fn get_string_field(row: &[(String, FieldValue)], field_name: &str) -> String {
    row.iter()
        .find(|(name, _)| name == field_name)
        .map(|(_, val)| val.to_string_repr())
        .unwrap_or_default()
}

fn find_dataset<'a>(datasets: &'a [DataSet], name: &str) -> &'a [Vec<(String, FieldValue)>] {
    datasets
        .iter()
        .find(|(n, _)| n == name)
        .map(|(_, rows)| rows.as_slice())
        .unwrap_or(&[])
}

impl RegisteredLecture {
    /// OZ DataModule의 데이터셋으로부터 [`RegisteredLecture`] 목록을 생성합니다.
    pub fn from_datasets(datasets: &[DataSet]) -> Result<Vec<Self>, RusaintError> {
        let lectures: Vec<Self> = find_dataset(datasets, "ET_BOOKED")
            .iter()
            .map(|row| Self {
                syllabus: None,
                category: get_string_field(row, "CATEGORY"),
                sub_category: None,
                abeek_info: Some(get_string_field(row, "ABEEK_INFO")).filter(|s| !s.is_empty()),
                field: None,
                code: get_string_field(row, "SE_SHORT"),
                name: get_string_field(row, "SE_STEXT"),
                division: Some(get_string_field(row, "BUNBAN")).filter(|s| !s.is_empty()),
                professor: get_string_field(row, "PROF_NM"),
                time_points: get_string_field(row, "TIME_CREDIT"),
                schedule_room: get_string_field(row, "LEC_TIME_ROOM"),
                sm_objid: get_string_field(row, "SM_OBJID"),
                se_objid: get_string_field(row, "SE_OBJID"),
                full_name: get_string_field(row, "LONG_NAME"),
                multi_major_info: get_string_field(row, "MULTI"),
                program_code: get_string_field(row, "PROGC_VAR"),
                program_title: get_string_field(row, "PROGC_VART"),
                registration_date: get_string_field(row, "BOOKDATE"),
                registration_time: get_string_field(row, "BOOKTIME"),
                remark: get_string_field(row, "REMARK"),
            })
            .collect();

        if lectures.is_empty() {
            return Err(ApplicationError::NoLectureResult.into());
        }

        Ok(lectures)
    }
}
