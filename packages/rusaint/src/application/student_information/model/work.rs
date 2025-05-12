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
/// 학생의 직업 정보
pub struct StudentWorkInformation {
    job: Option<String>,
    public_official: Option<String>,
    company_name: Option<String>,
    department_name: Option<String>,
    title: Option<String>,
    zip_code: Option<String>,
    address: Option<String>,
    specific_address: Option<String>,
    tel_number: Option<String>,
    fax_number: Option<String>,
}

impl<'a> StudentWorkInformation {
    // 직장정보
    define_elements! {
        // 직장정보 탭
        TAB_WORK: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_WORK";
        // 직업
        COJOB: ComboBox<'a> = "ZCMW1001.ID_0001:VIW_TAB_WORK.COJOB";
        // 공무원 구분
        COMPANY_ORGR: ComboBox<'a> = "ZCMW1001.ID_0001:VIW_TAB_WORK.COMPANY_ORGR";
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
        #[allow(unused)]
        WORK_MODIFY_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_WORK.MODIFY_BUTTON";
        #[allow(unused)]
        WORK_SAVE_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_WORK.SAVE_BUTTON";
    }

    pub(crate) async fn with_client(
        client: &mut USaintClient,
    ) -> Result<StudentWorkInformation, WebDynproError> {
        let mut parser = ElementParser::new(client.body());
        let event = parser.read(TabStripTabSelectEventCommand::new(
            StudentInformationApplication::TAB_ADDITION,
            Self::TAB_WORK,
            0,
            0,
        ))?;
        client.process_event(false, event).await?;
        parser = ElementParser::new(client.body());
        Ok(Self {
            job: parser.read(ComboBoxValueCommand::new(Self::COJOB)).ok(),
            public_official: parser
                .read(ComboBoxValueCommand::new(Self::COMPANY_ORGR))
                .ok(),
            company_name: parser
                .read(InputFieldValueCommand::new(Self::COMPANY_NAM))
                .ok(),
            department_name: parser
                .read(InputFieldValueCommand::new(Self::COMPANY_DEPT_NAM))
                .ok(),
            title: parser
                .read(InputFieldValueCommand::new(Self::COMPANY_TITLE))
                .ok(),
            zip_code: parser
                .read(InputFieldValueCommand::new(Self::COMPANY_ZIP_COD))
                .ok(),
            address: parser
                .read(InputFieldValueCommand::new(Self::COMPANY_ADDRESS))
                .ok(),
            specific_address: parser
                .read(InputFieldValueCommand::new(Self::COMPANY_ADDRESS2))
                .ok(),
            tel_number: parser
                .read(InputFieldValueCommand::new(Self::COMPANY_TEL1))
                .ok(),
            fax_number: parser
                .read(InputFieldValueCommand::new(Self::COMPANY_TEL2))
                .ok(),
        })
    }
    /// 직업을 반환합니다.
    pub fn job(&self) -> Option<&str> {
        self.job.as_deref()
    }

    /// 공무원 구분을 반환합니다.
    pub fn public_official(&self) -> Option<&str> {
        self.public_official.as_deref()
    }

    /// 직장명을 반환합니다.
    pub fn company_name(&self) -> Option<&str> {
        self.company_name.as_deref()
    }

    /// 부서명을 반환합니다.
    pub fn department_name(&self) -> Option<&str> {
        self.department_name.as_deref()
    }

    /// 직위를 반환합니다.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// 우편번호를 반환합니다.
    pub fn zip_code(&self) -> Option<&str> {
        self.zip_code.as_deref()
    }

    /// 주소를 반환합니다.
    pub fn address(&self) -> (Option<&str>, Option<&str>) {
        (self.address.as_deref(), self.specific_address.as_deref())
    }

    /// 전화번호를 반환합니다.
    pub fn tel_number(&self) -> Option<&str> {
        self.tel_number.as_deref()
    }

    /// 팩스 번호를 반환합니다.
    pub fn fax_number(&self) -> Option<&str> {
        self.fax_number.as_deref()
    }
}
