use crate::application::utils::de_with::deserialize_comma_u64_string;
use crate::application::utils::de_with::deserialize_semester_type;
use std::collections::HashMap;

use serde::{
    Deserialize, Serialize,
    de::{IntoDeserializer, value::MapDeserializer},
};

use crate::application::utils::de_with::deserialize_u32_string;
use crate::webdynpro::command::WebDynproCommandExecutor;
use crate::webdynpro::element::parser::ElementParser;
use crate::{
    RusaintError, define_elements,
    error::ApplicationError,
    model::SemesterType,
    webdynpro::{
        command::element::complex::SapTableBodyCommand,
        element::{
            ElementDefWrapper,
            complex::{
                SapTable,
                sap_table::{
                    FromSapTable,
                    cell::{SapTableCell, SapTableCellWrapper},
                },
            },
            definition::ElementDefinition,
        },
        error::{ElementError, WebDynproError},
    },
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// 수혜받은 장학금 정보
pub struct Scholarship {
    #[serde(
        rename(deserialize = "학년"),
        deserialize_with = "deserialize_u32_string"
    )]
    year: u32,
    #[serde(
        rename(deserialize = "학기"),
        deserialize_with = "deserialize_semester_type"
    )]
    semester: SemesterType,
    #[serde(rename(deserialize = "장학금명"))]
    name: String,
    #[serde(
        rename(deserialize = "실수혜금액"),
        deserialize_with = "deserialize_comma_u64_string"
    )]
    received_amount: u64,
    #[serde(rename(deserialize = "지급방법"))]
    receive_type: String,
    #[serde(rename(deserialize = "처리상태"))]
    status: String,
    #[serde(rename(deserialize = "처리일자"))]
    processed_at: String,
    #[serde(
        rename(deserialize = "선발금액"),
        deserialize_with = "deserialize_comma_u64_string"
    )]
    selected_amount: u64,
    #[serde(
        rename(deserialize = "환수금액"),
        deserialize_with = "deserialize_comma_u64_string"
    )]
    refunded_amount: u64,
    #[serde(
        rename(deserialize = "교체금액"),
        deserialize_with = "deserialize_comma_u64_string"
    )]
    replaced_amount: u64,
    #[serde(rename(deserialize = "교체장학금명"))]
    replaced_by: String,
    #[serde(rename(deserialize = "탈락사유"))]
    drop_reason: String,
    #[serde(rename(deserialize = "비고"))]
    note: String,
    #[serde(rename(deserialize = "근로부서"))]
    worked_at: String,
}

impl Scholarship {
    pub(crate) fn with_parser(parser: &ElementParser) -> Result<Vec<Self>, RusaintError> {
        define_elements! {
            TABLE: SapTable<'_> = "ZCMW7530.ID_0001:VIW_MAIN.TABLE_2";
        }
        let table = parser.read(SapTableBodyCommand::new(TABLE))?;
        let Some(first_row) = table.iter().next() else {
            return Err(ApplicationError::NoChapelInformation.into());
        };
        if let Some(Ok(SapTableCellWrapper::Normal(cell))) = first_row.iter_value(parser).next() {
            if let Some(ElementDefWrapper::TextView(tv_def)) = cell.content() {
                if let Ok(tv) = parser.element_from_def(&tv_def) {
                    if tv.text().contains("없습니다.") {
                        return Err(ApplicationError::NoChapelInformation.into());
                    }
                }
            }
        }
        Ok(table.try_table_into::<Self>(parser)?)
    }

    /// 장학금 수혜 년도를 반환합니다.
    pub fn year(&self) -> u32 {
        self.year
    }

    /// 장학금 수혜 학기를 반환합니다.
    pub fn semester(&self) -> SemesterType {
        self.semester
    }

    /// 장학금명을 반환합니다.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 실수혜금액을 반환합니다.
    pub fn received_amount(&self) -> u64 {
        self.received_amount
    }

    /// 지급방법을 반환합니다.
    pub fn receive_type(&self) -> &str {
        &self.receive_type
    }

    /// 처리상태를 반환합니다.
    pub fn status(&self) -> &str {
        &self.status
    }

    /// 처리일자를 반환합니다.
    pub fn processed_at(&self) -> &str {
        &self.processed_at
    }

    /// 선발금액을 반환합니다.
    pub fn selected_amount(&self) -> u64 {
        self.selected_amount
    }

    /// 환수금액을 반환합니다.
    pub fn refunded_amount(&self) -> u64 {
        self.refunded_amount
    }

    /// 교체금액을 반환합니다.
    pub fn replaced_amount(&self) -> u64 {
        self.replaced_amount
    }

    /// 교체장학금명을 반환합니다.
    pub fn replaced_by(&self) -> &str {
        &self.replaced_by
    }

    /// 탈락사유를 반환합니다.
    pub fn drop_reason(&self) -> &str {
        &self.drop_reason
    }

    /// 비고를 반환합니다.
    pub fn note(&self) -> &str {
        &self.note
    }

    /// 근로부서를 반환합니다.
    pub fn worked_at(&self) -> &str {
        &self.worked_at
    }
}

impl<'body> FromSapTable<'body> for Scholarship {
    fn from_table(
        header: &'body crate::webdynpro::element::complex::sap_table::SapTableHeader,
        row: &'body crate::webdynpro::element::complex::sap_table::SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, WebDynproError> {
        let map_string = row.try_row_into::<HashMap<String, String>>(header, parser)?;
        let map_de: MapDeserializer<_, serde::de::value::Error> = map_string.into_deserializer();
        Ok(
            Self::deserialize(map_de).map_err(|e| ElementError::InvalidContent {
                element: row.table_def().id().to_string(),
                content: e.to_string(),
            })?,
        )
    }
}
