use std::sync::Arc;

use rusaint::{application::chapel::model::ChapelInformation, model::SemesterType};
use tokio::sync::RwLock;

use crate::{error::RusaintError, session::USaintSession};

#[derive(uniffi::Object)]
pub struct ChapelApplication(
    RwLock<rusaint::application::chapel::ChapelApplication>,
);


#[uniffi::export(async_runtime = "tokio")]
impl ChapelApplication {
    pub async fn information(
        &self,
        year: u32,
        semester: SemesterType,
    ) -> Result<ChapelInformation, RusaintError> {
        Ok(self.0.write().await.information(year, semester).await?)
    }
}

#[derive(uniffi::Object)]
pub struct ChapelApplicationBuilder {}


#[uniffi::export(async_runtime = "tokio")]
impl ChapelApplicationBuilder {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {  }
    }

    pub async fn build(
        &self,
        session: Arc<USaintSession>,
    ) -> Result<ChapelApplication, RusaintError> {
        let mut original_builder = rusaint::application::USaintClientBuilder::new();
        original_builder = original_builder.session(session.original());
        let original_app = original_builder.build_into::<rusaint::application::chapel::ChapelApplication>()
        .await?;
    Ok(ChapelApplication(RwLock::new(original_app)))
    }
}