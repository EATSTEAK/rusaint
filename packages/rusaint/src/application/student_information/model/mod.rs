use crate::{
    define_elements,
    webdynpro::{
        command::element::text::InputFieldValueCommand,
        element::{action::Button, graphic::Image, text::InputField},
        error::WebDynproError,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// 기본 학생 정보
pub struct StudentInformation {
    apply_year: u32,
    student_number: u32,
    name: String,
    rrn: u32,
    collage: String,
    department: String,
    major: Option<String>,
    division: Option<String>,
    grade: u32,
    term: u32,
    #[allow(unused)]
    image: Vec<u8>,
    alias: Option<String>,
    kanji_name: Option<String>,
    email: Option<String>,
    tel_number: Option<String>,
    mobile_number: Option<String>,
    post_code: Option<String>,
    address: Option<String>,
    specific_address: Option<String>,
    is_transfer_student: bool,
    apply_date: String,
    applied_collage: String,
    applied_department: String,
    plural_major: Option<String>,
    sub_major: Option<String>,
    connected_major: Option<String>,
    abeek: Option<String>,
}

impl<'a> StudentInformation {
    define_elements! {
        // 입학 년도
        APPLY_PERYR: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.APPLY_PERYR";
        // 학번
        STUDENT12: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.STUDENT12";
        // 이름
        VORNA: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.VORNA";
        // 주민번호
        PRDNI: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.PRDNI";
        // 대학(원)
        COLEG_TXT: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.COLEG_TXT";
        // 학과(부)
        DEPT_TXT: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.DEPT_TXT";
        // 전공
        MAJOR_TXT: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.MAJOR_TXT";
        // 분반
        TITEL: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TITEL";
        // 학년
        CMSTYEAR: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.CMSTYEAR";
        // 학기
        ZSCHTERM: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.ZSCHTERM";
        // 사진
        #[allow(unused)]
        ST_IMAGE: Image<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.ST_IMAGE";
    }

    define_elements! {
        // 통칭 이름
        RUFNM: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.RUFNM";
        // 한자 이름
        BIRTHNAME: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.BIRTHNAME";
        // 이메일
        SMTP_ADDR: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.SMTP_ADDR";
        // 집전화
        TEL_NUMBER: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TEL_NUMBER";
        // 휴대전화
        MOB_NUMBER: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.MOB_NUMBER";
    }

    define_elements! {
        // 우편번호
        POST_CODE: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.POST_CODE";
        // 주소1
        CITY1: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.CITY1";
        // 주소2
        STREET: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.STREET";
    }

    define_elements! {
        // 신편입구분
        NEWINCOR_CDT: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_NEWINCOR_CDT";
        // 입학년월일
        APPLY_DT: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_APPLY_DT";
        // (입학)대학
        COLEG_CDT: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_COLEG_CDT";
        // (입학)학부
        DEPT_CDT: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_DEPT_CDT";
        #[allow(unused)]
        MODIFY_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.MODIFY_BUTTON";
        #[allow(unused)]
        SAVE_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.SAVE_BUTTON";
    }

    define_elements! {
        // 복수전공
        CG_STEXT1: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_CG_STEXT1";
        // 부전공
        CG_STEXT2: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_CG_STEXT2";
        // 연계전공
        CG_STEXT3: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_CG_STEXT3";
        // 공학인증
        CG_STEXT4: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_CG_STEXT4";
    }

    pub(super) fn with_parser(
        parser: &'a ElementParser,
    ) -> Result<StudentInformation, WebDynproError> {
        Ok(Self {
            apply_year: parser
                .element_from_def(&Self::APPLY_PERYR)?
                .value_into_u32()?,
            student_number: parser
                .element_from_def(&Self::STUDENT12)?
                .value_into_u32()?,
            name: parser.read(InputFieldValueCommand::new(Self::VORNA))?,
            rrn: parser.element_from_def(&Self::PRDNI)?.value_into_u32()?,
            collage: parser.read(InputFieldValueCommand::new(Self::COLEG_TXT))?,
            department: parser.read(InputFieldValueCommand::new(Self::DEPT_TXT))?,
            major: parser
                .read(InputFieldValueCommand::new(Self::MAJOR_TXT))
                .ok(),
            division: parser.read(InputFieldValueCommand::new(Self::TITEL)).ok(),
            grade: parser.element_from_def(&Self::CMSTYEAR)?.value_into_u32()?,
            term: parser.element_from_def(&Self::ZSCHTERM)?.value_into_u32()?,
            image: Vec::with_capacity(0), // TODO: Image to bytes
            alias: parser.read(InputFieldValueCommand::new(Self::RUFNM)).ok(),
            kanji_name: parser
                .read(InputFieldValueCommand::new(Self::BIRTHNAME))
                .ok(),
            email: parser
                .read(InputFieldValueCommand::new(Self::SMTP_ADDR))
                .ok(),
            tel_number: parser
                .read(InputFieldValueCommand::new(Self::TEL_NUMBER))
                .ok(),
            mobile_number: parser
                .read(InputFieldValueCommand::new(Self::MOB_NUMBER))
                .ok(),
            post_code: parser
                .read(InputFieldValueCommand::new(Self::POST_CODE))
                .ok(),
            address: parser.read(InputFieldValueCommand::new(Self::CITY1)).ok(),
            specific_address: parser.read(InputFieldValueCommand::new(Self::STREET)).ok(),
            is_transfer_student: !parser
                .read(InputFieldValueCommand::new(Self::NEWINCOR_CDT))?
                .contains("신입학"),
            apply_date: parser.read(InputFieldValueCommand::new(Self::APPLY_DT))?,
            applied_collage: parser.read(InputFieldValueCommand::new(Self::COLEG_CDT))?,
            applied_department: parser.read(InputFieldValueCommand::new(Self::DEPT_CDT))?,
            plural_major: parser
                .read(InputFieldValueCommand::new(Self::CG_STEXT1))
                .ok(),
            sub_major: parser
                .read(InputFieldValueCommand::new(Self::CG_STEXT2))
                .ok(),
            connected_major: parser
                .read(InputFieldValueCommand::new(Self::CG_STEXT3))
                .ok(),
            abeek: parser
                .read(InputFieldValueCommand::new(Self::CG_STEXT4))
                .ok(),
        })
    }

    /// 입학년도를 반환합니다.
    pub fn apply_year(&self) -> u32 {
        self.apply_year
    }

    /// 학번을 반환합니다.
    pub fn student_number(&self) -> u32 {
        self.student_number
    }

    /// 이름을 반환합니다.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 주민번호(앞자리)를 반환합니다.
    pub fn rrn(&self) -> u32 {
        self.rrn
    }

    /// 대학(원)을 반환합니다.
    pub fn collage(&self) -> &str {
        &self.collage
    }

    /// 학과(부)를 반환합니다.
    pub fn department(&self) -> &str {
        &self.department
    }

    /// 전공을 반환합니다.
    pub fn major(&self) -> Option<&str> {
        self.major.as_deref()
    }

    /// 분반을 반환합니다.
    pub fn division(&self) -> Option<&str> {
        self.division.as_deref()
    }

    /// 학년을 반환합니다.
    pub fn grade(&self) -> u32 {
        self.grade
    }

    /// 학기를 반환합니다.
    pub fn term(&self) -> u32 {
        self.term
    }

    // 사진의 바이너리를 반환합니다.
    // pub fn image(&self) -> &[u8] {
    //     &self.image
    // }

    /// 통칭 이름을 반환합니다.
    pub fn alias(&self) -> Option<&str> {
        self.alias.as_deref()
    }

    /// 한자 이름을 반환합니다.
    pub fn kanji_name(&self) -> Option<&str> {
        self.kanji_name.as_deref()
    }

    /// 이메일을 반환합니다.
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    /// 집전화 번호를 반환합니다.
    pub fn tel_number(&self) -> Option<&str> {
        self.tel_number.as_deref()
    }

    /// 휴대전화 번호를 반환합니다.
    pub fn mobile_number(&self) -> Option<&str> {
        self.mobile_number.as_deref()
    }

    /// 우편번호를 반환합니다.
    pub fn post_code(&self) -> Option<&str> {
        self.post_code.as_deref()
    }

    /// 주소를 반환합니다.
    pub fn address(&self) -> (Option<&str>, Option<&str>) {
        (self.address.as_deref(), self.specific_address.as_deref())
    }

    /// 편입학생 여부를 반환합니다.
    pub fn is_transfer_student(&self) -> bool {
        self.is_transfer_student
    }

    /// 입학년월일을 반환합니다.
    pub fn apply_date(&self) -> &str {
        &self.apply_date
    }

    /// 입학 당시 단과 대학을 반환합니다.
    pub fn applied_collage(&self) -> &str {
        &self.applied_collage
    }

    /// 입학 당시 학부를 반환합니다.
    pub fn applied_department(&self) -> &str {
        &self.applied_department
    }

    /// 복수전공을 반환합니다.
    pub fn plural_major(&self) -> Option<&str> {
        self.plural_major.as_deref()
    }

    /// 부전공을 반환합니다.
    pub fn sub_major(&self) -> Option<&str> {
        self.sub_major.as_deref()
    }

    /// 연계전공을 반환합니다.
    pub fn connected_major(&self) -> Option<&str> {
        self.connected_major.as_deref()
    }

    /// 공학인증을 반환합니다.
    pub fn abeek(&self) -> Option<&str> {
        self.abeek.as_deref()
    }
}
mod academic_record;
mod bank_account;
mod family;
mod graduation;
mod qualification;
mod religion;
mod research_bank_account;
mod transfer;
mod work;

use crate::webdynpro::command::WebDynproCommandExecutor;
use crate::webdynpro::element::parser::ElementParser;
pub use academic_record::{StudentAcademicRecord, StudentAcademicRecords};
pub use bank_account::StudentBankAccount;
pub use family::{StudentFamily, StudentFamilyMember};
pub use graduation::StudentGraduation;
pub use qualification::{
    StudentForignStudyInformation, StudentLifelongInformation, StudentQualification,
    StudentTeachingMajorInformation, StudentTeachingPluralMajorInformation,
};
pub use religion::StudentReligion;
pub use research_bank_account::StudentResearchBankAccount;
pub use transfer::{StudentTransferRecord, StudentTransferRecords};
pub use work::StudentWorkInformation;
