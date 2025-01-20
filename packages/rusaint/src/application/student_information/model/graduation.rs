use crate::webdynpro::element::parser::ElementParser;
use crate::{
    define_elements,
    webdynpro::{
        element::{definition::ElementDefinition, text::InputField},
        error::{ElementError, WebDynproError},
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// 학생의 졸업 정보를 반환합니다. 졸업하지 않았다면 반환되지 않습니다.
pub struct StudentGraduation {
    graduation_cardinal: u32,
    graduation_certification_number: u32,
    graduation_year: u32,
    graduation_terms: u32,
    graduation_date: String,
    academic_degree_number: u32,
    academic_degree_name: String,
    early_graduation: bool,
    graduation_rank: u32,
    graduation_personnel_number: u32,
}

impl<'a> StudentGraduation {
    define_elements! {
        // 졸업회수
        GRDU_NO: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_GRDU_NO";
        // 졸업증서번호
        CERTIFY_NO: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_CERTIFY_NO";
        // 졸업년도
        GRDU_PERYR: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_GRDU_PERYR";
        // 졸업학기
        GRDU_PERIDT: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_GRDU_PERIDT";
        // 학위수여일
        GRDU_DATE: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_GRDU_DATE";
        // 학위번호
        ACAD_SEQ: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_ACAD_SEQ";
        // 학위명
        ACAD_CDT: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_ACAD_CDT";
        // 조기졸업여부
        E_GRAD: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_E_GRAD";
        // 전체졸업석차
        TOT_ORDER: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_TOT_ORDER";
        // 전체졸업인원
        TDPT_NUMBER: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_TDPT_NUMBER";
    }

    pub(crate) fn with_parser(
        parser: &'a ElementParser,
    ) -> Result<StudentGraduation, WebDynproError> {
        let graduation_year = parser
            .element_from_def(&Self::GRDU_PERYR)?
            .value_into_u32()?;
        if graduation_year == 0 {
            Err(WebDynproError::Element(ElementError::NoSuchContent {
                element: Self::GRDU_NO.id().to_string(),
                content: "No graduation information provided. Is this student graduated?"
                    .to_string(),
            }))
        } else {
            Ok(Self {
                graduation_cardinal: parser.element_from_def(&Self::GRDU_NO)?.value_into_u32()?,
                graduation_certification_number: parser
                    .element_from_def(&Self::CERTIFY_NO)?
                    .value_into_u32()?,
                graduation_year,
                graduation_terms: parser
                    .element_from_def(&Self::GRDU_PERIDT)?
                    .value_into_u32()?,
                graduation_date: parser.element_from_def(&Self::GRDU_DATE)?.value_string()?,
                academic_degree_number: parser
                    .element_from_def(&Self::ACAD_SEQ)?
                    .value_into_u32()?,
                academic_degree_name: parser.element_from_def(&Self::ACAD_CDT)?.value_string()?,
                early_graduation: parser
                    .element_from_def(&Self::E_GRAD)?
                    .value_string()?
                    .contains("예"),
                graduation_rank: parser
                    .element_from_def(&Self::TOT_ORDER)?
                    .value_into_u32()?,
                graduation_personnel_number: parser
                    .element_from_def(&Self::TDPT_NUMBER)?
                    .value_into_u32()?,
            })
        }
    }

    /// 졸업회수를 반환합니다.
    pub fn graduation_cardinal(&self) -> u32 {
        self.graduation_cardinal
    }

    /// 졸업증서번호를 반환합니다.
    pub fn graduation_certification_number(&self) -> u32 {
        self.graduation_certification_number
    }

    /// 졸업년도를 반환합니다.
    pub fn graduation_year(&self) -> u32 {
        self.graduation_year
    }

    /// 졸업학기를 반환합니다.
    pub fn graduation_terms(&self) -> u32 {
        self.graduation_terms
    }

    /// 학위수여일을 반환합니다.
    pub fn graduation_date(&self) -> &str {
        &self.graduation_date
    }

    /// 학위번호를 반환합니다.
    pub fn academic_degree_number(&self) -> u32 {
        self.academic_degree_number
    }

    /// 학위명을 반환합니다.
    pub fn academic_degree_name(&self) -> &str {
        &self.academic_degree_name
    }

    /// 조기졸업여부를 반환합니다.
    pub fn early_graduation(&self) -> bool {
        self.early_graduation
    }

    /// 전체졸업석차를 반환합니다.
    pub fn graduation_rank(&self) -> u32 {
        self.graduation_rank
    }

    /// 전체졸업인원을 반환합니다.
    pub fn graduation_personnel_number(&self) -> u32 {
        self.graduation_personnel_number
    }
}
