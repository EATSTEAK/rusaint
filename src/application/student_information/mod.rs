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

    /// 일반 학생 정보를 반환합니다.
    pub fn general(&self) -> Result<GeneralStudentInformation, WebDynproError> {
        GeneralStudentInformation::from_body(self.body())
    }

    /// 학생의 졸업과 관련된 정보를 반환합니다.
    pub fn graduation(&self) -> Result<StudentGraduationInformation, WebDynproError> {
        StudentGraduationInformation::from_body(self.body())
    }

    /// 학생의 교직, 평생교육사, 7+1 프로그램 등 자격 관련 정보를 반환합니다.
    pub fn qualifications(&self) -> StudentQualificationInformation {
        StudentQualificationInformation::from_body(self.body())
    }

    /// 학생의 직장 정보를 반환합니다.
    pub async fn work(&mut self) -> Result<StudentWorkInformation, WebDynproError> {
        StudentWorkInformation::with_client(&mut self.client).await
    }

    /// 학생의 가족관계 정보를 반환합니다.
    pub async fn family(&mut self) -> Result<StudentFamilyInformation, WebDynproError> {
        StudentFamilyInformation::with_client(&mut self.client).await
    }

    /// 학생의 종교 정보를 반환합니다.
    pub async fn religion(&mut self) -> Result<StudentReligionInformation, WebDynproError> {
        StudentReligionInformation::with_client(&mut self.client).await
    }

    /// 학생의 편입정보를 반환합니다.
    pub async fn transfer(&mut self) -> Result<StudentTransferInformation, WebDynproError> {
        StudentTransferInformation::with_client(&mut self.client).await
    }

    /// 학생의 은행계좌 정보를 반환합니다.
    pub async fn bank_account(&mut self) -> Result<StudentBankAccountInformation, WebDynproError> {
        StudentBankAccountInformation::with_client(&mut self.client).await
    }

    /// 학생의 학적상태 정보를 반환합니다.
    pub async fn academic_record(
        &mut self,
    ) -> Result<StudentAcademicRecordInformation, WebDynproError> {
        StudentAcademicRecordInformation::with_client(&mut self.client).await
    }

    /// 학생의 연구비 입금 계좌를 반환합니다.
    pub async fn research_bank_account(
        &mut self,
    ) -> Result<StudentResearchBankAccountInformation, WebDynproError> {
        StudentResearchBankAccountInformation::with_client(&mut self.client).await
    }

    fn body(&self) -> &Body {
        self.client.body()
    }
}

/// [`StudentInformation`] 애플리케이션에서 사용하는 데이터
pub mod model;

#[cfg(test)]
mod test {
}
