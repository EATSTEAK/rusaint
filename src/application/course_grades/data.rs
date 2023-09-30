use std::collections::HashMap;

#[derive(Debug)]
#[allow(unused)]
pub struct GradeSummary {
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

impl GradeSummary {
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
    ) -> GradeSummary {
        GradeSummary {
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

#[derive(Debug)]
#[allow(unused)]
pub struct ClassGrade {
    year: String,
    semester: String,
    code: String,
    class_name: String,
    grade_points: f32,
    score: ClassScore,
    rank: String,
    professor: String,
    detail: HashMap<String, f32>,
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
        detail: HashMap<String, f32>,
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
