use crate::webdynpro::command::WebDynproCommandExecutor;
use crate::webdynpro::element::parser::ElementParser;
use crate::{
    application::{USaintClient, student_information::StudentInformationApplication},
    define_elements,
    webdynpro::{
        command::element::{
            layout::TabStripTabSelectEventCommand, selection::ComboBoxValueCommand,
            text::InputFieldValueCommand,
        },
        element::{
            action::Button, layout::tab_strip::item::TabStripItem, selection::ComboBox,
            text::InputField,
        },
        error::WebDynproError,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
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
        let mut parser = ElementParser::new(client.body());
        let event = parser.read(TabStripTabSelectEventCommand::new(
            StudentInformationApplication::TAB_ADDITION,
            Self::TAB_RES_ACCOUNT,
            6,
            0,
        ))?;
        client.process_event(false, event).await?;
        parser = ElementParser::new(client.body());
        Ok(Self {
            bank: parser.read(ComboBoxValueCommand::new(Self::BANK_TEXT)).ok(),
            account_number: parser
                .read(InputFieldValueCommand::new(Self::BANKN_TEXT))
                .ok(),
            holder: parser
                .read(InputFieldValueCommand::new(Self::ZKOINH_TEXT))
                .ok(),
        })
    }

    /// 학생 연구비 입금 계좌의 은행을 반환합니다.
    pub fn bank(&self) -> Option<&str> {
        self.bank.as_deref()
    }

    /// 학생 연구비 입금 계좌번호를 반환합니다.
    pub fn account_number(&self) -> Option<&str> {
        self.account_number.as_deref()
    }

    /// 학생 연구비 입금 계좌의 예금주를 반환합니다.
    pub fn holder(&self) -> Option<&str> {
        self.holder.as_deref()
    }
}
