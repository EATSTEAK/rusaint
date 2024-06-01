use crate::{define_elements, webdynpro::element::text::InputField};

pub struct StudentGraduationInformation {
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

impl<'a> StudentGraduationInformation {
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
}