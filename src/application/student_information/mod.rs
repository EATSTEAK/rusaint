use model::{GeneralStudentInformation, StudentAcademicRecordInformation, StudentBankAccountInformation, StudentFamilyInformation, StudentReligionInformation, StudentResearchBankAccountInformation, StudentWorkInformation};

use crate::{define_elements, webdynpro::{client::body::Body, element::{action::Button, complex::SapTable, graphic::Image, layout::{tab_strip::item::TabStripItem, TabStrip}, selection::ComboBox, text::InputField}, error::WebDynproError}, RusaintError};

use super::{USaintApplication, USaintClient};

/// [학생 정보 수정 및 조회](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW1001n)
pub struct StudentInformation {
    client: USaintClient,
}

impl USaintApplication for StudentInformation {
    const APP_NAME: &'static str = "ZCMW1001n";

    fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
        if client.name() != Self::APP_NAME {
            Err(RusaintError::InvalidClientError)
        } else {
            Ok(Self { client })
        }
    }
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

    define_elements! {
        // 이름 참...
        // 복수전공
        CG_STEXT1: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_CG_STEXT1";
        // 부전공
        CG_STEXT2: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_CG_STEXT2";
        // 연계전공
        CG_STEXT3: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_CG_STEXT3";
        // 공학인증
        CG_STEXT4: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_CG_STEXT4";
    }

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

    // 교직(복수전공)
    define_elements! {
        // 표시과목
        DOUBLE_OTYPE: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_DOUBLE_OTYPE";
        // 교원자격증번호
        DOUBLE_QUAL_NUM: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_DOUBLE_QUAL_NUM";
        // 교원자격증 발급일자
        DOUBLEL_DT: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.TC_DEFAULT_DOUBLEL_DT";
    }

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

    // 7+1 프로그램
    define_elements! {
        // 신청일자
        APPRODATE: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.APPRODATE";
        // 인증서번호
        AUTHEN_NO: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.AUTHEN_NO";
        // 발급일자
        ISSUEDATE: InputField<'a> = "ZCMW1001.ID_0001:VIW_DEFAULT.ISSUEDATE";
    }

    // 부가정보 탭
    define_elements! {
        TAB_ADDITION: TabStrip<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_ADDITION";
    }

    // 직장정보
    define_elements! {
        // 직장정보 탭
        TAB_WORK: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_WORK";
        // 직업
        COJOB: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_WORK.COJOB";
        // 공무원 구분
        COMPANY_ORGR: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_WORK.COMPANY_ORGR";
        // 직장명
        COMPANY_NAM: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_WORK.COMPANY_NAM";
        // 부서명
        COMPANY_DEPT_NAM: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_WORK.COMPANY_DEPT_NAM";
        // 직위
        COMPANY_TITLE: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_WORK.COMPANY_TITLE";
        // 우편번호/시
        COMPANY_ZIP_COD: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_WORK.COMPANY_ZIP_COD";
        // 주소
        COMPANY_ADDRESS: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_WORK.COMPANY_ADDRESS";
        // 주소2
        COMPANY_ADDRESS2: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_WORK.COMPANY_ADDRESS2";
        // 전화번호
        COMPANY_TEL1: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_WORK.COMPANY_TEL1";
        // FAX번호
        COMPANY_TEL2: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_WORK.COMPANY_TEL2";
        WORK_MODIFY_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_WORK.MODIFY_BUTTON";
        WORK_SAVE_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_WORK.SAVE_BUTTON";
    }

    // 가족사항
    define_elements! {
        // 가족사항 탭
        TAB_FAMILY: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_FAMILY";
        // 가족사항 표
        TABLE_FAMILY: SapTable<'a> = "ZCMW1001.ID_0001:VIW_TAB_FAMILY_TABLE.TABLE_FAMILY";
    }

    // 종교
    define_elements! {
        // 종교 탭
        TAB_RELIGION: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_RELIGION";
        // TODO: Add fields
    }

    // 편입정보
    define_elements! {
        // 편입정보 탭
        TAB_TRANSFER: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_TRANSFER";
        // 편입정보 표
        TABLE_TRANSFER: SapTable<'a> = "ZCMW1001.ID_0001:VIW_TAB_TRANSFER.TABLE_TRANSFER";
    }

    // 은행계좌정보
    define_elements! {
        // 은행계좌정보 탭
        TAB_BANK_CP: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_BANK_CP";
        // 은행구분
        LIST_BANKS: ComboBox<'a> = "ZCMW1001.ID_0001:VIW_TAB_BANK_CP.LIST_BANKS";
        // 은행계좌번호
        BANKN: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_BANK_CP.BANKN";
        // 예금주
        ZKOINH: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_BANK_CP.ZKOINH";
        BANK_CP_MODIFY_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_BANK_CP.MODIFY_BUTTON";
        BANK_CP_SAVE_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_BANK_CP.SAVE_BUTTON";
    }

    // 학적상태
    define_elements! {
        // 학적상태 탭
        TAB_READ_9600: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_READ_9600";
        // 학적상태 표
        TABLE_9600: SapTable<'a> = "ZCMW1001.ID_0001:VIW_TAB_9600.TABLE";
    }

    // 연구비 입금 계좌
    define_elements! {
        // 연구비 입금 계좌 탭
        TAB_RES_ACCOUNT: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_RES_ACCOUNT";
        // 은행구분
        BANK_TEXT: ComboBox<'a> = "ZCMW1001.ID_0001:VIW_TAB_RES_ACCOUNT.BANK_TEXT";
        // 은행계좌번호
        BANKN_TEXT: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RES_ACCOUNT.BANKN_TEXT";
        // 예금주
        ZKOINH_TEXT: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RES_ACCOUNT.ZKOINH_TEXT";
        RES_ACCOUNT_MODIFY_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_RES_ACCOUNT.MODIFY_BUTTON";
        RES_ACCOUNT_SAVE_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_RES_ACCOUNT.SAVE_BUTTON";
    }

    pub fn general(&self) -> Result<GeneralStudentInformation, WebDynproError> {
        todo!()
    }

    pub async fn work(&mut self) -> Result<StudentWorkInformation, WebDynproError> {
        todo!()
    }

    pub async fn family(&mut self) -> Result<StudentFamilyInformation, WebDynproError> {
        todo!()
    }

    pub async fn religion(&mut self) -> Result<StudentReligionInformation, WebDynproError> {
        todo!()
    }

    pub async fn bank_account(&mut self) -> Result<StudentBankAccountInformation, WebDynproError> {
        todo!()
    }

    pub async fn academic_record(&mut self) -> Result<StudentAcademicRecordInformation, WebDynproError> {
        todo!()
    }

    pub async fn research_bank_account(&mut self) -> Result<StudentResearchBankAccountInformation, WebDynproError> {
        todo!()
    }

    fn body(&self) -> &Body {
        self.client.body()
    }
}

pub mod model;

#[cfg(test)]
mod test {
    use serial_test::serial;

    use crate::{
        application::{student_information::StudentInformation, USaintClientBuilder},
        global_test_utils::get_session,
        webdynpro::element::ElementWrapper,
    };

    #[tokio::test]
    #[serial]
    async fn examine_elements() {
        let session = get_session().await.unwrap();
        let app = USaintClientBuilder::new()
            .session(session)
            .build_into::<StudentInformation>()
            .await
            .unwrap();
        let ct_selector = scraper::Selector::parse("[ct]").unwrap();
        for elem_ref in app.body().document().select(&ct_selector) {
            let elem = ElementWrapper::dyn_element(elem_ref);
            if let Ok(elem) = elem {
                println!("{:?}", elem);
            }
        }
    }
}
