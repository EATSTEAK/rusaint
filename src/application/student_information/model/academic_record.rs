use crate::{
    define_elements,
    webdynpro::element::{complex::SapTable, layout::tab_strip::item::TabStripItem},
};

pub struct StudentAcademicRecordInformation {}

impl<'a> StudentAcademicRecordInformation {
    // 학적상태
    define_elements! {
        // 학적상태 탭
        TAB_READ_9600: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_READ_9600";
        // 학적상태 표
        TABLE_9600: SapTable<'a> = "ZCMW1001.ID_0001:VIW_TAB_9600.TABLE";
    }
}
