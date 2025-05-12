use std::collections::HashMap;

use serde::{
    Deserialize, Serialize,
    de::{IntoDeserializer, value::MapDeserializer},
};

use crate::application::utils::sap_table::try_table_into_with_scroll;
use crate::webdynpro::command::WebDynproCommandExecutor;
use crate::webdynpro::element::parser::ElementParser;
use crate::{
    application::{USaintClient, student_information::StudentInformationApplication},
    define_elements,
    webdynpro::{
        command::element::layout::TabStripTabSelectEventCommand,
        element::{
            complex::{SapTable, sap_table::FromSapTable},
            definition::ElementDefinition,
            layout::tab_strip::item::TabStripItem,
        },
        error::{ElementError, WebDynproError},
    },
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// 학생 편입 정보
pub struct StudentTransferRecords {
    records: Vec<StudentTransferRecord>,
}

impl<'a> StudentTransferRecords {
    // 편입정보
    define_elements! {
        // 편입정보 탭
        TAB_TRANSFER: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_TRANSFER";
        // 편입정보 표
        TABLE_TRANSFER: SapTable<'a> = "ZCMW1001.ID_0001:VIW_TAB_TRANSFER.TABLE_TRANSFER";
    }

    pub(crate) async fn with_client(client: &mut USaintClient) -> Result<Self, WebDynproError> {
        let mut parser = ElementParser::new(client.body());
        let event = parser.read(TabStripTabSelectEventCommand::new(
            StudentInformationApplication::TAB_ADDITION,
            Self::TAB_TRANSFER,
            3,
            0,
        ))?;
        client.process_event(false, event).await?;
        parser = ElementParser::new(client.body());
        let records = try_table_into_with_scroll::<StudentTransferRecord>(
            client,
            parser,
            Self::TABLE_TRANSFER,
        )
        .await?;
        Ok(Self { records })
    }

    /// 편입정보 기록을 반환합니다.
    pub fn records(&self) -> &[StudentTransferRecord] {
        &self.records
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// 편입정보 내 기록
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

impl StudentTransferRecord {
    /// 신편입구분을 반환합니다.
    pub fn is_transfer(&self) -> &str {
        &self.is_transfer
    }

    /// 입학일자를 반환합니다.
    pub fn admission_date(&self) -> &str {
        &self.admission_date
    }

    /// 편입학년을 반환합니다.
    pub fn admission_grade(&self) -> &str {
        &self.admission_grade
    }

    /// 편입학기를 반환합니다.
    pub fn admission_term(&self) -> &str {
        &self.admission_term
    }

    /// 인정학점을 반환합니다.
    pub fn accepted_credit(&self) -> &str {
        &self.accepted_credit
    }

    /// 인정학기를 반환합니다.
    pub fn accepted_terms(&self) -> &str {
        &self.accepted_terms
    }
}

impl<'a> FromSapTable<'a> for StudentTransferRecord {
    fn from_table(
        header: &'a crate::webdynpro::element::complex::sap_table::SapTableHeader,
        row: &'a crate::webdynpro::element::complex::sap_table::SapTableRow,
        parser: &'a ElementParser,
    ) -> Result<Self, WebDynproError> {
        let map_string = row.try_row_into::<HashMap<String, String>>(header, parser)?;
        let map_de: MapDeserializer<_, serde::de::value::Error> = map_string.into_deserializer();
        Ok(StudentTransferRecord::deserialize(map_de).map_err(|e| {
            ElementError::InvalidContent {
                element: row.table_def().id().to_string(),
                content: e.to_string(),
            }
        })?)
    }
}
