use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::element::sub::definition::SubElementDefinition;
use crate::webdynpro::element::sub::SubElement;
use crate::webdynpro::element::{Element, ElementDefWrapper};
use crate::webdynpro::error::WebDynproError;
use tl::Bytes;

/// [`SapTable`] 셀의 Wrapper
#[derive(Debug)]
pub enum SapTableCellWrapper<'a> {
    /// 일반적인 셀
    Normal(SapTableNormalCell<'a>),
    /// 헤더 셀
    Header(SapTableHeaderCell<'a>),
    /// 순차형 셀
    Hierarchical(SapTableHierarchicalCell<'a>),
    /// Matrix 레이아웃용 셀
    Matrix(SapTableMatrixCell<'a>),
    /// 선택을 위한 셀
    Selection(SapTableSelectionCell<'a>),
}

impl<'a> SapTableCellWrapper<'a> {
    /// 셀을 표현하는 HTML 엘리먼트로부터 [`SapTableCellWrapper`]를 생성합니다.
    pub fn from_tag(
        table_def: <SapTable<'a> as Element<'a>>::Def,
        tag: &tl::HTMLTag<'a>,
        parser: &'a ElementParser,
    ) -> Option<SapTableCellWrapper<'a>> {
        let subct_id = tag
            .attributes()
            .get("subct")
            .flatten()
            .and_then(Bytes::try_as_utf8_str);
        let tag_id = tag.attributes().id().and_then(Bytes::try_as_utf8_str)?;
        match subct_id {
            Some(SapTableNormalCell::SUBCONTROL_ID) => Some(
                parser
                    .subelement_from_def(&<SapTableNormalCell<'_> as SubElement>::Def::new_dynamic(
                        table_def,
                        tag_id.to_owned(),
                    ))
                    .ok()?
                    .wrap(),
            ),
            Some(SapTableHeaderCell::SUBCONTROL_ID) => Some(
                parser
                    .subelement_from_def(&<SapTableHeaderCell<'_> as SubElement>::Def::new_dynamic(
                        table_def,
                        tag_id.to_owned(),
                    ))
                    .ok()?
                    .wrap(),
            ),
            Some(SapTableHierarchicalCell::SUBCONTROL_ID) => Some(
                parser
                    .subelement_from_def(
                        &<SapTableHierarchicalCell<'_> as SubElement>::Def::new_dynamic(
                            table_def,
                            tag_id.to_owned(),
                        ),
                    )
                    .ok()?
                    .wrap(),
            ),
            Some(SapTableMatrixCell::SUBCONTROL_ID) => Some(
                parser
                    .subelement_from_def(&<SapTableMatrixCell<'_> as SubElement>::Def::new_dynamic(
                        table_def,
                        tag_id.to_owned(),
                    ))
                    .ok()?
                    .wrap(),
            ),
            Some(SapTableSelectionCell::SUBCONTROL_ID) => Some(
                parser
                    .subelement_from_def(
                        &<SapTableSelectionCell<'_> as SubElement>::Def::new_dynamic(
                            table_def,
                            tag_id.to_owned(),
                        ),
                    )
                    .ok()?
                    .wrap(),
            ),
            _ => None,
        }
    }

    pub fn from_def(
        wrapper: &SapTableCellDefWrapper,
        parser: &'a ElementParser,
    ) -> Result<SapTableCellWrapper<'a>, WebDynproError> {
        Ok(match wrapper {
            SapTableCellDefWrapper::Normal(def) => {
                SapTableCellWrapper::Normal(parser.subelement_from_def(def)?)
            }
            SapTableCellDefWrapper::Header(def) => {
                SapTableCellWrapper::Header(parser.subelement_from_def(def)?)
            }
            SapTableCellDefWrapper::Hierarchical(def) => {
                SapTableCellWrapper::Hierarchical(parser.subelement_from_def(def)?)
            }
            SapTableCellDefWrapper::Matrix(def) => {
                SapTableCellWrapper::Matrix(parser.subelement_from_def(def)?)
            }
            SapTableCellDefWrapper::Selection(def) => {
                SapTableCellWrapper::Selection(parser.subelement_from_def(def)?)
            }
        })
    }

    /// 셀의 id를 반환합니다.
    pub fn id(&self) -> &str {
        match self {
            SapTableCellWrapper::Normal(cell) => cell.id(),
            SapTableCellWrapper::Header(cell) => cell.id(),
            SapTableCellWrapper::Hierarchical(cell) => cell.id(),
            SapTableCellWrapper::Matrix(cell) => cell.id(),
            SapTableCellWrapper::Selection(cell) => cell.id(),
        }
    }
}

