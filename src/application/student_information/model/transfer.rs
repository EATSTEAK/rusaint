use std::collections::HashMap;

use serde::{de::{value::MapDeserializer, IntoDeserializer}, Deserialize};

use crate::{
    application::{student_information::StudentInformation, USaintClient}, define_elements, webdynpro::{command::element::layout::TabStripTabSelectCommand, element::{complex::{sap_table::FromSapTable, SapTable}, definition::ElementDefinition, layout::tab_strip::item::TabStripItem}, error::{ElementError, WebDynproError}}
};

pub struct StudentTransferInformation {
    records: Vec<StudentTransferRecord>,
}

impl<'a> StudentTransferInformation {
    // 편입정보
    define_elements! {
        // 편입정보 탭
        TAB_TRANSFER: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_TRANSFER";
        // 편입정보 표
        TABLE_TRANSFER: SapTable<'a> = "ZCMW1001.ID_0001:VIW_TAB_TRANSFER.TABLE_TRANSFER";
    }

    pub(crate) async fn with_client(client: &mut USaintClient) -> Result<Self, WebDynproError> {
        client
            .send(TabStripTabSelectCommand::new(
                StudentInformation::TAB_ADDITION,
                Self::TAB_TRANSFER,
                3,
                0,
            ))
            .await?;
        let table_element = Self::TABLE_TRANSFER.from_body(client.body())?;
        let table = table_element.table()?;
        let records = table.try_table_into::<StudentTransferRecord>(client.body())?;
        Ok(Self { records })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct StudentTransferRecord {
    #[serde(rename(deserialize = "신편입구분"))]
    is_transfer: String,
    #[serde(rename(deserialize = "입학일자"))]
    admission_date: String,
    #[serde(rename(deserialize = "편입학년"))]
    admission_grade: String,
    #[serde(rename(deserialize = "편입학기"))]
    admission_term: String,
    #[serde(rename(deserialize = "인정학점"))]
    accepted_credit: String,
    #[serde(rename(deserialize = "인정학기"))]
    accepted_terms: String,
}

impl<'a> FromSapTable<'a> for StudentTransferRecord {
    fn from_table(
        body: &'a crate::webdynpro::client::body::Body,
        header: &'a crate::webdynpro::element::complex::sap_table::SapTableHeader<'a>,
        row: &'a crate::webdynpro::element::complex::sap_table::SapTableRow<'a>,
    ) -> Result<Self, crate::webdynpro::error::WebDynproError> {
        let map_string = row.try_row_into::<HashMap<String, String>>(header, body)?;
            let map_de: MapDeserializer<_, serde::de::value::Error> = map_string.into_deserializer();
            Ok(StudentTransferRecord::deserialize(map_de).map_err(|e| {
                ElementError::InvalidContent {
                    element: row.table_def().id().to_string(),
                    content: e.to_string(),
                }
            })?)
    }
}
