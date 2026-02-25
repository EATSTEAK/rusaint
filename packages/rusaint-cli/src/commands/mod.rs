pub mod assessment;
pub mod chapel;
pub mod course_schedule;
pub mod create_session;
pub mod grades;
pub mod graduation;
pub mod personal_schedule;
pub mod registration;
pub mod scholarships;
pub mod student_info;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// 세션 JSON 파일 생성
    CreateSession(create_session::CreateSessionArgs),
    /// 강의시간표 조회
    CourseSchedule {
        #[command(subcommand)]
        command: course_schedule::CourseScheduleCommands,
    },
    /// 학생정보 조회
    StudentInfo {
        #[command(subcommand)]
        command: student_info::StudentInfoCommands,
    },
    /// 성적 조회
    Grades {
        #[command(subcommand)]
        command: grades::GradesCommands,
    },
    /// 채플 정보 조회
    ChapelInfo {
        #[command(subcommand)]
        command: chapel::ChapelCommands,
    },
    /// 수강신청 조회
    Registration {
        #[command(subcommand)]
        command: registration::RegistrationCommands,
    },
    /// 졸업사정표 조회
    Graduation {
        #[command(subcommand)]
        command: graduation::GraduationCommands,
    },
    /// 강의평가 조회
    Assessment {
        #[command(subcommand)]
        command: assessment::AssessmentCommands,
    },
    /// 개인시간표 조회
    PersonalSchedule {
        #[command(subcommand)]
        command: personal_schedule::PersonalScheduleCommands,
    },
    /// 장학금 조회
    Scholarships {
        #[command(subcommand)]
        command: scholarships::ScholarshipsCommands,
    },
}
