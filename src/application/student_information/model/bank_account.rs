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
/// 학생의 은행 계좌 정보
pub struct StudentBankAccount {
    bank: Option<String>,
    account_number: Option<String>,
    holder: Option<String>,
}

impl<'a> StudentBankAccount {
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
        #[allow(unused)]
        MODIFY_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_BANK_CP.MODIFY_BUTTON";
        #[allow(unused)]
        SAVE_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_BANK_CP.SAVE_BUTTON";
    }

    pub(crate) async fn with_client(client: &mut USaintClient) -> Result<Self, WebDynproError> {
        client
            .send(TabStripTabSelectCommand::new(
                StudentInformationApplication::TAB_ADDITION,
                Self::TAB_BANK_CP,
                4,
                0,
            ))
            .await?;
        Ok(Self {
            bank: client
                .read(ReadComboBoxValueCommand::new(Self::LIST_BANKS))
                .ok(),
            account_number: client
                .read(ReadInputFieldValueCommand::new(Self::BANKN))
                .ok(),
            holder: client
                .read(ReadInputFieldValueCommand::new(Self::ZKOINH))
                .ok(),
        })
    }

    /// 학생 계좌의 은행을 반환합니다.
    pub fn bank(&self) -> Option<&str> {
        self.bank.as_ref().map(String::as_str)
    }

    /// 학생 계좌번호를 반환합니다.
    pub fn account_number(&self) -> Option<&str> {
        self.account_number.as_ref().map(String::as_str)
    }

    /// 학생 계좌의 예금주를 반환합니다.
    pub fn holder(&self) -> Option<&str> {
        self.holder.as_ref().map(String::as_str)
    }
}
