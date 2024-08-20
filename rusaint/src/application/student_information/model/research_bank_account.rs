use crate::{
    application::{student_information::StudentInformationApplication, USaintClient},
    define_elements,
    webdynpro::{
        command::element::{
            layout::TabStripTabSelectCommand, selection::ReadComboBoxValueCommand,
            text::ReadInputFieldValueCommand,
        },
        element::{
            action::Button, layout::tab_strip::item::TabStripItem, selection::ComboBox,
            text::InputField,
        },
        error::WebDynproError,
    },
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// 연구비 입금 계좌 정보
pub struct StudentResearchBankAccount {
    bank: Option<String>,
    account_number: Option<String>,
    holder: Option<String>,
}

impl<'a> StudentResearchBankAccount {
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
        #[allow(unused)]
        MODIFY_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_RES_ACCOUNT.MODIFY_BUTTON";
        #[allow(unused)]
        SAVE_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_RES_ACCOUNT.SAVE_BUTTON";
    }

    pub(crate) async fn with_client(client: &mut USaintClient) -> Result<Self, WebDynproError> {
        client
            .send(TabStripTabSelectCommand::new(
                StudentInformationApplication::TAB_ADDITION,
                Self::TAB_RES_ACCOUNT,
                6,
                0,
            ))
            .await?;
        Ok(Self {
            bank: client
                .read(ReadComboBoxValueCommand::new(Self::BANK_TEXT))
                .ok(),
            account_number: client
                .read(ReadInputFieldValueCommand::new(Self::BANKN_TEXT))
                .ok(),
            holder: client
                .read(ReadInputFieldValueCommand::new(Self::ZKOINH_TEXT))
                .ok(),
        })
    }

    /// 학생 연구비 입금 계좌의 은행을 반환합니다.
    pub fn bank(&self) -> Option<&str> {
        self.bank.as_ref().map(String::as_str)
    }

    /// 학생 연구비 입금 계좌번호를 반환합니다.
    pub fn account_number(&self) -> Option<&str> {
        self.account_number.as_ref().map(String::as_str)
    }

    /// 학생 연구비 입금 계좌의 예금주를 반환합니다.
    pub fn holder(&self) -> Option<&str> {
        self.holder.as_ref().map(String::as_str)
    }
}
