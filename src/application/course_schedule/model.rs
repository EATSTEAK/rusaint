pub enum LectureCategory {
    Major {
        collage: String,
        department: String,
        major: Option<String>,
    },
    RequiredElective {
        lecture_name: String,
    },
    OptionalElective {
        category: String,
    },
    Chapel {
        lecture_name: String,
    },
    Education,
    LifelongLearning,
    StandardSelection,
    Graduated,
    ConnectedMajor {
        major: String,
    },
    UnitedMajor {
        major: String,
    },
    FindByProfessor {
        keyword: String,
    },
    FindByLecture {
        keyword: String,
    },
    RecognizedOtherMajor {
        collage: String,
        department: String,
        major: Option<String>,
    },
    DualListing,
    Cyber,
}

pub struct Lecture {
  syllabus: Option<String>,
  category: String,
  sub_category: String,
  abeek_info: String,
  field: String,
  code: String,
  name: String,
  division: String,
  professor: String,
  department: String,
  time_points: String,
  personeel: String,
  remaining_seats: String,
  schedule_room: String,
  target: String
}