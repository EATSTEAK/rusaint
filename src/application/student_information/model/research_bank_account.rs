use crate::{
    application::{student_information::StudentInformation, USaintClient}, define_elements, webdynpro::{command::element::layout::TabStripTabSelectCommand, element::{
        action::Button, definition::ElementDefinition, layout::tab_strip::item::TabStripItem, selection::ComboBox, text::InputField
    }, error::WebDynproError}
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
        MODIFY_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_RES_ACCOUNT.MODIFY_BUTTON";
        SAVE_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_RES_ACCOUNT.SAVE_BUTTON";
    }

    pub(crate) async fn with_client(client: &mut USaintClient) -> Result<Self, WebDynproError> {
        client
            .send(TabStripTabSelectCommand::new(
                StudentInformation::TAB_ADDITION,
                Self::TAB_RES_ACCOUNT,
                1,
                0,
            ))
            .await?;
        Ok(
            Self {
                bank: Self::BANK_TEXT.from_body(client.body())?.value().map(str::to_string),
                account_number: Self::BANKN_TEXT.from_body(client.body())?.value().map(str::to_string),
                holder: Self::ZKOINH_TEXT.from_body(client.body())?.value().map(str::to_string),
            }
        )
    }
}
