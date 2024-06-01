use crate::{
    define_elements,
    webdynpro::element::{complex::SapTable, layout::tab_strip::item::TabStripItem},
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
}

pub struct StudentFamilyMember {
    relation_type: String,
    tel_number: String,
    name: String,
    mobile_number: String,
    office: String,
    job: String,
    position: String,
    is_guardian: bool,
    is_cohabit: bool,
}