/// [`SapTable`] 셀에 대한 [`SubElementDefinition`] Wrapper
#[derive(Clone, Debug)]
pub enum SapTableCellDefWrapper {
    /// 일반 셀 정의
    Normal(SapTableNormalCellDef),
    /// 헤더 셀 정의
    Header(SapTableHeaderCellDef),
    /// 순차형 셀 정의
    Hierarchical(SapTableHierarchicalCellDef),
    /// Matrix 레이아웃용 셀 정의
    Matrix(SapTableMatrixCellDef),
    /// 선택을 위한 셀 정의
    Selection(SapTableSelectionCellDef),
}

impl SapTableCellDefWrapper {
    /// 셀을 표현하는 HTML 엘리먼트로부터 [`SapTableCellDefWrapper`]를 생성합니다.
    pub fn from_tag(
        table_def: SapTableDef,
        tag: &tl::HTMLTag<'_>,
    ) -> Option<SapTableCellDefWrapper> {
        let subct_id = tag
            .attributes()
            .get("subct")
            .flatten()
            .and_then(Bytes::try_as_utf8_str);
        let tag_id = tag.attributes().id().and_then(Bytes::try_as_utf8_str)?;
        match subct_id {
            Some(SapTableNormalCell::SUBCONTROL_ID) => Some(SapTableCellDefWrapper::Normal(
                SapTableNormalCellDef::new_dynamic(table_def, tag_id.to_owned()),
            )),
            Some(SapTableHeaderCell::SUBCONTROL_ID) => Some(SapTableCellDefWrapper::Header(
                SapTableHeaderCellDef::new_dynamic(table_def, tag_id.to_owned()),
            )),
            Some(SapTableHierarchicalCell::SUBCONTROL_ID) => {
                Some(SapTableCellDefWrapper::Hierarchical(
                    SapTableHierarchicalCellDef::new_dynamic(table_def, tag_id.to_owned()),
                ))
            }
            Some(SapTableMatrixCell::SUBCONTROL_ID) => Some(SapTableCellDefWrapper::Matrix(
                SapTableMatrixCellDef::new_dynamic(table_def, tag_id.to_owned()),
            )),
            Some(SapTableSelectionCell::SUBCONTROL_ID) => Some(SapTableCellDefWrapper::Selection(
                SapTableSelectionCellDef::new_dynamic(table_def, tag_id.to_owned()),
            )),
            _ => None,
        }
    }
}

/// [`SapTable`]의 공통된 셀 기능
pub trait SapTableCell<'a> {
    /// 셀 내부 컨텐츠 엘리먼트를 반환합니다.
    fn content(&self, parser: &'a ElementParser) -> Option<ElementDefWrapper<'a>>;
}

impl<'a> SapTableCell<'a> for SapTableCellWrapper<'a> {
    fn content(&self, parser: &'a ElementParser) -> Option<ElementDefWrapper<'a>> {
        match self {
            SapTableCellWrapper::Normal(elem) => elem.content(parser),
            SapTableCellWrapper::Header(elem) => elem.content(parser),
            SapTableCellWrapper::Hierarchical(elem) => elem.content(parser),
            SapTableCellWrapper::Matrix(elem) => elem.content(parser),
            SapTableCellWrapper::Selection(elem) => elem.content(parser),
        }
    }
}

mod header_cell;
mod hierarchical_cell;
mod matrix_cell;
mod normal_cell;
mod selection_cell;

pub use self::header_cell::{SapTableHeaderCell, SapTableHeaderCellDef, SapTableHeaderCellLSData};
pub use self::hierarchical_cell::{
    SapTableHierarchicalCell, SapTableHierarchicalCellDef, SapTableHierarchicalCellLSData,
};
pub use self::matrix_cell::{SapTableMatrixCell, SapTableMatrixCellDef, SapTableMatrixCellLSData};
pub use self::normal_cell::{SapTableNormalCell, SapTableNormalCellDef, SapTableNormalCellLSData};
pub use self::selection_cell::{
    SapTableSelectionCell, SapTableSelectionCellDef, SapTableSelectionCellLSData,
};

use super::{SapTable, SapTableDef};
