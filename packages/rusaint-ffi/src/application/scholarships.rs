use std::sync::Arc;

use crate::{error::RusaintError, session::USaintSession};
use rusaint::application::scholarships::model::Scholarship;
use tokio::sync::RwLock;

/// [장학금수혜내역조회](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW7530n)
#[derive(uniffi::Object)]
pub struct ScholarshipsApplication(
    RwLock<rusaint::application::scholarships::ScholarshipsApplication>,
);

#[uniffi::export(async_runtime = "tokio")]
impl ScholarshipsApplication {
    /// 장학금 수혜 내역을 가져옵니다.
    pub async fn scholarships(&self) -> Result<Vec<Scholarship>, RusaintError> {
        Ok(self.0.write().await.scholarships().await?)
    }
}

/// [`ScholarshipsApplication`] 생성을 위한 빌더
#[derive(uniffi::Object)]
pub struct ScholarshipsApplicationBuilder {}

#[uniffi::export(async_runtime = "tokio")]
impl ScholarshipsApplicationBuilder {
    /// 새로운 [`ScholarshipsApplicationBuilder`]를 만듭니다.
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {}
    }

    /// 세션과 함께 [`ScholarshipsApplication`]을 만듭니다.
    pub async fn build(
        &self,
        session: Arc<USaintSession>,
    ) -> Result<ScholarshipsApplication, RusaintError> {
        let original_builder =
            rusaint::application::USaintClientBuilder::new().session(session.original());
        let original_app = original_builder
            .build_into::<rusaint::application::scholarships::ScholarshipsApplication>()
            .await?;
        Ok(ScholarshipsApplication(RwLock::new(original_app)))
    }
}

impl Default for ScholarshipsApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
