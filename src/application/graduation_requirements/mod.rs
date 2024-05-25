use crate::{
    define_elements,
    webdynpro::{client::body::Body, error::WebDynproError},
    RusaintError,
};

use self::model::{GraduationRequirementsInfo, GradutionStudentInfo};

use super::{USaintApplication, USaintClient};

/// [졸업사정표](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW8015)
pub struct GraduationRequirements {
    client: USaintClient,
}

impl USaintApplication for GraduationRequirements {
    const APP_NAME: &'static str = "ZCMW8015";

    fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
        if client.name() != Self::APP_NAME {
            Err(RusaintError::InvalidClientError)
        } else {
            Ok(Self { client })
        }
    }
}

impl<'a> GraduationRequirements {
    // 학생정보
    define_elements! {
        STUDENT_NUM: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_STUDENT12";
        STUDENT_NAME: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_STNAME";
        STUDENT_GRADE: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_GRADE";
        // 이수학기
        PRCL: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_PRCL";
        // 학적상태
        STATUS: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_STATUST";
        // 입학년도
        APPLY_YEAR: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_APPLY_PERYR";
        // 입학유형
        NEWINCOR_CDT: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_NEWINCOR_CDT";
        // 소속
        CG_IDT_DEPT: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_CG_IDT_DEPT";
        // 제 1~4전공
        CG_IDT1: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_CG_IDT1";
        CG_IDT2: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_CG_IDT2";
        CG_IDT3: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_CG_IDT3";
        CG_IDT4: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_CG_IDT4";
        // 졸업사정일자
        AUDIT_DATE: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_AUDIT_DAT";
        // 졸업학점
        GR_CPOP: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_GR_CPOP";
        // 인정학점
        COMP_CPOP: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_COMP_CPOP";
    }

    // 졸업사정결과
    define_elements! {
        AUDIT_RESULT: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_AUDIT_RESULT_T";
        SHOW_DETAILS: Button<'a> = "ZCMW8015.ID_0001:MAIN.BTN_DTL";
        MAIN_TABLE: SapTable<'a> = "ZCMW8015.ID_0001:MAIN.TABLE";
    }

    pub async fn student_info(&mut self) -> Result<GradutionStudentInfo, WebDynproError> {
        todo!()
    }

    pub async fn requirments(&mut self) -> Result<GraduationRequirementsInfo, WebDynproError> {
        todo!()
    }
    fn body(&self) -> &Body {
        self.client.body()
    }
}

pub mod model;

#[cfg(test)]
mod test {
    use serial_test::serial;

    use crate::{
        application::{graduation_requirements::GraduationRequirements, USaintClientBuilder},
        global_test_utils::get_session,
        webdynpro::element::ElementWrapper,
    };

    #[tokio::test]
    #[serial]
    async fn examine_elements() {
        let session = get_session().await.unwrap();
        let app = USaintClientBuilder::new()
            .session(session)
            .build_into::<GraduationRequirements>()
            .await
            .unwrap();
        let ct_selector = scraper::Selector::parse("[ct]").unwrap();
        for elem_ref in app.body().document().select(&ct_selector) {
            let elem = ElementWrapper::dyn_element(elem_ref);
            if let Ok(elem) = elem {
                println!("{:?}", elem);
            }
        }
    }
}
