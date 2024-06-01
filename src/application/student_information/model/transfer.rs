use crate::{define_elements, webdynpro::element::{complex::SapTable, layout::tab_strip::item::TabStripItem}};

pub struct StudentTrasferInformation {}

impl<'a> StudentTrasferInformation {
  // 편입정보
  define_elements! {
    // 편입정보 탭
    TAB_TRANSFER: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_TRANSFER";
    // 편입정보 표
    TABLE_TRANSFER: SapTable<'a> = "ZCMW1001.ID_0001:VIW_TAB_TRANSFER.TABLE_TRANSFER";
}
}