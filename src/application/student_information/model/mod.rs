pub struct GeneralStudentInformation {
    apply_year: u32,
    student_number: u32,
}

mod academic_record;
mod bank_account;
mod family;
mod religion;
mod research_bank_account;
mod work;

pub use academic_record::StudentAcademicRecordInformation;
pub use bank_account::StudentBankAccountInformation;
pub use family::StudentFamilyInformation;
pub use religion::StudentReligionInformation;
pub use research_bank_account::StudentResearchBankAccountInformation;
pub use work::StudentWorkInformation;
