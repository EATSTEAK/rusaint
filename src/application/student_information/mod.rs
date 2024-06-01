use model::{
    GeneralStudentInformation, StudentAcademicRecordInformation, StudentBankAccountInformation,
    StudentFamilyInformation, StudentGraduationInformation, StudentQualificationInformation,
    StudentReligionInformation, StudentResearchBankAccountInformation, StudentTransferInformation,
    StudentWorkInformation,
};

use crate::{
    define_elements,
    webdynpro::{client::body::Body, element::layout::TabStrip, error::WebDynproError},
    RusaintError,
};

use super::{USaintApplication, USaintClient};

/// [학생 정보 수정 및 조회](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW1001n)
pub struct StudentInformation {
    client: USaintClient,
}

impl USaintApplication for StudentInformation {
    const APP_NAME: &'static str = "ZCMW1001n";

    fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
        if client.name() != Self::APP_NAME {
            Err(RusaintError::InvalidClientError)
        } else {
            Ok(Self { client })
        }
    }
}

impl<'a> StudentInformation {
    // 부가정보 탭
    define_elements! {
        TAB_ADDITION: TabStrip<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_ADDITION";
    }

    pub fn general(&self) -> Result<GeneralStudentInformation, WebDynproError> {
        todo!()
    }

    pub fn graduation(&self) -> Result<StudentGraduationInformation, WebDynproError> {
        todo!()
    }

    pub fn qualifications(&self) -> Result<StudentQualificationInformation, WebDynproError> {
        todo!()
    }

    pub async fn work(&mut self) -> Result<StudentWorkInformation, WebDynproError> {
        todo!()
    }

    pub async fn family(&mut self) -> Result<StudentFamilyInformation, WebDynproError> {
        todo!()
    }

    pub async fn religion(&mut self) -> Result<StudentReligionInformation, WebDynproError> {
        todo!()
    }

    pub async fn transfer(&mut self) -> Result<StudentTransferInformation, WebDynproError> {
        todo!()
    }

    pub async fn bank_account(&mut self) -> Result<StudentBankAccountInformation, WebDynproError> {
        todo!()
    }

    pub async fn academic_record(
        &mut self,
    ) -> Result<StudentAcademicRecordInformation, WebDynproError> {
        todo!()
    }

    pub async fn research_bank_account(
        &mut self,
    ) -> Result<StudentResearchBankAccountInformation, WebDynproError> {
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
        application::{student_information::StudentInformation, USaintClientBuilder},
        global_test_utils::get_session,
        webdynpro::element::ElementWrapper,
    };

    #[tokio::test]
    #[serial]
    async fn examine_elements() {
        let session = get_session().await.unwrap();
        let app = USaintClientBuilder::new()
            .session(session)
            .build_into::<StudentInformation>()
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
