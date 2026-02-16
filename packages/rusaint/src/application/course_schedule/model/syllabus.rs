use serde::{Deserialize, Serialize};

use ozra::types::{DataSet, FieldValue};

use crate::ApplicationError;
use crate::RusaintError;

/// 강의계획서 - Full lecture syllabus information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct LectureSyllabus {
    /// 교과목명
    pub course_name: String,
    /// 담당교수
    pub professor: String,
    /// 과목코드
    pub course_code: String,
    /// 학년도
    pub year: String,
    /// 학기
    pub semester: String,
    /// 학점
    pub credits: String,
    /// 강의개요
    pub abstract_text: String,
    /// 수업방법
    pub teaching_method: String,
    /// 주교재
    pub main_textbook: String,
    /// 부교재
    pub sub_textbook: String,
    /// 교수 전화번호
    pub professor_phone: String,
    /// 교수 이메일
    pub professor_email: String,
    /// 상담시간
    pub office_hours: String,
    /// 수강대상
    pub target_students: String,
    /// 이수구분
    pub designation: String,
    /// 결석처리
    pub absence_policy: String,
    /// 성적평가 항목
    pub grading_items: Vec<SyllabusGradingItem>,
    /// 학습목표
    pub learning_objectives: Vec<String>,
    /// 주차별 수업계획
    pub weekly_schedule: Vec<SyllabusWeeklyPlan>,
    /// 핵심역량
    pub competencies: Vec<SyllabusCompetency>,
}

/// 성적평가 항목
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct SyllabusGradingItem {
    /// 평가항목명
    pub name: String,
    /// 비율 (%)
    pub rate: String,
}

/// 주차별 수업계획
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct SyllabusWeeklyPlan {
    /// 주차
    pub week: String,
    /// 핵심주제
    pub topic: String,
    /// 세부내용
    pub details: String,
    /// 수업방법
    pub teaching_method: String,
}

/// 핵심역량
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct SyllabusCompetency {
    /// 역량명
    pub name: String,
    /// 관련비율 (%)
    pub rate: String,
}

/// OZ DataModule row에서 주어진 필드명에 해당하는 값을 문자열로 추출합니다.
/// 필드가 존재하지 않으면 빈 문자열을 반환합니다.
fn get_string_field(row: &[(String, FieldValue)], field_name: &str) -> String {
    row.iter()
        .find(|(name, _)| name == field_name)
        .map(|(_, val)| val.to_string_repr())
        .unwrap_or_default()
}

/// OZ DataModule 응답에서 주어진 이름의 데이터셋 행들을 찾아 반환합니다.
/// 데이터셋이 존재하지 않으면 빈 슬라이스를 반환합니다.
fn find_dataset<'a>(datasets: &'a [DataSet], name: &str) -> &'a [Vec<(String, FieldValue)>] {
    datasets
        .iter()
        .find(|(n, _)| n == name)
        .map(|(_, rows)| rows.as_slice())
        .unwrap_or(&[])
}

impl LectureSyllabus {
    /// OZ DataModule의 데이터셋으로부터 [`LectureSyllabus`]를 생성합니다.
    ///
    /// `ET_PLAN` 데이터셋에 강의계획서 메인 정보(1 row)가 포함되어 있어야 합니다.
    /// 데이터셋이 비어있으면 에러를 반환합니다.
    pub fn from_datasets(datasets: &[DataSet]) -> Result<Self, RusaintError> {
        // ET_PLAN — main syllabus info (1 row expected)
        let plan_rows = find_dataset(datasets, "ET_PLAN");
        let plan = plan_rows.first().ok_or_else(|| {
            ApplicationError::SyllabusFetchError(
                "ET_PLAN dataset is empty or missing in OZ response".to_string(),
            )
        })?;

        let course_name = get_string_field(plan, "SMTEXT");
        let professor = get_string_field(plan, "PROF_NM");
        let course_code = get_string_field(plan, "SMOBJID");
        let year = get_string_field(plan, "PERYR");
        let semester = get_string_field(plan, "PERID");
        let credits = get_string_field(plan, "PTPLAN");
        let abstract_text = get_string_field(plan, "ABSTRACT");
        let teaching_method = get_string_field(plan, "CLSWY_TEXT");
        let main_textbook = get_string_field(plan, "TXTREFER_M");
        let sub_textbook = get_string_field(plan, "TXTREFER_S");
        let professor_phone = get_string_field(plan, "PROF_TELNR");
        let professor_email = get_string_field(plan, "SMTPADR");
        let office_hours = get_string_field(plan, "COUNSELTM");
        let target_students = get_string_field(plan, "BOOK_TARGET");
        let designation = get_string_field(plan, "DESIGNATION");
        let absence_policy = get_string_field(plan, "SMABSENT");

        // ET_APP — grading breakdown
        let grading_items: Vec<SyllabusGradingItem> = find_dataset(datasets, "ET_APP")
            .iter()
            .map(|row| SyllabusGradingItem {
                name: get_string_field(row, "AGRDESC"),
                rate: get_string_field(row, "RATE"),
            })
            .collect();

        // ET_GOAL — learning objectives
        let learning_objectives: Vec<String> = find_dataset(datasets, "ET_GOAL")
            .iter()
            .map(|row| get_string_field(row, "ZGOAL"))
            .collect();

        // ET_WEEK — weekly schedule
        let weekly_schedule: Vec<SyllabusWeeklyPlan> = find_dataset(datasets, "ET_WEEK")
            .iter()
            .map(|row| SyllabusWeeklyPlan {
                week: get_string_field(row, "WEEKLY"),
                topic: get_string_field(row, "COREWORD"),
                details: get_string_field(row, "DETAILS"),
                teaching_method: get_string_field(row, "REMARKT"),
            })
            .collect();

        // ET_PRO_ABLI — professional competencies
        let competencies: Vec<SyllabusCompetency> = find_dataset(datasets, "ET_PRO_ABLI")
            .iter()
            .map(|row| SyllabusCompetency {
                name: get_string_field(row, "PRO_ABLIT"),
                rate: get_string_field(row, "REL_RATE"),
            })
            .collect();

        Ok(Self {
            course_name,
            professor,
            course_code,
            year,
            semester,
            credits,
            abstract_text,
            teaching_method,
            main_textbook,
            sub_textbook,
            professor_phone,
            professor_email,
            office_hours,
            target_students,
            designation,
            absence_policy,
            grading_items,
            learning_objectives,
            weekly_schedule,
            competencies,
        })
    }
}
