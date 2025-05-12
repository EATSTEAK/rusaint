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
/// 학생의 종교 정보
pub struct StudentReligion {
    religion_type: Option<String>,
    start_date: Option<String>,
    church: Option<String>,
    church_man: Option<String>,
    baptism_level: Option<String>,
    baptism_grp: Option<String>,
    service_department: Option<String>,
    service_department_title: Option<String>,
    church_address: Option<String>,
    singeub: Option<String>,
    baptism_date: Option<String>,
    baptism_church: Option<String>,
    baptism_man: Option<String>,
    church_grp: Option<String>,
}

impl<'a> StudentReligion {
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
        #[allow(unused)]
        MODIFY_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.MODIFY_BUTTON";
        #[allow(unused)]
        SAVE_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.SAVE_BUTTON";
    }

    pub(crate) async fn with_client(client: &mut USaintClient) -> Result<Self, WebDynproError> {
        let mut parser = ElementParser::new(client.body());
        let event = parser.read(TabStripTabSelectEventCommand::new(
            StudentInformationApplication::TAB_ADDITION,
            Self::TAB_RELIGION,
            2,
            0,
        ))?;
        client.process_event(false, event).await?;
        parser = ElementParser::new(client.body());
        Ok(Self {
            religion_type: parser
                .read(ComboBoxValueCommand::new(Self::RELIGION_COD))
                .ok(),
            start_date: parser
                .read(InputFieldValueCommand::new(Self::BELIEF_START_DATE))
                .ok(),
            church: parser
                .read(InputFieldValueCommand::new(Self::PRES_CHURCH))
                .ok(),
            church_man: parser
                .read(InputFieldValueCommand::new(Self::CHURCH_MAN))
                .ok(),
            baptism_level: parser
                .read(ComboBoxValueCommand::new(Self::BAPTISM_LVL))
                .ok(),
            baptism_grp: parser
                .read(InputFieldValueCommand::new(Self::BAPTISM_GRP))
                .ok(),
            service_department: parser
                .read(InputFieldValueCommand::new(Self::SERVICE_DEPT_DES))
                .ok(),
            service_department_title: parser
                .read(InputFieldValueCommand::new(Self::SERVICE_DEPT_LVL))
                .ok(),
            church_address: parser
                .read(InputFieldValueCommand::new(Self::CHURCH_ADDR))
                .ok(),
            singeub: parser
                .read(ComboBoxValueCommand::new(Self::SINGEUB_DES))
                .ok(),
            baptism_date: parser
                .read(InputFieldValueCommand::new(Self::BAPTISM_DT))
                .ok(),
            baptism_church: parser
                .read(InputFieldValueCommand::new(Self::BAPTISM_CH))
                .ok(),
            baptism_man: parser
                .read(InputFieldValueCommand::new(Self::BAPTISM_MAN))
                .ok(),
            church_grp: parser
                .read(InputFieldValueCommand::new(Self::CHURCH_GRP))
                .ok(),
        })
    }

    /// 종교 분류를 반환합니다.
    pub fn religion_type(&self) -> Option<&str> {
        self.religion_type.as_deref()
    }

    /// 신앙시작일을 반환합니다.
    pub fn start_date(&self) -> Option<&str> {
        self.start_date.as_deref()
    }

    /// 출석교회를 반환합니다.
    pub fn church(&self) -> Option<&str> {
        self.church.as_deref()
    }

    /// 담임목사를 반환합니다.
    pub fn church_man(&self) -> Option<&str> {
        self.church_man.as_deref()
    }

    /// 직분을 반환합니다.
    pub fn baptism_level(&self) -> Option<&str> {
        self.baptism_level.as_deref()
    }

    /// 교단을 반환합니다.
    pub fn baptism_grp(&self) -> Option<&str> {
        self.baptism_grp.as_deref()
    }

    /// 봉사부서를 반환합니다.
    pub fn service_department(&self) -> Option<&str> {
        self.service_department.as_deref()
    }

    /// 직책을 반환합니다.
    pub fn service_department_title(&self) -> Option<&str> {
        self.service_department_title.as_deref()
    }

    /// 교회주소를 반환합니다.
    pub fn church_address(&self) -> Option<&str> {
        self.church_address.as_deref()
    }

    /// 신급을 반환합니다.
    pub fn singeub(&self) -> Option<&str> {
        self.singeub.as_deref()
    }

    /// 세례일자를 반환합니다.
    pub fn baptism_date(&self) -> Option<&str> {
        self.baptism_date.as_deref()
    }

    /// 세례교회를 반환합니다.
    pub fn baptism_church(&self) -> Option<&str> {
        self.baptism_church.as_deref()
    }

    /// 집례목사를 반환합니다.
    pub fn baptism_man(&self) -> Option<&str> {
        self.baptism_man.as_deref()
    }

    /// 교단을 반환합니다.
    pub fn church_grp(&self) -> Option<&str> {
        self.church_grp.as_deref()
    }
}
