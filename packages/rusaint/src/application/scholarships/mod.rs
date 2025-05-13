use super::{USaintApplication, USaintClient};
use crate::application::scholarships::model::Scholarship;
use crate::webdynpro::element::parser::ElementParser;
use crate::{RusaintError, webdynpro::client::body::Body};

/// [장학금수혜내역조회](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW7530n)
pub struct ScholarshipsApplication {
    client: USaintClient,
}

impl USaintApplication for ScholarshipsApplication {
    const APP_NAME: &'static str = "ZCMW7530n";

    fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
        if client.name() != Self::APP_NAME {
            Err(RusaintError::InvalidClientError)
        } else {
            Ok(Self { client })
        }
    }
}

impl ScholarshipsApplication {
    fn body(&self) -> &Body {
        self.client.body()
    }

    /// 장학금 수혜 내역을 가져옵니다.
    pub async fn scholarships(&mut self) -> Result<Vec<Scholarship>, RusaintError> {
        let parser = ElementParser::new(self.body());
        Scholarship::with_parser(&parser)
    }
}

/// [`ScholarshipsApplication`] 애플리케이션에 사용되는 데이터
pub mod model;
