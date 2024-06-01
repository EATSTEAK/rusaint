use crate::{define_elements, webdynpro::element::{action::Button, layout::tab_strip::item::TabStripItem, selection::ComboBox, text::InputField}};

pub struct StudentReligionInformation {}

impl<'a> StudentReligionInformation {
    // 종교
    define_elements! {
        // 종교 탭
        TAB_RELIGION: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_RELIGION";
        // 종교
        RELIGION_COD: ComboBox<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.RELIGION_COD";
        // 신앙시작일
        BELIEF_START_DATE: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.BELIEF_START_DAT";
        // 출석교회
        PRES_CHURCH: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.PRES_CHURCH";
        // 담임목사
        CHURCH_MAN: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.CHURCH_MAN";
        // 직분
        BAPTISM_LVL: ComboBox<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.BAPTISM_LVL";
        // 교단
        BAPTISM_GRP: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.BAPTISM_GRP";
        // 봉사부서
        SERVICE_DEPT_DES: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.SERVICE_DEPT_DES";
        // 직책
        SERVICE_DEPT_LVL: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.SERVICE_DEPT_LVL";
        // 교회주소
        CHURCH_ADDR: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.CHURCH_ADDR";
        // 신급
        SINGEUB_DES: ComboBox<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.SINGEUB_DES";
        // 세례일자
        BAPTISM_DT: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.BAPTISM_DT";
        // 세례교회
        BAPTISM_CH: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.BAPTISM_CH";
        // 집례목사
        BAPTISM_MAN: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.BAPTISM_MAN";
        // 교단
        CHURCH_GRP: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.CHURCH_GRP";
        MODIFY_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.MODIFY_BUTTON";
        SAVE_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.SAVE_BUTTON";
    }
}
