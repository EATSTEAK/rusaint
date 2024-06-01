use crate::{define_elements, webdynpro::element::layout::tab_strip::item::TabStripItem};

pub struct StudentReligionInformation {}

impl<'a> StudentReligionInformation {
    // 종교
    define_elements! {
        // 종교 탭
        TAB_RELIGION: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_RELIGION";
        // TODO: Add fields
    }
}
