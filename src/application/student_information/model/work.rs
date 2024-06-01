use crate::{
    define_elements,
    webdynpro::element::{action::Button, layout::tab_strip::item::TabStripItem, text::InputField},
};

pub struct StudentWorkInformation {}

impl<'a> StudentWorkInformation {
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
}
