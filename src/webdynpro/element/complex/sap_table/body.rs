use std::ops::Index;

use scraper::ElementRef;

use crate::webdynpro::{
    client::body::Body,
    element::definition::ElementDefinition,
    error::{ElementError, WebDynproError},
};

use super::{
    property::SapTableRowType, row::SapTableRow, FromSapTable, SapTableDef, SapTableHeader,
};

/// [`SapTable`] 내부 테이블
#[derive(Clone, custom_debug_derive::Debug)]
#[allow(unused)]
pub struct SapTableBody<'a> {
    table_def: SapTableDef,
    #[debug(skip)]
    elem_ref: ElementRef<'a>,
    header: SapTableHeader<'a>,
    rows: Vec<SapTableRow<'a>>,
}

impl<'a> SapTableBody<'a> {
    pub(super) fn new(
        table_def: SapTableDef,
        elem_ref: ElementRef<'a>,
    ) -> Result<SapTableBody<'a>, ElementError> {
        let ref_iter = elem_ref
            .children()
            .filter_map(|node| scraper::ElementRef::wrap(node));
        let mut header_iter = ref_iter
            .clone()
            .filter_map(|row_ref| SapTableHeader::new(table_def.clone(), row_ref).ok());
        let Some(header) = header_iter.next() else {
            return Err(ElementError::NoSuchContent {
                element: table_def.id().to_owned(),
                content: "Header of table".to_owned(),
            });
        };
        if header_iter.next().is_some() {
            return Err(ElementError::InvalidContent {
                element: table_def.id().to_owned(),
                content: "Multiple header in table".to_owned(),
            });
        }
        let mut rows_iter = ref_iter
            .clone()
            .filter_map(|row_ref| SapTableRow::new(table_def.clone(), row_ref).ok())
            .filter(|row| !matches!(row.row_type(), SapTableRowType::Header));
        let rows = rows_iter.collect::<Vec<SapTableRow<'a>>>();
        Ok(SapTableBody {
            table_def,
            elem_ref,
            header,
            rows,
        })
    }

    /// 헤더 행을 제외한 행의 갯수를 반환합니다.
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// 내부 행의 Iterator를 반환합니다.
    pub fn iter(&'a self) -> impl Iterator<Item = &SapTableRow> + ExactSizeIterator {
        self.rows.iter()
    }

    /// 내부 행에 헤더 행을 포함한 튜플의 Iterator를 반환합니다.
    pub fn zip_header(
        &'a self,
    ) -> impl Iterator<Item = (&SapTableHeader, &SapTableRow)> + ExactSizeIterator {
        self.rows.iter().map(|row| (self.header(), row))
    }

    /// 이 테이블의 원본 [`SapTableDef`]를 반환합니다.
    pub fn table_def(&self) -> SapTableDef {
        self.table_def.clone()
    }

    /// 헤더 행을 반환합니다.
    pub fn header(&self) -> &SapTableHeader<'a> {
        &self.header
    }

    /// 테이블을 [`FromSapTable`]을 구현하는 형의 [`Vec`]으로 변환합니다.
    pub fn try_table_into<T: FromSapTable<'a>>(
        &'a self,
        body: &'a Body,
    ) -> Result<Vec<T>, WebDynproError> {
        self.iter()
            .map(|row| T::from_table(body, self.header(), row))
            .collect::<Result<Vec<T>, WebDynproError>>()
    }
}

impl<'a> Index<usize> for SapTableBody<'a> {
    type Output = SapTableRow<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}
