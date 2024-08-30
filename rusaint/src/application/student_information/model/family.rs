use std::collections::HashMap;

use serde::{
    de::{value::MapDeserializer, IntoDeserializer},
    Deserialize,
};

use crate::webdynpro::command::WebDynproCommandExecutor;
use crate::webdynpro::element::parser::ElementParser;
use crate::{
    application::{student_information::StudentInformationApplication, USaintClient},
    define_elements,
    utils::de_with::{deserialize_bool_string, deserialize_optional_string},
    webdynpro::{
        command::element::{complex::ReadSapTableBodyCommand, layout::TabStripTabSelectCommand},
        element::{
            complex::{sap_table::FromSapTable, SapTable},
            definition::ElementDefinition,
            layout::tab_strip::item::TabStripItem,
        },
        error::{ElementError, WebDynproError},
    },
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// 학생의 가족관계 정보
pub struct StudentFamily {
    members: Vec<StudentFamilyMember>,
}

impl<'a> StudentFamily {
    // 가족사항
    define_elements! {
        // 가족사항 탭
        TAB_FAMILY: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_FAMILY";
        // 가족사항 표
        TABLE_FAMILY: SapTable<'a> = "ZCMW1001.ID_0001:VIW_TAB_FAMILY_TABLE.TABLE_FAMILY";
    }

    pub(crate) async fn with_client(client: &mut USaintClient) -> Result<Self, WebDynproError> {
        let mut parser = ElementParser::new(client.body());
        let event = parser.read(TabStripTabSelectCommand::new(
            StudentInformationApplication::TAB_ADDITION,
            Self::TAB_FAMILY,
            1,
            0,
        ))?;
        client.process_event(false, event).await?;
        parser = ElementParser::new(client.body());
        let table = parser.read(ReadSapTableBodyCommand::new(Self::TABLE_FAMILY))?;
        let members = table.try_table_into::<StudentFamilyMember>(&parser)?;
        Ok(Self { members })
    }

    /// 학생의 가족 구성원 목록을 반환합니다.
    pub fn members(&self) -> &[StudentFamilyMember] {
        &self.members
    }
}

#[derive(Clone, Deserialize, Debug)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// 학생의 가족 구성원
pub struct StudentFamilyMember {
    #[serde(
        rename(deserialize = "가족 관계 유형"),
        deserialize_with = "deserialize_optional_string"
    )]
    relation_type: Option<String>,
    #[serde(
        rename(deserialize = "전화번호"),
        deserialize_with = "deserialize_optional_string"
    )]
    tel_number: Option<String>,
    #[serde(
        rename(deserialize = "성명"),
        deserialize_with = "deserialize_optional_string"
    )]
    name: Option<String>,
    #[serde(
        rename(deserialize = "휴대전화"),
        deserialize_with = "deserialize_optional_string"
    )]
    mobile_number: Option<String>,
    #[serde(
        rename(deserialize = "근무처(직장명)"),
        deserialize_with = "deserialize_optional_string"
    )]
    office: Option<String>,
    #[serde(
        rename(deserialize = "직업"),
        deserialize_with = "deserialize_optional_string"
    )]
    job: Option<String>,
    #[serde(
        rename(deserialize = "직위"),
        deserialize_with = "deserialize_optional_string"
    )]
    position: Option<String>,
    #[serde(
        rename(deserialize = "보호자여부"),
        deserialize_with = "deserialize_bool_string"
    )]
    is_guardian: bool,
    #[serde(
        rename(deserialize = "동거여부"),
        deserialize_with = "deserialize_bool_string"
    )]
    is_cohabit: bool,
}

impl StudentFamilyMember {
    /// 가족관계 유형을 반환합니다.
    pub fn relation_type(&self) -> Option<&str> {
        self.relation_type.as_ref().map(String::as_str)
    }
    /// 전화번호를 반환합니다.
    pub fn tel_number(&self) -> Option<&str> {
        self.tel_number.as_ref().map(String::as_str)
    }

    /// 성명을 반환합니다.
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_str)
    }

    /// 휴대전화 번호를 반환합니다.
    pub fn mobile_number(&self) -> Option<&str> {
        self.mobile_number.as_ref().map(String::as_str)
    }

    /// 근무처(직장명)을 반환합니다.
    pub fn office(&self) -> Option<&str> {
        self.office.as_ref().map(String::as_str)
    }

    /// 직업을 반환합니다.
    pub fn job(&self) -> Option<&str> {
        self.job.as_ref().map(String::as_str)
    }

    /// 직위를 반환합니다.
    pub fn position(&self) -> Option<&str> {
        self.position.as_ref().map(String::as_str)
    }

    /// 보호자 여부를 반환합니다.
    pub fn is_guardian(&self) -> bool {
        self.is_guardian
    }

    /// 동거인 여부를 반환합니다.
    pub fn is_cohabit(&self) -> bool {
        self.is_cohabit
    }
}

impl<'a> FromSapTable<'a> for StudentFamilyMember {
    fn from_table(
        header: &'a crate::webdynpro::element::complex::sap_table::SapTableHeader,
        row: &'a crate::webdynpro::element::complex::sap_table::SapTableRow,
        parser: &'a ElementParser,
    ) -> Result<Self, WebDynproError> {
        let map_string = row.try_row_into::<HashMap<String, String>>(header, parser)?;
        let map_de: MapDeserializer<_, serde::de::value::Error> = map_string.into_deserializer();
        Ok(
            StudentFamilyMember::deserialize(map_de).map_err(|e| ElementError::InvalidContent {
                element: row.table_def().id().to_string(),
                content: e.to_string(),
            })?,
        )
    }
}
