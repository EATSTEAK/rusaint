use std::collections::HashMap;

pub struct GradutionStudentInfo {
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

pub struct GraduationRequirementsInfo {
    is_graduatable: bool,
    requirements: HashMap<String, GraduationRequirement>,
}

pub struct GraduationRequirement {
    name: String,
    requirement: u32,
    calcuation: f32,
    difference: f32,
    result: bool,
    category: String,
    lectures: Vec<String>,
}
