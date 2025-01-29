use std::sync::Arc;

use rusaint::{application::chapel::model::ChapelInformation, model::SemesterType};
use tokio::sync::RwLock;

use crate::application::model::YearSemester;
use crate::{error::RusaintError, session::USaintSession};

/// [채플정보조회](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW3681)
#[derive(uniffi::Object)]
pub struct ChapelApplication(RwLock<rusaint::application::chapel::ChapelApplication>);

#[uniffi::export(async_runtime = "tokio")]
impl ChapelApplication {
    /// 해당 학기의 채플 정보를 가져옵니다.
    pub async fn information(
        &self,
        year: u32,
        semester: SemesterType,
    ) -> Result<ChapelInformation, RusaintError> {
        Ok(self.0.write().await.information(year, semester).await?)
    }

    /// 현재 페이지에 선택된 년도와 학기를 가져옵니다. 최초 로드 시 현재 학기를 가져올 가능성이 있습니다.
    /// 하지만 이 애플리케이션의 다른 함수를 호출하여 한번 정보를 가져왔다면 마지막으로 가져온 정보의 학기가 반환되므로 주의하여야 하며, 신뢰할 수 있는 현재 학기의 원천으로 사용되어서는 안됩니다.
    pub async fn get_selected_semester(&self) -> Result<YearSemester, RusaintError> {
        let (year, semester) = self.0.read().await.get_selected_semester()?;
        Ok(YearSemester::new(year, semester))
    }
}

/// [`ChapelApplication`] 생성을 위한 빌더
#[derive(uniffi::Object)]
pub struct ChapelApplicationBuilder {}

#[uniffi::export(async_runtime = "tokio")]
impl ChapelApplicationBuilder {
    /// 새로운 [`ChapelApplicationBuilder`]를 만듭니다.
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {}
    }

    /// 세션과 함께 [`ChapelApplication`]을 만듭니다.
    pub async fn build(
        &self,
        session: Arc<USaintSession>,
    ) -> Result<ChapelApplication, RusaintError> {
        let original_builder =
            rusaint::application::USaintClientBuilder::new().session(session.original());
        let original_app = original_builder
            .build_into::<rusaint::application::chapel::ChapelApplication>()
            .await?;
        Ok(ChapelApplication(RwLock::new(original_app)))
    }
}

impl Default for ChapelApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
