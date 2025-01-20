use crate::webdynpro::command::WebDynproCommandExecutor;
use crate::webdynpro::element::parser::ElementParser;
use crate::{
    define_elements,
    webdynpro::{
        command::element::text::InputFieldValueCommand, element::text::InputField,
        error::WebDynproError,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// 학생의 자격(교직이수, 평생교육사, 7+1 프로그램) 정보
pub struct StudentQualification {
    teaching_major: Option<StudentTeachingMajorInformation>,
    teaching_plural_major: Option<StudentTeachingPluralMajorInformation>,
    lifelong: Option<StudentLifelongInformation>,
    forign_study: Option<StudentForignStudyInformation>,
}

impl StudentQualification {
    pub(crate) fn with_parser(parser: &ElementParser) -> StudentQualification {
        Self {
            teaching_major: StudentTeachingMajorInformation::with_parser(parser).ok(),
            teaching_plural_major: StudentTeachingPluralMajorInformation::with_parser(parser).ok(),
            lifelong: StudentLifelongInformation::with_parser(parser).ok(),
            forign_study: StudentForignStudyInformation::with_parser(parser).ok(),
        }
    }

    /// 교직(주전공) 정보를 반환합니다.
    pub fn teaching_major(&self) -> Option<&StudentTeachingMajorInformation> {
        self.teaching_major.as_ref()
    }

    /// 교직(복수전공) 정보를 반환합니다.
    pub fn teaching_plural_major(&self) -> Option<&StudentTeachingPluralMajorInformation> {
        self.teaching_plural_major.as_ref()
    }

    /// 평생교육사 정보를 반환합니다.
    pub fn lifelong(&self) -> Option<&StudentLifelongInformation> {
        self.lifelong.as_ref()
    }

    /// 7+1 프로그램 정보를 반환합니다.
    pub fn forign_study(&self) -> Option<&StudentForignStudyInformation> {
        self.forign_study.as_ref()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// 교직이수(주전공) 정보
pub struct StudentTeachingMajorInformation {
    major_name: Option<String>,
    qualification_number: Option<String>,
    initiation_date: Option<String>,
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

    pub(crate) fn with_parser(
        parser: &'a ElementParser,
    ) -> Result<StudentTeachingMajorInformation, WebDynproError> {
        Ok(Self {
            major_name: parser
                .read(InputFieldValueCommand::new(Self::MAJOR_OTYPE))
                .ok(),
            qualification_number: parser
                .read(InputFieldValueCommand::new(Self::MAJOR_QUAL_NUM))
                .ok(),
            initiation_date: parser
                .read(InputFieldValueCommand::new(Self::MAJOR_SELECT_DT))
                .ok(),
            qualification_date: parser
                .read(InputFieldValueCommand::new(Self::MAJOR_QUAL_DT))
                .ok(),
        })
    }

    /// 표시과목을 반환합니다.
    pub fn major_name(&self) -> Option<&str> {
        self.major_name.as_deref()
    }

    /// 교원자격증번호를 반환합니다.
    pub fn qualification_number(&self) -> Option<&str> {
        self.qualification_number.as_deref()
    }

    /// 선발일자를 반환합니다.
    pub fn initiation_date(&self) -> Option<&str> {
        self.initiation_date.as_deref()
    }

    /// 교원자격증 발급일자를 반환합니다.
    pub fn qualification_date(&self) -> Option<&str> {
        self.qualification_date.as_deref()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// 교직이수(복수전공) 정보
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

    pub(crate) fn with_parser(
        parser: &'a ElementParser,
    ) -> Result<StudentTeachingPluralMajorInformation, WebDynproError> {
        Ok(Self {
            major_name: parser
                .read(InputFieldValueCommand::new(Self::DOUBLE_OTYPE))
                .ok(),
            qualification_number: parser
                .read(InputFieldValueCommand::new(Self::DOUBLE_QUAL_NUM))
                .ok(),
            qualification_date: parser
                .read(InputFieldValueCommand::new(Self::DOUBLEL_DT))
                .ok(),
        })
    }

    /// 표시과목을 반환합니다.
    pub fn major_name(&self) -> Option<&str> {
        self.major_name.as_deref()
    }

    /// 교원자격증번호를 반환합니다.
    pub fn qualification_number(&self) -> Option<&str> {
        self.qualification_number.as_deref()
    }

    /// 교원자격증 발급일자를 반환합니다.
    pub fn qualification_date(&self) -> Option<&str> {
        self.qualification_date.as_deref()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// 평생교육사 정보
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

    pub(crate) fn with_parser(
        parser: &'a ElementParser,
    ) -> Result<StudentLifelongInformation, WebDynproError> {
        Ok(Self {
            apply_date: parser
                .read(InputFieldValueCommand::new(Self::CONEDU_APP_DT))
                .ok(),
            lifelong_type: parser
                .read(InputFieldValueCommand::new(Self::CONEDU_TYPE))
                .ok(),
            qualification_number: parser
                .read(InputFieldValueCommand::new(Self::CONEDU_QUAL_NUM))
                .ok(),
            qualification_date: parser
                .read(InputFieldValueCommand::new(Self::CONEDU_QUAL_DT))
                .ok(),
        })
    }

    /// 신청일자를 반환합니다.
    pub fn apply_date(&self) -> Option<&str> {
        self.apply_date.as_deref()
    }

    /// 자격구분을 반환합니다.
    pub fn lifelong_type(&self) -> Option<&str> {
        self.lifelong_type.as_deref()
    }

    /// 자격증번호를 반환합니다.
    pub fn qualification_number(&self) -> Option<&str> {
        self.qualification_number.as_deref()
    }

    /// 자격증 발급일자를 반환합니다.
    pub fn qualification_date(&self) -> Option<&str> {
        self.qualification_date.as_deref()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// 7+1 프로그램 정보를 반환합니다.
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

    pub(crate) fn with_parser(
        parser: &'a ElementParser,
    ) -> Result<StudentForignStudyInformation, WebDynproError> {
        Ok(Self {
            approval_date: parser
                .read(InputFieldValueCommand::new(Self::APPRODATE))
                .ok(),
            authentication_number: parser
                .read(InputFieldValueCommand::new(Self::AUTHEN_NO))
                .ok(),
            issue_date: parser
                .read(InputFieldValueCommand::new(Self::ISSUEDATE))
                .ok(),
        })
    }

    /// 신청일자를 반환합니다.
    pub fn approval_date(&self) -> Option<&str> {
        self.approval_date.as_deref()
    }

    /// 인증서번호를 반환합니다.
    pub fn authentication_number(&self) -> Option<&str> {
        self.authentication_number.as_deref()
    }

    /// 발급일자를 반환합니다.
    pub fn issue_date(&self) -> Option<&str> {
        self.issue_date.as_deref()
    }
}
