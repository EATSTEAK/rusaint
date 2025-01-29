use rusaint::model::SemesterType;

#[derive(uniffi::Record)]
pub struct YearSemester {
    year: u32,
    semester: SemesterType,
}

impl YearSemester {
    pub fn new(year: u32, semester: SemesterType) -> YearSemester {
        YearSemester { year, semester }
    }
}
