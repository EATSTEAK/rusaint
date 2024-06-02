use serde::Deserialize;

use crate::{
    application::{student_information::StudentInformation, USaintClient}, define_elements, webdynpro::{
        command::element::layout::TabStripTabSelectCommand, element::{
            complex::{sap_table::FromSapTable, SapTable},
            definition::ElementDefinition,
            layout::tab_strip::item::TabStripItem,
        }, error::WebDynproError
    }
};

pub struct StudentFamilyInformation {
    members: Vec<StudentFamilyMember>,
}

impl<'a> StudentFamilyInformation {
    // 가족사항
    define_elements! {
        // 가족사항 탭
        TAB_FAMILY: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_FAMILY";
        // 가족사항 표
        TABLE_FAMILY: SapTable<'a> = "ZCMW1001.ID_0001:VIW_TAB_FAMILY_TABLE.TABLE_FAMILY";
    }

    pub(crate) async fn with_client(client: &mut USaintClient) -> Result<Self, WebDynproError> {
        client
            .send(TabStripTabSelectCommand::new(
                StudentInformation::TAB_ADDITION,
                Self::TAB_FAMILY,
                1,
                0,
            ))
            .await?;
        let table_element = Self::TABLE_FAMILY.from_body(client.body())?;
        let table = table_element.table()?;
        let members = table.try_table_into::<StudentFamilyMember>(client.body())?;
        Ok(Self { members })
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct StudentFamilyMember {
    #[serde(rename(deserialize = "가족 관계 유형"))]
    relation_type: String,
    #[serde(rename(deserialize = "전화번호"))]
    tel_number: String,
    #[serde(rename(deserialize = "성명"))]
    name: String,
    #[serde(rename(deserialize = "휴대전화"))]
    mobile_number: String,
    #[serde(rename(deserialize = "근무처(직장명)"))]
    office: String,
    #[serde(rename(deserialize = "직업"))]
    job: String,
    #[serde(rename(deserialize = "직위"))]
    position: String,
    #[serde(rename(deserialize = "보호자여부"))]
    is_guardian: bool,
    #[serde(rename(deserialize = "동거여부"))]
    is_cohabit: bool,
}

impl<'a> FromSapTable<'a> for StudentFamilyMember {
    fn from_table(
        body: &'a crate::webdynpro::client::body::Body,
        header: &'a crate::webdynpro::element::complex::sap_table::SapTableHeader<'a>,
        row: &'a crate::webdynpro::element::complex::sap_table::SapTableRow<'a>,
    ) -> Result<Self, WebDynproError> {
        todo!() // Implement Checkbox first
    }
}
