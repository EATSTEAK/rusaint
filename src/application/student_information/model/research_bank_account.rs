use crate::{
    define_elements,
    webdynpro::element::{
        action::Button, layout::tab_strip::item::TabStripItem, selection::ComboBox,
        text::InputField,
    },
};

pub struct StudentResearchBankAccountInformation {
    bank: Option<String>,
    account_number: Option<String>,
    holder: Option<String>,
}

impl<'a> StudentResearchBankAccountInformation {
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
}
