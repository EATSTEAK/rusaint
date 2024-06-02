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
        GeneralStudentInformation::from_body(self.body())
    }

    pub fn graduation(&self) -> Result<StudentGraduationInformation, WebDynproError> {
        StudentGraduationInformation::from_body(self.body())
    }

    pub fn qualifications(&self) -> StudentQualificationInformation {
        StudentQualificationInformation::from_body(self.body())
    }

    pub async fn work(&mut self) -> Result<StudentWorkInformation, WebDynproError> {
        StudentWorkInformation::with_client(&mut self.client).await
    }

    pub async fn family(&mut self) -> Result<StudentFamilyInformation, WebDynproError> {
        StudentFamilyInformation::with_client(&mut self.client).await
    }

    pub async fn religion(&mut self) -> Result<StudentReligionInformation, WebDynproError> {
        StudentReligionInformation::with_client(&mut self.client).await
    }

    pub async fn transfer(&mut self) -> Result<StudentTransferInformation, WebDynproError> {
        StudentTransferInformation::with_client(&mut self.client).await
    }

    pub async fn bank_account(&mut self) -> Result<StudentBankAccountInformation, WebDynproError> {
        StudentBankAccountInformation::with_client(&mut self.client).await
    }

    pub async fn academic_record(
        &mut self,
    ) -> Result<StudentAcademicRecordInformation, WebDynproError> {
        StudentAcademicRecordInformation::with_client(&mut self.client).await
    }

    pub async fn research_bank_account(
        &mut self,
    ) -> Result<StudentResearchBankAccountInformation, WebDynproError> {
        StudentResearchBankAccountInformation::with_client(&mut self.client).await
    }

    fn body(&self) -> &Body {
        self.client.body()
    }
}

pub mod model;

#[cfg(test)]
mod test {
}
