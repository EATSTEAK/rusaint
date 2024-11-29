use std::sync::Arc;

use crate::{error::RusaintError, session::USaintSession};
use rusaint::application::student_information::model::{
    StudentAcademicRecords, StudentBankAccount, StudentFamily, StudentGraduation,
    StudentInformation, StudentQualification, StudentReligion, StudentResearchBankAccount,
    StudentTransferRecords, StudentWorkInformation,
};
use tokio::sync::RwLock;

/// [학생 정보 수정 및 조회](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW1001n)
#[derive(uniffi::Object)]
pub struct StudentInformationApplication(
    RwLock<rusaint::application::student_information::StudentInformationApplication>,
);

#[uniffi::export(async_runtime = "tokio")]
impl StudentInformationApplication {
    /// 일반 학생 정보를 반환합니다.
    pub async fn general(&self) -> Result<StudentInformation, RusaintError> {
        Ok(self.0.read().await.general()?)
    }

    /// 학생의 졸업과 관련된 정보를 반환합니다.
    pub async fn graduation(&self) -> Result<StudentGraduation, RusaintError> {
        Ok(self.0.read().await.graduation()?)
    }

    /// 학생의 교직, 평생교육사, 7+1 프로그램 등 자격 관련 정보를 반환합니다.
    pub async fn qualifications(&self) -> Result<StudentQualification, RusaintError> {
        Ok(self.0.read().await.qualifications()?)
    }

    /// 학생의 직장 정보를 반환합니다.
    pub async fn work(&self) -> Result<StudentWorkInformation, RusaintError> {
        Ok(self.0.write().await.work().await?)
    }

    /// 학생의 가족관계 정보를 반환합니다.
    pub async fn family(&self) -> Result<StudentFamily, RusaintError> {
        Ok(self.0.write().await.family().await?)
    }

    /// 학생의 종교 정보를 반환합니다.
    pub async fn religion(&self) -> Result<StudentReligion, RusaintError> {
        Ok(self.0.write().await.religion().await?)
    }

    /// 학생의 편입정보를 반환합니다.
    pub async fn transfer(&self) -> Result<StudentTransferRecords, RusaintError> {
        Ok(self.0.write().await.transfer().await?)
    }

    /// 학생의 은행계좌 정보를 반환합니다.
    pub async fn bank_account(&self) -> Result<StudentBankAccount, RusaintError> {
        Ok(self.0.write().await.bank_account().await?)
    }

    /// 학생의 학적상태 정보를 반환합니다.
    pub async fn academic_record(&self) -> Result<StudentAcademicRecords, RusaintError> {
        Ok(self.0.write().await.academic_record().await?)
    }

    /// 학생의 연구비 입금 계좌를 반환합니다.
    pub async fn research_bank_account(&self) -> Result<StudentResearchBankAccount, RusaintError> {
        Ok(self.0.write().await.research_bank_account().await?)
    }
}

/// [`StudentInformationApplication`] 생성을 위한 빌더
#[derive(uniffi::Object)]
pub struct StudentInformationApplicationBuilder {}

#[uniffi::export(async_runtime = "tokio")]
impl StudentInformationApplicationBuilder {
    /// 새로운 [`StudentInformationApplicationBuilder`]를 만듭니다.
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {}
    }

    /// 세션과 함께 [`StudentInformationApplication`]을 만듭니다.
    pub async fn build(
        &self,
        session: Arc<USaintSession>,
    ) -> Result<StudentInformationApplication, RusaintError> {
        let original_builder =
            rusaint::application::USaintClientBuilder::new().session(session.original());
        let original_app = original_builder
            .build_into::<rusaint::application::student_information::StudentInformationApplication>(
            )
            .await?;
        Ok(StudentInformationApplication(RwLock::new(original_app)))
    }
}

impl Default for StudentInformationApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
