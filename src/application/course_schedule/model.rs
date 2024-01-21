pub enum CourseCategory {
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
