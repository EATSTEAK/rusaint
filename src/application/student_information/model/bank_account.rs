use crate::{
    define_elements,
    webdynpro::element::{
        action::Button, layout::tab_strip::item::TabStripItem, selection::ComboBox,
        text::InputField,
    },
};

pub struct StudentBankAccountInformation {
    bank: Option<String>,
    account_number: Option<String>,
    holder: Option<String>,
}

impl<'a> StudentBankAccountInformation {
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
}
