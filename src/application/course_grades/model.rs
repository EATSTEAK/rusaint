use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use getset::{CopyGetters, Getters};

/// 전체 성적(학적부, 증명)
#[derive(Getters, CopyGetters, Debug)]
#[allow(unused)]
#[get_copy = "pub"]
pub struct GradeSummary {
    /// 신청학점
    attempted_credits: f32,
    /// 취득학점
    earned_credits: f32,
    /// 평점계
    grade_points_sum: f32,
    /// 평점평균
    grade_points_avarage: f32,
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
            grade_points_avarage: cgpa,
            arithmetic_mean: avg,
            pf_earned_credits,
        }
    }
}

/// 학기별 성적
#[derive(Debug, Getters, CopyGetters)]
#[allow(unused)]
#[get_copy = "pub"]
pub struct SemesterGrade {
    /// 학년도
    year: u32,
    /// 학기
    #[getset(skip)]
    semester: String,
    /// 신청학점
    attempted_credits: f32,
    /// 취득학점
    earned_credits: f32,
    /// P/F학점
    pf_earned_credits: f32,
    /// 평점평균
    grade_points_avarage: f32,
    /// 평점계
    grade_points_sum: f32,
    /// 산술평균
    arithmetic_mean: f32,
    /// 학기석차
    semester_rank: (u32, u32),
    /// 전체석차
    general_rank: (u32, u32),
    /// 학사경고
    academic_probation: bool,
    /// 상담
    consult: bool,
    /// 유급
    flunked: bool,
}

impl SemesterGrade {
    pub(crate) fn new(
        year: u32,
        semester: String,
        attempt_credits: f32,
        earn_credits: f32,
        pf_credits: f32,
        grade_points_avarage: f32,
        grade_points_sum: f32,
        arithmetic_mean: f32,
        semester_rank: (u32, u32),
        general_rank: (u32, u32),
        academic_probation: bool,
        consult: bool,
        flunked: bool,
    ) -> Self {
        Self {
            year,
            semester,
            attempted_credits: attempt_credits,
            earned_credits: earn_credits,
            pf_earned_credits: pf_credits,
            grade_points_avarage,
            grade_points_sum,
            arithmetic_mean,
            semester_rank,
            general_rank,
            academic_probation,
            consult,
            flunked,
        }
    }

    /// 학기
    pub fn semester(&self) -> &str {
        self.semester.as_ref()
    }
}

/// 과목별 성적
#[derive(Debug, CopyGetters)]
#[allow(unused)]
pub struct ClassGrade {
    /// 이수학년도
    year: String,
    /// 이수학기
    semester: String,
    /// 과목코드
    code: String,
    /// 과목명
    class_name: String,
    /// 과목학점
    #[getset(get_copy = "pub")]
    grade_points: f32,
    /// 성적
    #[get_copy = "pub"]
    score: ClassScore,
    /// 등급
    rank: String,
    /// 교수명
    professor: String,
    /// 상세성적
    detail: Option<HashMap<String, f32>>,
}

impl ClassGrade {
    pub(crate) fn new(
        year: String,
        semester: String,
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
    pub fn year(&self) -> &str {
        self.year.as_ref()
    }

    /// 이수학기
    pub fn semester(&self) -> &str {
        self.semester.as_ref()
    }

    /// 과목코드
    pub fn code(&self) -> &str {
        self.code.as_ref()
    }

    /// 과목명
    pub fn class_name(&self) -> &str {
        self.class_name.as_ref()
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
#[derive(Debug, Clone, Copy)]
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

/// 과목 점수
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub enum ClassScore {
    /// P/F 과목의 Pass
    Pass,
    /// P/F 과목의 Failed
    Failed,
    /// 일반 과목의 점수
    Score(u32),
}

impl FromStr for ClassScore {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "P" => Self::Pass,
            "F" => Self::Failed,
            _ => Self::Score(s.parse::<u32>()?),
        })
    }
}
