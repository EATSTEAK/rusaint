
/// 학위과정
#[derive(uniffi::Enum, Debug, Clone, Copy)]
#[allow(unused)]
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

impl From<CourseType> for rusaint::application::course_grades::model::CourseType {
    fn from(value: CourseType) -> Self {
        match value {
            CourseType::Phd => Self::Phd,
            CourseType::Master => Self::Master,
            CourseType::PhdIntergrated => Self::PhdIntergrated,
            CourseType::Research => Self::Research,
            CourseType::Bachelor => Self::Bachelor,
        }
    }
}

/// 전체 성적(학적부, 증명)
#[derive(uniffi::Record, Debug)]
pub struct GradeSummary {
    /// 신청학점
    pub attempted_credits: f32,
    /// 취득학점
    pub earned_credits: f32,
    /// 평점계
    pub grade_points_sum: f32,
    /// 평점평균
    pub grade_points_avarage: f32,
    /// 산술평균
    pub arithmetic_mean: f32,
    /// P/F 학점계
    pub pf_earned_credits: f32,
}

impl From<rusaint::application::course_grades::model::GradeSummary> for GradeSummary {
    fn from(value: rusaint::application::course_grades::model::GradeSummary) -> Self {
        Self {
            attempted_credits: value.attempted_credits(),
            earned_credits: value.earned_credits(),
            grade_points_sum: value.grade_points_sum(),
            grade_points_avarage: value.grade_points_avarage(),
            arithmetic_mean: value.arithmetic_mean(),
            pf_earned_credits: value.pf_earned_credits(),
        }
    }
}