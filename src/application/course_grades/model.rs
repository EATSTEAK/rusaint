use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use getset::Getters;

#[derive(Getters, Debug)]
#[allow(unused)]
#[get = "pub"]
pub struct GradeSummary {
    attempted_credits: f32,
    earned_credits: f32,
    gpa: f32,
    cgpa: f32,
    avg: f32,
    pf_earned_credits: f32
}
impl GradeSummary {
    pub(crate) fn new(attempted_credits: f32, earned_credits: f32, gpa: f32, cgpa: f32, avg: f32, pf_earned_credits: f32) -> GradeSummary {
        GradeSummary {
            attempted_credits,
            earned_credits,
            gpa,
            cgpa,
            avg,
            pf_earned_credits
        }
    }
}



#[derive(Debug, Getters)]
#[allow(unused)]
#[get = "pub"]
pub struct SemesterGrade {
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
}

impl SemesterGrade {
    pub fn new(
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
            attempt_credits,
            earn_credits,
            pf_credits,
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
}

#[derive(Debug, Getters)]
#[allow(unused)]
#[get = "pub"]
pub struct ClassGrade {
    year: String,
    semester: String,
    code: String,
    class_name: String,
    grade_points: f32,
    score: ClassScore,
    rank: String,
    professor: String,
    detail: Option<HashMap<String, f32>>,
}

impl ClassGrade {
    pub fn new(
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
}

#[derive(Debug)]
#[allow(unused)]
pub enum ClassScore {
    Pass,
    Failed,
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
