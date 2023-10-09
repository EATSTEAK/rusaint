use std::{iter, ops::Deref};

use scraper::ElementRef;

use crate::webdynpro::{element::ElementDef, error::ElementError};

use super::{
    property::SapTableRowType,
    row::SapTableRow,
    SapTable,
};

#[derive(custom_debug_derive::Debug)]
#[allow(unused)]
pub struct SapTableBody<'a> {
    table_def: ElementDef<'a, SapTable<'a>>,
    #[debug(skip)]
    elem_ref: ElementRef<'a>,
    header: SapTableRow<'a>,
    rows: Vec<SapTableRow<'a>>,
}

impl<'a> SapTableBody<'a> {
    pub(super) fn new(
        table_def: ElementDef<'a, SapTable<'a>>,
        elem_ref: ElementRef<'a>,
    ) -> Result<SapTableBody<'a>, ElementError> {
        let rows_iter = elem_ref
            .children()
            .filter_map(|node| scraper::ElementRef::wrap(node))
            .filter_map(|row_ref| SapTableRow::new(table_def.clone(), row_ref).ok());
        let mut header_iter = rows_iter
            .clone()
            .filter(|row| matches!(row.row_type(), SapTableRowType::Header));
        let Some(header) = header_iter.next() else {
            return Err(ElementError::NoSuchContent { element: table_def.id().to_owned(), content: "Header of table".to_owned() });
        };
        if header_iter.next().is_some() {
            return Err(ElementError::InvalidContent {
                element: table_def.id().to_owned(),
                content: "Multiple header in table".to_owned(),
            });
        }
        let rows = rows_iter.skip(1).collect::<Vec<SapTableRow<'a>>>();
        Ok(SapTableBody {
            table_def,
            elem_ref,
            header,
            rows,
        })
    }

    pub fn zip_header(&'a self) -> impl Iterator<Item = (&SapTableRow, &SapTableRow)> {
        let header_iter = iter::repeat(self.header());
        header_iter
            .zip(self.rows.iter())
    }

    pub fn with_header(&'a self) -> impl Iterator<Item = &SapTableRow> {
        [self.header()].into_iter().chain(self.rows.iter())
    }

    pub fn table_def(&self) -> ElementDef<'a, SapTable<'a>> {
        self.table_def.clone()
    }

    pub fn header(&self) -> &SapTableRow<'a> {
        &self.header
    }
}

impl<'a> Deref for SapTableBody<'a> {
    type Target = Vec<SapTableRow<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.rows
    }
}
