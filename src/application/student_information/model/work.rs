use crate::{
    application::{student_information::StudentInformation, USaintClient},
    define_elements,
    webdynpro::{
        command::element::layout::TabStripTabSelectCommand,
        element::{
            action::Button, definition::ElementDefinition, layout::tab_strip::item::TabStripItem,
            selection::ComboBox, text::InputField,
        },
        error::WebDynproError,
    },
};

#[derive(Clone, Debug)]
pub struct StudentWorkInformation {
    job: Option<String>,
    public_official: Option<String>,
    company_name: Option<String>,
    department_name: Option<String>,
    title: Option<String>,
    zip_code: Option<String>,
    address: (Option<String>, Option<String>),
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
        WORK_MODIFY_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_WORK.MODIFY_BUTTON";
        WORK_SAVE_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_WORK.SAVE_BUTTON";
    }

    pub(crate) async fn with_client(
        client: &mut USaintClient,
    ) -> Result<StudentWorkInformation, WebDynproError> {
        client
            .send(TabStripTabSelectCommand::new(
                StudentInformation::TAB_ADDITION,
                Self::TAB_WORK,
                0,
                0,
            ))
            .await?;
        Ok(Self {
            job: Self::COJOB
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            public_official: Self::COMPANY_ORGR
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            company_name: Self::COMPANY_NAM
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            department_name: Self::COMPANY_DEPT_NAM
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            title: Self::COMPANY_TITLE
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            zip_code: Self::COMPANY_ZIP_COD
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            address: (
                Self::COMPANY_ADDRESS
                    .from_body(client.body())?
                    .value()
                    .map(str::to_string),
                Self::COMPANY_ADDRESS2
                    .from_body(client.body())?
                    .value()
                    .map(str::to_string),
            ),
            tel_number: Self::COMPANY_TEL1
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            fax_number: Self::COMPANY_TEL2
                .from_body(client.body())?
                .value()
                .map(str::to_string),
        })
    }
}
