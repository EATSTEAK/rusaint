use crate::{define_elements, webdynpro::element::text::InputField};

pub struct StudentQualificationInformation {
    teaching_major: Option<StudentTeachingMajorInformation>,
    teaching_plural_major: Option<StudentTeachingPluralMajorInformation>,
    lifelong: Option<StudentLifelongInformation>,
    forign_study: Option<StudentForignStudyInformation>,
}

impl<'a> StudentQualificationInformation {}

pub struct StudentTeachingMajorInformation {
    major_name: Option<String>,
    qualification_number: Option<String>,
    initiciation_date: Option<String>,
    qualification_date: Option<String>,
}

impl<'a> StudentTeachingMajorInformation {
    // 교직(주전공)
    define_elements! {
        // 표시과목
        MAJOR_OTYPE: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_MAJOR_OTYPE";
        // 교원자격증번호
        MAJOR_QUAL_NUM: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_MAJOR_QUAL_NUM";
        // 선발일자
        MAJOR_SELECT_DT: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_MAJOR_SELECT_DT";
        // 교원자격증 발급일자
        MAJOR_QUAL_DT: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_MAJOR_QUAL_DT";
    }
}

pub struct StudentTeachingPluralMajorInformation {
    major_name: Option<String>,
    qualification_number: Option<String>,
    qualification_date: Option<String>,
}

impl<'a> StudentTeachingPluralMajorInformation {
    // 교직(복수전공)
    define_elements! {
      // 표시과목
      DOUBLE_OTYPE: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_DOUBLE_OTYPE";
      // 교원자격증번호
      DOUBLE_QUAL_NUM: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_DOUBLE_QUAL_NUM";
      // 교원자격증 발급일자
      DOUBLEL_DT: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_DOUBLEL_DT";
    }
}

pub struct StudentLifelongInformation {
    apply_date: Option<String>,
    lifelong_type: Option<String>,
    qualification_number: Option<String>,
    qualification_date: Option<String>,
}

impl<'a> StudentLifelongInformation {
    // 평생교육사
    define_elements! {
      // 신청일자
      CONEDU_APP_DT: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_CONEDU_APP_DT";
      // 자격구분
      CONEDU_TYPE: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_CONEDU_TYPE";
      // 자격증번호
      CONEDU_QUAL_NUM: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_CONEDU_QUAL_NUM";
      // 자격증 발급일자
      CONEDU_QUAL_DT: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_CONEDU_QUAL_DT";
    }
}

pub struct StudentForignStudyInformation {
    approval_date: Option<String>,
    authentication_number: Option<String>,
    issue_date: Option<String>,
}

impl<'a> StudentForignStudyInformation {
    // 7+1 프로그램
    define_elements! {
      // 신청일자
      APPRODATE: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.APPRODATE";
      // 인증서번호
      AUTHEN_NO: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.AUTHEN_NO";
      // 발급일자
      ISSUEDATE: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.ISSUEDATE";
    }
}
