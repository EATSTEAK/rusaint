use model::ChapelInformation;

use crate::{
    define_elements, model::SemesterType, webdynpro::{
        client::body::Body, element::{action::Button, selection::ComboBox}, error::WebDynproError
    }, RusaintError
};

use super::{USaintApplication, USaintClient};

/// [채플정보조회](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW3681)
pub struct Chapel {
    client: USaintClient,
}

impl USaintApplication for Chapel {
    const APP_NAME: &'static str = "ZCMW3681";

    fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
        if client.name() != Self::APP_NAME {
            Err(RusaintError::InvalidClientError)
        } else {
            Ok(Self { client })
        }
    }
}


impl<'a> Chapel {

    define_elements! {
        SEL_PERYR: ComboBox<'a> = "ZCMW3681.ID_0001:V_MAIN.TC_SEL_PERYR";
        SEL_PERID: ComboBox<'a> = "ZCMW3681.ID_0001:V_MAIN.TC_SEL_PERID";
        BTN_SEL: Button<'a> = "ZCMW3681.ID_0001:V_MAIN.BTN_SEL";
    }

    fn body(&self) -> &Body {
        self.client.body()
    }

    pub async fn information(&mut self, year: u32, semester: SemesterType) -> Result<ChapelInformation, WebDynproError> {
        todo!()
    }

}

/// [`Chapel`] 애플리케이션에 사용되는 데이터
pub mod model;