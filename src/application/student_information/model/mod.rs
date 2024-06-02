use crate::{
    define_elements,
    webdynpro::{
        client::body::Body,
        element::{
            action::Button, definition::ElementDefinition, graphic::Image, text::InputField,
        },
        error::WebDynproError,
    },
};

#[derive(Clone, Debug)]
pub struct GeneralStudentInformation {
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
    image: Vec<u8>,
    alias: Option<String>,
    kanji_name: Option<String>,
    email: Option<String>,
    tel_number: Option<String>,
    mobile_number: Option<String>,
    post_code: Option<String>,
    address: (Option<String>, Option<String>),
    is_transfer_student: bool,
    apply_date: String,
    applied_collage: String,
    applied_department: String,
}

impl<'a> GeneralStudentInformation {
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
        MODIFY_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.MODIFY_BUTTON";
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

    pub(super) fn from_body(body: &'a Body) -> Result<GeneralStudentInformation, WebDynproError> {
        Ok(Self {
            apply_year: Self::APPLY_PERYR.from_body(body)?.value_into_u32()?,
            student_number: Self::STUDENT12.from_body(body)?.value_into_u32()?,
            name: Self::VORNA.from_body(body)?.value_string()?,
            rrn: Self::PRDNI.from_body(body)?.value_into_u32()?,
            collage: Self::COLEG_TXT.from_body(body)?.value_string()?,
            department: Self::DEPT_TXT.from_body(body)?.value_string()?,
            major: Self::MAJOR_TXT.from_body(body)?.value().map(str::to_string),
            division: Self::TITEL.from_body(body)?.value().map(str::to_string),
            grade: Self::CMSTYEAR.from_body(body)?.value_into_u32()?,
            term: Self::ZSCHTERM.from_body(body)?.value_into_u32()?,
            image: Vec::with_capacity(0), // TODO: Image to bytes
            alias: Self::RUFNM.from_body(body)?.value().map(str::to_string),
            kanji_name: Self::BIRTHNAME.from_body(body)?.value().map(str::to_string),
            email: Self::SMTP_ADDR.from_body(body)?.value().map(str::to_string),
            tel_number: Self::TEL_NUMBER.from_body(body)?.value().map(str::to_string),
            mobile_number: Self::MOB_NUMBER.from_body(body)?.value().map(str::to_string),
            post_code: Self::POST_CODE.from_body(body)?.value().map(str::to_string),
            address: (Self::CITY1.from_body(body)?.value().map(str::to_string), Self::STREET.from_body(body)?.value().map(str::to_string)),
            is_transfer_student: !Self::NEWINCOR_CDT.from_body(body)?.value_string()?.contains("신입학"),
            apply_date: Self::APPLY_DT.from_body(body)?.value_string()?,
            applied_collage: Self::COLEG_CDT.from_body(body)?.value_string()?,
            applied_department: Self::DEPT_CDT.from_body(body)?.value_string()?,
        })
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

pub use academic_record::{StudentAcademicRecord, StudentAcademicRecordInformation};
pub use bank_account::StudentBankAccountInformation;
pub use family::{StudentFamilyInformation, StudentFamilyMember};
pub use graduation::StudentGraduationInformation;
pub use qualification::{
    StudentForignStudyInformation, StudentLifelongInformation, StudentQualificationInformation,
    StudentTeachingMajorInformation, StudentTeachingPluralMajorInformation,
};
pub use religion::StudentReligionInformation;
pub use research_bank_account::StudentResearchBankAccountInformation;
pub use transfer::{StudentTransferInformation, StudentTransferRecord};
pub use work::StudentWorkInformation;
