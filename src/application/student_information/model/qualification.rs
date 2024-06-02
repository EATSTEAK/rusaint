use crate::{
    define_elements,
    webdynpro::{
        client::body::Body,
        element::{definition::ElementDefinition, text::InputField},
        error::WebDynproError,
    },
};

pub struct StudentQualificationInformation {
    teaching_major: Option<StudentTeachingMajorInformation>,
    teaching_plural_major: Option<StudentTeachingPluralMajorInformation>,
    lifelong: Option<StudentLifelongInformation>,
    forign_study: Option<StudentForignStudyInformation>,
}

impl<'a> StudentQualificationInformation {
    pub(crate) fn from_body(body: &'a Body) -> StudentQualificationInformation {
        Self {
            teaching_major: StudentTeachingMajorInformation::from_body(body).ok(),
            teaching_plural_major: StudentTeachingPluralMajorInformation::from_body(body).ok(),
            lifelong: StudentLifelongInformation::from_body(body).ok(),
            forign_study: StudentForignStudyInformation::from_body(body).ok(),
        }
    }
}

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

    pub(crate) fn from_body(
        body: &'a Body,
    ) -> Result<StudentTeachingMajorInformation, WebDynproError> {
        Ok(Self {
            major_name: Self::MAJOR_OTYPE
                .from_body(body)?
                .value()
                .map(str::to_string),
            qualification_number: Self::MAJOR_QUAL_NUM
                .from_body(body)?
                .value()
                .map(str::to_string),
            initiciation_date: Self::MAJOR_SELECT_DT
                .from_body(body)?
                .value()
                .map(str::to_string),
            qualification_date: Self::MAJOR_QUAL_DT
                .from_body(body)?
                .value()
                .map(str::to_string),
        })
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

    pub(crate) fn from_body(
        body: &'a Body,
    ) -> Result<StudentTeachingPluralMajorInformation, WebDynproError> {
        Ok(Self {
            major_name: Self::DOUBLE_OTYPE
                .from_body(body)?
                .value()
                .map(str::to_string),
            qualification_number: Self::DOUBLE_QUAL_NUM
                .from_body(body)?
                .value()
                .map(str::to_string),
            qualification_date: Self::DOUBLEL_DT
                .from_body(body)?
                .value()
                .map(str::to_string),
        })
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

    pub(crate) fn from_body(body: &'a Body) -> Result<StudentLifelongInformation, WebDynproError> {
        Ok(Self {
            apply_date: Self::CONEDU_APP_DT
                .from_body(body)?
                .value()
                .map(str::to_string),
            lifelong_type: Self::CONEDU_TYPE
                .from_body(body)?
                .value()
                .map(str::to_string),
            qualification_number: Self::CONEDU_QUAL_NUM
                .from_body(body)?
                .value()
                .map(str::to_string),
            qualification_date: Self::CONEDU_QUAL_DT
                .from_body(body)?
                .value()
                .map(str::to_string),
        })
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

    pub(crate) fn from_body(
        body: &'a Body,
    ) -> Result<StudentForignStudyInformation, WebDynproError> {
        Ok(Self {
            approval_date: Self::APPRODATE.from_body(body)?.value().map(str::to_string),
            authentication_number: Self::AUTHEN_NO.from_body(body)?.value().map(str::to_string),
            issue_date: Self::ISSUEDATE.from_body(body)?.value().map(str::to_string),
        })
    }
}
