use std::collections::HashMap;

use serde::{
    Deserialize, Serialize,
    de::{IntoDeserializer as _, value::MapDeserializer},
};
use wdpe::{
    define_elements,
    element::{
        complex::{SapTable, sap_table::FromSapTable},
        definition::ElementDefinition as _,
        parser::ElementParser,
    },
    error::ElementError,
};

use crate::application::utils::sap_table::is_sap_table_empty;

/// 강의 변경 이력
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct LectureChangeHistory {
    /// 시작일자
    #[serde(rename(deserialize = "시작일자"))]
    pub start_date: String,
    /// 종료일자
    #[serde(rename(deserialize = "종료일자"))]
    pub end_date: String,
    /// 과목명
    #[serde(rename(deserialize = "과목명"))]
    pub name: String,
}

impl<'body> FromSapTable<'body> for LectureChangeHistory {
    fn from_table(
        header: Option<&wdpe::element::complex::sap_table::SapTableHeader>,
        row: &'body wdpe::element::complex::sap_table::SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, wdpe::error::WebDynproError> {
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

/// 대체 과목 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct AlternativeLecture {
    /// 구분
    #[serde(rename(deserialize = "구분"))]
    pub kind: String,
    /// 과목번호
    #[serde(rename(deserialize = "과목번호"))]
    pub code: String,
    /// 과목명
    #[serde(rename(deserialize = "과목명"))]
    pub name: String,
}

impl<'body> FromSapTable<'body> for AlternativeLecture {
    fn from_table(
        header: Option<&wdpe::element::complex::sap_table::SapTableHeader>,
        row: &'body wdpe::element::complex::sap_table::SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, wdpe::error::WebDynproError> {
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

/// 선수 과목 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PrerequisiteLecture {
    /// 과목번호
    #[serde(rename(deserialize = "과목번호"))]
    pub code: String,
    /// 과목명
    #[serde(rename(deserialize = "과목명"))]
    pub name: String,
}

impl<'body> FromSapTable<'body> for PrerequisiteLecture {
    fn from_table(
        header: Option<&wdpe::element::complex::sap_table::SapTableHeader>,
        row: &'body wdpe::element::complex::sap_table::SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, wdpe::error::WebDynproError> {
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

/// 강의 상세 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct LectureDetail {
    /// 강의 변경 이력 목록
    pub changes_history: Vec<LectureChangeHistory>,
    /// 대체 과목 목록
    pub alternative_lectures: Vec<AlternativeLecture>,
    /// 과목 분류 목록
    pub categories: Vec<String>,
    /// 선수 과목 목록
    pub prerequisites: Vec<PrerequisiteLecture>,
}

impl LectureDetail {
    define_elements! {
      CHANGES_HISTORY_TABLE: SapTable<'_> = "ZCMW2100.ID_0001:VIW_POPUP_SMINFO.TABLE01";
      ALTERNATIVE_LECTURES_TABLE: SapTable<'_> = "ZCMW2100.ID_0001:VIW_POPUP_SMINFO.TABLE02";
      CATEGORIES_TABLE: SapTable<'_> = "ZCMW2100.ID_0001:VIW_POPUP_SMINFO.TABLE03";
      PREREQUISITES_TABLE: SapTable<'_> = "ZCMW2100.ID_0001:VIW_POPUP_SMINFO.TABLE04";
    }

    /// [`ElementParser`]로부터 강의 상세 정보를 파싱하여 [`LectureDetail`]을 생성합니다.
    pub fn with_parser(parser: &ElementParser) -> Result<Self, wdpe::error::WebDynproError> {
        let changes_history_table = parser.element_from_def(&Self::CHANGES_HISTORY_TABLE)?;
        let alternative_lectures_table =
            parser.element_from_def(&Self::ALTERNATIVE_LECTURES_TABLE)?;
        let categories_table = parser.element_from_def(&Self::CATEGORIES_TABLE)?;
        let prerequisites_table = parser.element_from_def(&Self::PREREQUISITES_TABLE)?;

        let changes_history_body = changes_history_table.table()?;
        let changes_history = if is_sap_table_empty(changes_history_body, parser) {
            vec![]
        } else {
            changes_history_body.try_table_into::<LectureChangeHistory>(parser)?
        };

        let alternative_lectures_body = alternative_lectures_table.table()?;
        let alternative_lectures = if is_sap_table_empty(alternative_lectures_body, parser) {
            vec![]
        } else {
            alternative_lectures_body.try_table_into::<AlternativeLecture>(parser)?
        };

        let categories_body = categories_table.table()?;
        let categories = if is_sap_table_empty(categories_body, parser) {
            vec![]
        } else {
            categories_body
                .try_table_into::<Vec<String>>(parser)?
                .iter()
                .flatten()
                .cloned()
                .collect()
        };

        let prerequisites_body = prerequisites_table.table()?;
        let prerequisites = if is_sap_table_empty(prerequisites_body, parser) {
            vec![]
        } else {
            prerequisites_body.try_table_into::<PrerequisiteLecture>(parser)?
        };

        Ok(Self {
            changes_history,
            alternative_lectures,
            categories,
            prerequisites,
        })
    }
}
