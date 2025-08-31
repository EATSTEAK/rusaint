/// 학생 성적 조회: [`CourseGradesApplication`](course_grades::CourseGradesApplication)
pub mod course_grades;

/// 강의시간표: [`CourseScheduleApplication`](course_schedule::CourseScheduleApplication)
pub mod course_schedule;

/// 졸업사정표: [`GraduationRequirementsApplication`](graduation_requirements::GraduationRequirementsApplication)
pub mod graduation_requirements;

/// 학생 정보 조회: [`StudentInformationApplication`](student_information::StudentInformationApplication)
pub mod student_information;

/// 채플 정보 조회: [`ChapelApplication`](chapel::ChapelApplication)
pub mod chapel;

/// 개인 수업 시간표 조회: [`PersonalCourseScheduleApplication`](personal_course_schedule::PersonalCourseScheduleApplication)
pub mod personal_course_schedule;

/// 강의평가 조회: [`LectureAssessmentApplication`](lecture_assessment::LectureAssessmentApplication)
pub mod lecture_assessment;

/// 장학금 수혜내역 조회: [`ScholarshipsApplication`](scholarships::ScholarshipsApplication)
pub mod scholarships;

/// 플랫폼 지원을 위한 데이터
pub mod model;
